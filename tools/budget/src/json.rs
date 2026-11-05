//! A minimal JSON reader, enough for `cargo metadata`; the tool stays free of crates.

/// A JSON value. Numbers are kept as their text.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

impl Value {
    /// The member `key` of an object.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(members) => members.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> &[Value] {
        match self {
            Value::Array(items) => items,
            _ => &[],
        }
    }
}

/// Parses one JSON document.
pub fn parse(text: &str) -> Result<Value, String> {
    let mut p = Parser {
        s: text,
        b: text.as_bytes(),
        i: 0,
    };
    let value = p.value()?;
    p.space();
    if p.i != p.b.len() {
        return Err(format!("JSON: trailing input at byte {}", p.i));
    }
    Ok(value)
}

struct Parser<'a> {
    s: &'a str,
    b: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn space(&mut self) {
        while self.b.get(self.i).is_some_and(|c| c.is_ascii_whitespace()) {
            self.i += 1;
        }
    }

    fn fail<T>(&self, what: &str) -> Result<T, String> {
        Err(format!("JSON: {what} at byte {}", self.i))
    }

    fn eat(&mut self, c: u8) -> bool {
        self.space();
        let found = self.b.get(self.i) == Some(&c);
        self.i += usize::from(found);
        found
    }

    fn word(&mut self, word: &str, value: Value) -> Result<Value, String> {
        if self.b[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(value)
        } else {
            self.fail("unknown word")
        }
    }

    fn value(&mut self) -> Result<Value, String> {
        self.space();
        match self.b.get(self.i) {
            Some(b'{') => {
                self.i += 1;
                let mut members = Vec::new();
                if !self.eat(b'}') {
                    loop {
                        self.space();
                        let key = self.string()?;
                        if !self.eat(b':') {
                            return self.fail("expected ':'");
                        }
                        members.push((key, self.value()?));
                        if self.eat(b'}') {
                            break;
                        }
                        if !self.eat(b',') {
                            return self.fail("expected ',' or '}'");
                        }
                    }
                }
                Ok(Value::Object(members))
            }
            Some(b'[') => {
                self.i += 1;
                let mut items = Vec::new();
                if !self.eat(b']') {
                    loop {
                        items.push(self.value()?);
                        if self.eat(b']') {
                            break;
                        }
                        if !self.eat(b',') {
                            return self.fail("expected ',' or ']'");
                        }
                    }
                }
                Ok(Value::Array(items))
            }
            Some(b'"') => self.string().map(Value::String),
            Some(b't') => self.word("true", Value::Bool(true)),
            Some(b'f') => self.word("false", Value::Bool(false)),
            Some(b'n') => self.word("null", Value::Null),
            Some(c) if *c == b'-' || c.is_ascii_digit() => {
                let start = self.i;
                while self
                    .b
                    .get(self.i)
                    .is_some_and(|c| b"+-.eE".contains(c) || c.is_ascii_digit())
                {
                    self.i += 1;
                }
                Ok(Value::Number(
                    String::from_utf8_lossy(&self.b[start..self.i]).into_owned(),
                ))
            }
            _ => self.fail("expected a value"),
        }
    }

    fn string(&mut self) -> Result<String, String> {
        if self.b.get(self.i) != Some(&b'"') {
            return self.fail("expected a string");
        }
        self.i += 1;
        let mut units: Vec<u16> = Vec::new();
        let mut out = String::new();
        loop {
            let Some(&c) = self.b.get(self.i) else {
                return self.fail("unterminated string");
            };
            if c != b'\\' && !units.is_empty() {
                out.extend(char::decode_utf16(units.drain(..)).map(|r| r.unwrap_or('\u{FFFD}')));
            }
            match c {
                b'"' => {
                    self.i += 1;
                    return Ok(out);
                }
                b'\\' => {
                    let escape = self.b.get(self.i + 1).copied();
                    self.i += 2;
                    let plain = match escape {
                        Some(b'"') => '"',
                        Some(b'\\') => '\\',
                        Some(b'/') => '/',
                        Some(b'b') => '\u{8}',
                        Some(b'f') => '\u{c}',
                        Some(b'n') => '\n',
                        Some(b'r') => '\r',
                        Some(b't') => '\t',
                        Some(b'u') => {
                            let hex = self
                                .b
                                .get(self.i..self.i + 4)
                                .and_then(|h| std::str::from_utf8(h).ok());
                            let Some(unit) = hex.and_then(|h| u16::from_str_radix(h, 16).ok())
                            else {
                                return self.fail("bad \\u escape");
                            };
                            self.i += 4;
                            units.push(unit);
                            continue;
                        }
                        _ => return self.fail("bad escape"),
                    };
                    out.extend(
                        char::decode_utf16(units.drain(..)).map(|r| r.unwrap_or('\u{FFFD}')),
                    );
                    out.push(plain);
                }
                _ => {
                    // `i` only ever moves past ASCII bytes or whole characters, so it is on a boundary.
                    let ch = self.s[self.i..].chars().next().unwrap_or_default();
                    out.push(ch);
                    self.i += ch.len_utf8();
                }
            }
        }
    }
}
