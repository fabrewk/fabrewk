//! Counting one Rust source file (REQ-BUD-1).
//!
//! A line counts when it is not blank and some code token outside a `#[cfg(test)]` module touches it. Comments (`//`,
//! `///`, `//!`, `/* */` nested, `/** */`, `/*! */`) are not code; a string literal is code on every line it spans,
//! so `//` or `/*` inside a string is code. A `#[cfg(test)]` module is excluded from its first outer attribute to its
//! closing brace, unless it lies inside a macro invocation or a `macro_rules!` body, whose input may become any code.
//!
//! What would bring code the counter does not see is an error: a test module declared out of line
//! (`#[cfg(test)] mod tests;`; test modules are written inline, core-specification 11.2, rule 1), the identifier
//! `include` in any form (`include!`, `std::include`, an alias of it, `r#include`), a `path` attribute (`#[path = …]`,
//! also inside `cfg_attr`) and an attribute built by a macro (`#[$name …]`). What the compiler really reads is checked
//! against the count from Cargo's dep-info (`crate::sources`); the literal paths of `include_str!` and
//! `include_bytes!` are returned as data, so that check can tell data from code.

use std::fmt;

/// Why a file could not be counted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountError {
    /// A comment, string or character literal that does not end; `line` is where it starts (1-based).
    Unterminated { what: &'static str, line: usize },
    /// A bracket with no match, inside an attribute or a test module.
    Unbalanced { line: usize },
    /// `#[cfg(test)] mod name;`, whose file the counter would not see.
    OutOfLineTestModule { line: usize },
    /// `include!` or a `path` attribute, which bring in a file the counter would not see.
    HiddenSource { what: &'static str, line: usize },
}

impl fmt::Display for CountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CountError::Unterminated { what, line } => {
                write!(f, "line {line}: {what} does not end")
            }
            CountError::Unbalanced { line } => write!(f, "line {line}: bracket without its match"),
            CountError::HiddenSource { what, line } => {
                write!(
                    f,
                    "line {line}: {what} brings in code the counter does not see; not allowed in a ring"
                )
            }
            CountError::OutOfLineTestModule { line } => write!(
                f,
                "line {line}: #[cfg(test)] module declared out of line; write it inline so the counter sees it"
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Code,
    Comment,
}

#[derive(Debug, Clone, Copy)]
struct Token {
    kind: Kind,
    start: usize,
    end: usize,
}

/// The number of counted lines in `src`.
pub fn count_source(src: &str) -> Result<usize, CountError> {
    scan_source(src).map(|scan| scan.lines)
}

/// What one file holds for the budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scan {
    /// Counted lines.
    pub lines: usize,
    /// The literal paths given to `include_str!` and `include_bytes!`, relative to the file, as written.
    pub data: Vec<String>,
}

/// Counts `src` and collects the data files it includes.
pub fn scan_source(src: &str) -> Result<Scan, CountError> {
    let lines = LineIndex::new(src);
    let code: Vec<Token> = lex(src, &lines)?
        .into_iter()
        .filter(|t| t.kind == Kind::Code)
        .collect();
    hidden_sources(src, &code, &lines)?;
    let excluded = test_module_spans(src, &code, &lines)?;
    let mut touched = vec![false; lines.len()];
    let mut spans = excluded.iter().peekable();
    for (i, token) in code.iter().enumerate() {
        while spans.next_if(|&&(_, last)| last < i).is_some() {}
        if spans.peek().is_some_and(|&&(first, _)| first <= i) {
            continue;
        }
        let (first, last) = (lines.line_of(token.start), lines.line_of(token.end - 1));
        for line in &mut touched[first..=last] {
            *line = true;
        }
    }
    let counted = touched
        .iter()
        .enumerate()
        .filter(|&(n, &t)| t && !lines.text(src, n).trim().is_empty())
        .count();
    Ok(Scan {
        lines: counted,
        data: data_paths(src, &code),
    })
}

/// An identifier's name without the raw prefix `r#`.
fn name(token: &str) -> &str {
    token.strip_prefix("r#").unwrap_or(token)
}

/// The literal, escape-free paths of `include_str!("…")` and `include_bytes!("…")`, with any of the three delimiters.
fn data_paths(src: &str, code: &[Token]) -> Vec<String> {
    let text = |k: usize| code.get(k).map_or("", |t| &src[t.start..t.end]);
    (0..code.len())
        .filter(|&k| matches!(name(text(k)), "include_str" | "include_bytes"))
        .filter(|&k| text(k + 1) == "!" && matches!(text(k + 2), "(" | "[" | "{"))
        .filter_map(|k| string_value(text(k + 3)))
        .collect()
}

/// The value of a plain string literal without escapes, or of a raw string literal; `None` for anything else.
fn string_value(token: &str) -> Option<String> {
    if let Some(raw) = token.strip_prefix('r') {
        let hashes = raw.bytes().take_while(|&b| b == b'#').count();
        let body = raw[hashes..].strip_prefix('"')?;
        return Some(body[..body.len().checked_sub(hashes + 1)?].to_string());
    }
    let body = token.strip_prefix('"')?.strip_suffix('"')?;
    (!body.contains('\\')).then(|| body.to_string())
}

/// Byte offsets where each line starts.
struct LineIndex {
    starts: Vec<usize>,
}

impl LineIndex {
    fn new(src: &str) -> Self {
        let mut starts = vec![0];
        starts.extend(
            src.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'\n')
                .map(|(i, _)| i + 1),
        );
        LineIndex { starts }
    }

    fn len(&self) -> usize {
        self.starts.len()
    }

    /// The 0-based line holding byte `offset`.
    fn line_of(&self, offset: usize) -> usize {
        self.starts.partition_point(|&s| s <= offset) - 1
    }

    /// The line's text, with its newline if it has one.
    fn text<'a>(&self, src: &'a str, line: usize) -> &'a str {
        let end = self.starts.get(line + 1).copied().unwrap_or(src.len());
        &src[self.starts[line]..end]
    }
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b >= 0x80
}

/// Length in bytes of the UTF-8 character that starts with `lead`.
fn utf8_len(lead: u8) -> usize {
    match lead {
        0xF0.. => 4,
        0xE0.. => 3,
        0xC0.. => 2,
        _ => 1,
    }
}

fn lex(src: &str, lines: &LineIndex) -> Result<Vec<Token>, CountError> {
    let b = src.as_bytes();
    let at = |i: usize| b.get(i).copied();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let start = i;
        let c = b[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        let kind = if c == b'/' && at(i + 1) == Some(b'/') {
            i = b[i..]
                .iter()
                .position(|&x| x == b'\n')
                .map_or(b.len(), |p| i + p);
            Kind::Comment
        } else if c == b'/' && at(i + 1) == Some(b'*') {
            i = block_comment_end(b, i).ok_or(CountError::Unterminated {
                what: "block comment",
                line: lines.line_of(start) + 1,
            })?;
            Kind::Comment
        } else if let Some(end) = literal_end(b, i, lines)? {
            i = end;
            Kind::Code
        } else if is_ident_byte(c) {
            if c == b'r' && at(i + 1) == Some(b'#') && at(i + 2).is_some_and(is_ident_byte) {
                i += 2; // a raw identifier, `r#name`, is one token
            }
            while at(i).is_some_and(is_ident_byte) {
                i += 1;
            }
            Kind::Code
        } else {
            i += 1;
            Kind::Code
        };
        tokens.push(Token {
            kind,
            start,
            end: i,
        });
    }
    Ok(tokens)
}

/// End of the block comment starting at `i` (which holds `/*`), honouring nesting.
fn block_comment_end(b: &[u8], mut i: usize) -> Option<usize> {
    let mut depth = 0usize;
    while i + 1 < b.len() {
        match (b[i], b[i + 1]) {
            (b'/', b'*') => {
                depth += 1;
                i += 2;
            }
            (b'*', b'/') => {
                depth -= 1;
                i += 2;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => i += 1,
        }
    }
    None
}

/// If a string, byte string, C string, raw string, character, byte or lifetime starts at `i`, where it ends.
fn literal_end(b: &[u8], i: usize, lines: &LineIndex) -> Result<Option<usize>, CountError> {
    let at = |j: usize| b.get(j).copied();
    let unterminated = |what| CountError::Unterminated {
        what,
        line: lines.line_of(i) + 1,
    };
    let mut j = i;
    if matches!(at(j), Some(b'b' | b'c')) {
        j += 1;
    }
    if at(j) == Some(b'r') && matches!(at(j + 1), Some(b'"' | b'#')) {
        let hashes = b[j + 1..].iter().take_while(|&&x| x == b'#').count();
        if at(j + 1 + hashes) != Some(b'"') {
            return Ok(None); // a raw identifier such as `r#type`
        }
        let body = j + 2 + hashes;
        let closing: Vec<u8> = std::iter::once(b'"')
            .chain(std::iter::repeat_n(b'#', hashes))
            .collect();
        return b[body..]
            .windows(closing.len())
            .position(|w| w == closing.as_slice())
            .map(|p| Some(body + p + closing.len()))
            .ok_or(unterminated("raw string"));
    }
    match at(j) {
        Some(b'"') => {
            j += 1;
            while let Some(x) = at(j) {
                match x {
                    b'\\' => j += 2,
                    b'"' => return Ok(Some(j + 1)),
                    _ => j += 1,
                }
            }
            Err(unterminated("string"))
        }
        Some(b'\'') => quote_end(b, j)
            .map(Some)
            .ok_or(unterminated("character literal")),
        _ => Ok(None),
    }
}

/// End of a character literal or lifetime whose `'` is at `j`.
fn quote_end(b: &[u8], j: usize) -> Option<usize> {
    let at = |k: usize| b.get(k).copied();
    match at(j + 1)? {
        b'\\' => {
            let close = b
                .get(j + 3..)?
                .iter()
                .position(|&x| x == b'\'' || x == b'\n')?;
            (b[j + 3 + close] == b'\'').then_some(j + 4 + close)
        }
        lead => {
            let after = j + 1 + utf8_len(lead);
            if at(after) == Some(b'\'') {
                Some(after + 1)
            } else {
                let mut k = j + 1;
                while at(k).is_some_and(is_ident_byte) {
                    k += 1;
                }
                Some(k)
            }
        }
    }
}

/// Inclusive ranges of code-token indices covered by `#[cfg(test)]` modules, in order and not overlapping.
fn test_module_spans(
    src: &str,
    code: &[Token],
    lines: &LineIndex,
) -> Result<Vec<(usize, usize)>, CountError> {
    let text = |k: usize| code.get(k).map_or("", |t| &src[t.start..t.end]);
    let line = |k: usize| lines.line_of(code[k].start) + 1;
    let mut spans = Vec::new();
    let mut i = 0;
    while i < code.len() {
        if let Some(group) = macro_group(&text, i) {
            // Resume at the group's closing token, which the next pass steps over.
            i = group_close(code, src, group)
                .ok_or(CountError::Unbalanced { line: line(group) })?;
            continue;
        }
        if !(text(i) == "#" && text(i + 1) == "[") {
            i += 1;
            continue;
        }
        let first = i;
        let mut cfg_test = false;
        while text(i) == "#" && text(i + 1) == "[" {
            let close = matching(code, src, i + 1, "[", "]")
                .ok_or(CountError::Unbalanced { line: line(i) })?;
            cfg_test |= (i + 2..close).map(text).eq(["cfg", "(", "test", ")"]);
            i = close + 1;
        }
        if !cfg_test {
            continue;
        }
        let mut k = i;
        if text(k) == "pub" {
            k += 1;
            if text(k) == "(" {
                k = matching(code, src, k, "(", ")")
                    .ok_or(CountError::Unbalanced { line: line(k) })?
                    + 1;
            }
        }
        if text(k) == "mod" {
            match text(k + 2) {
                "{" => {
                    let close = matching(code, src, k + 2, "{", "}")
                        .ok_or(CountError::Unbalanced { line: line(k + 2) })?;
                    spans.push((first, close));
                    i = close; // the next pass steps over the closing brace
                }
                ";" => return Err(CountError::OutOfLineTestModule { line: line(first) }),
                _ => {}
            }
        }
    }
    Ok(spans)
}

/// Where the delimited group of a macro invocation (`name!(…)`, `name![…]`, `name!{…}`) or of a `macro_rules!`
/// definition starting at `i` opens.
fn macro_group<'a>(text: &impl Fn(usize) -> &'a str, i: usize) -> Option<usize> {
    let ident = |k: usize| {
        text(k)
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_alphabetic() || b == b'_' || b >= 0x80)
    };
    if !(ident(i) && text(i + 1) == "!") {
        return None;
    }
    let group = if name(text(i)) == "macro_rules" && ident(i + 2) {
        i + 3
    } else {
        i + 2
    };
    matches!(text(group), "(" | "[" | "{").then_some(group)
}

/// Index of the token closing the group that opens at `start`, whichever its delimiter.
fn group_close(code: &[Token], src: &str, start: usize) -> Option<usize> {
    let open = &src[code.get(start)?.start..code[start].end];
    let close = match open {
        "(" => ")",
        "[" => "]",
        "{" => "}",
        _ => return None,
    };
    matching(code, src, start, open, close)
}

/// Refuses the identifier `include`, any attribute holding `path = …` and any attribute holding a metavariable,
/// anywhere in the file.
fn hidden_sources(src: &str, code: &[Token], lines: &LineIndex) -> Result<(), CountError> {
    let text = |k: usize| code.get(k).map_or("", |t| &src[t.start..t.end]);
    let line = |k: usize| lines.line_of(code[k].start) + 1;
    for k in 0..code.len() {
        if name(text(k)) == "include" {
            return Err(CountError::HiddenSource {
                what: "include",
                line: line(k),
            });
        }
        if text(k) != "#" {
            continue;
        }
        let open = if text(k + 1) == "!" { k + 2 } else { k + 1 };
        if text(open) != "[" {
            continue;
        }
        let close =
            matching(code, src, open, "[", "]").ok_or(CountError::Unbalanced { line: line(k) })?;
        if (open..close).any(|m| name(text(m)) == "path" && text(m + 1) == "=") {
            return Err(CountError::HiddenSource {
                what: "a path attribute",
                line: line(k),
            });
        }
        if (open..close).any(|m| text(m) == "$") {
            return Err(CountError::HiddenSource {
                what: "an attribute built by a macro",
                line: line(k),
            });
        }
    }
    Ok(())
}

/// Index of the token closing the `open` at `start`; `None` when `start` is not `open` or nothing closes it.
fn matching(code: &[Token], src: &str, start: usize, open: &str, close: &str) -> Option<usize> {
    if code.get(start).is_none_or(|t| &src[t.start..t.end] != open) {
        return None;
    }
    let mut depth = 0usize;
    for (k, t) in code.iter().enumerate().skip(start) {
        let s = &src[t.start..t.end];
        if s == open {
            depth += 1;
        } else if s == close {
            depth -= 1;
            if depth == 0 {
                return Some(k);
            }
        }
    }
    None
}
