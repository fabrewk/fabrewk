//! The exact contents of the files the plan creates or changes (point 5; REQ-DOM-4, REQ-DOM-6, REQ-INS-8): the project
//! file, the first maestro's file and fermento, the first alambique, the entry block. Every word comes from the
//! catalogue in the language given; the person's answers are written exactly as given.

use fabrewk_core::cli::say;

/// The lines that delimit the entry block (REQ-DOM-6).
pub const BEGIN: &str = "<!-- fabrewk:entry begin -->";
pub const END: &str = "<!-- fabrewk:entry end -->";

/// The domain's ignore file (REQ-DOM-4).
pub const GITIGNORE: &str = "local/\n";

/// A missing `CLAUDE.md` (REQ-DOM-6).
pub const CLAUDE: &str = "@AGENTS.md\n";

/// `s` as a TOML basic string.
pub fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out += "\\\"",
            '\\' => out += "\\\\",
            c if c.is_control() => out += &format!("\\u{:04X}", c as u32),
            c => out.push(c),
        }
    }
    out + "\""
}

/// A tool `--tool` names: its name, its adapter and its provider.
pub type Tool<'a> = (&'a str, &'a str, &'a str);

/// The project file (point 5): the project's name, the first maestro and their key when given, the canonical
/// documentation language, each tool with its provider allowed; no model and no profile.
pub fn project(name: &str, alias: Option<&str>, key: Option<&str>, lang: &str, tools: &[Tool]) -> String {
    let mut out = format!("[project]\nname = {}\n", quote(name));
    if let Some(alias) = alias {
        out += &format!("\n[maintainers.{alias}]\nrole = \"maestro\"\n");
        if let Some(key) = key {
            out += &format!("key = {}\n", quote(key));
        }
    }
    out += &format!("\n[languages]\ncanonicalDocumentationLanguage = {}\n", quote(lang));
    for (tool, adapter, provider) in tools {
        out += &format!("\n[tools.{tool}]\nadapter = {}\nproviders = [{}]\n", quote(adapter), quote(provider));
    }
    let mut providers: Vec<&str> = tools.iter().map(|t| t.2).collect();
    providers.sort_unstable();
    providers.dedup();
    for p in providers {
        out += &format!("\n[providers.{p}]\nallowed = true\n");
    }
    out
}

/// The first maestro's own file: their alias and the language agents speak to them in.
pub fn personal(alias: &str, lang: &str) -> String {
    format!("[maintainer]\nalias = {}\n\n[languages]\nagentInteraction = {}\n", quote(alias), quote(lang))
}

/// The first maestro's fermento: the catalogue's empty input and the separator block (REQ-FER-1).
pub fn fermento(alias: &str, lang: &str) -> String {
    say(lang, "install.fermento", &[("alias", alias)]) + &fabrewk_core::fermento::block(lang, alias)
}

/// The first alambique (REQ-INS-8): the title naming the project, the opening, and three numbered sections, each
/// holding the answer exactly as given, or the catalogue's placeholder line when it is empty.
pub fn alambique(name: &str, lang: &str, answers: [Option<&str>; 3]) -> String {
    let t = |k: &str| say(lang, &format!("skeleton.{k}"), &[]);
    let mut out = format!("# {name}\n\n{}\n", t("opening").trim_end());
    for (n, (section, answer)) in ["purpose", "audience", "excluded"].iter().zip(answers).enumerate() {
        let answer = answer.filter(|a| !a.is_empty()).map_or_else(|| t("empty"), str::to_owned);
        out += &format!("\n## {}. {}\n\n{answer}\n", n + 1, t(section));
    }
    out
}

/// The entry block in `lang` (REQ-DOM-6), its lines ended.
pub fn block(lang: &str) -> String {
    format!("{BEGIN}\n{}\n{END}\n", say(lang, "install.entry", &[]).trim_end())
}

/// `text` with `block`: the one present replaced in place, or the block added at the end after one blank line. Gives
/// the text after and the lines replaced, if any; `None` when the markers stand other than as one block.
pub fn with_block(text: &str, block: &str) -> Option<(String, Option<String>)> {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let at = |m: &str| -> Vec<usize> {
        lines.iter().enumerate().filter(|(_, l)| l.trim_end_matches(['\n', '\r']) == m).map(|(i, _)| i).collect()
    };
    let (begins, ends) = (at(BEGIN), at(END));
    match (&begins[..], &ends[..]) {
        ([], []) => {
            let gap = match () {
                _ if text.is_empty() || text.ends_with("\n\n") => "",
                _ if text.ends_with('\n') => "\n",
                _ => "\n\n",
            };
            Some((format!("{text}{gap}{block}"), None))
        }
        ([b], [e]) if b < e => {
            let old: String = lines[*b..=*e].concat();
            let after = lines[..*b].concat() + block + &lines[e + 1..].concat();
            Some((after, Some(old)))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_block_is_added_once_or_replaced_in_place() {
        let b = block("en");
        assert!(b.starts_with(&format!("{BEGIN}\n")) && b.ends_with(&format!("\n{END}\n")));
        assert_eq!(with_block("", &b), Some((b.clone(), None)));
        assert_eq!(with_block("a\n", &b).unwrap().0, format!("a\n\n{b}"));
        assert_eq!(with_block("a", &b).unwrap().0, format!("a\n\n{b}"));
        assert_eq!(with_block("a\n\n", &b).unwrap().0, format!("a\n\n{b}"));
        let old = format!("x\n{BEGIN}\nold\n{END}\ny\n");
        let (after, replaced) = with_block(&old, &b).unwrap();
        assert_eq!((after, replaced), (format!("x\n{b}y\n"), Some(format!("{BEGIN}\nold\n{END}\n"))));
        // Applying the same block again changes nothing.
        let once = with_block("a\n", &b).unwrap().0;
        assert_eq!(with_block(&once, &b).unwrap().0, once);
        for damaged in [format!("{BEGIN}\n"), format!("{END}\n{BEGIN}\n"), format!("{BEGIN}\n{BEGIN}\n{END}\n")] {
            assert_eq!(with_block(&damaged, &b), None, "{damaged}");
        }
    }

    #[test]
    fn values_are_quoted_as_toml_strings() {
        assert_eq!(quote("a\"b\\c\nd\u{7f}"), "\"a\\\"b\\\\c\\u000Ad\\u007F\"");
        let text = project(
            "my \"repo\"",
            Some("ana"),
            Some("ssh-ed25519 AAAA x\"y"),
            "es",
            &[("claude", "claude-code", "anthropic")],
        );
        let doc = toml::de::DeTable::parse(&text).unwrap();
        let get = |t: &str, k: &str| {
            let table = doc.get_ref().get(t).unwrap().get_ref().as_table().unwrap().clone();
            table.get(k).map(|v| v.get_ref().as_str().unwrap().to_owned())
        };
        assert_eq!(get("project", "name").as_deref(), Some("my \"repo\""));
        assert!(text.contains("[maintainers.ana]\nrole = \"maestro\"\nkey = \"ssh-ed25519 AAAA x\\\"y\"\n"), "{text}");
        assert!(text.ends_with("[tools.claude]\nadapter = \"claude-code\"\nproviders = [\"anthropic\"]\n\n[providers.anthropic]\nallowed = true\n"));
    }
}
