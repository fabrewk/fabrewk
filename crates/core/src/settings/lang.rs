//! Language tags (REQ-SET-10, AD-3): BCP 47's well-formed shape, and the fallback from a regional variant to its base
//! and then to English.

/// The irregular tags RFC 5646 keeps from before its grammar (section 2.2.8); its regular ones fit the grammar.
const IRREGULAR: [&str; 17] = [
    "en-gb-oed",
    "i-ami",
    "i-bnn",
    "i-default",
    "i-enochian",
    "i-hak",
    "i-klingon",
    "i-lux",
    "i-mingo",
    "i-navajo",
    "i-pwn",
    "i-tao",
    "i-tay",
    "i-tsu",
    "sgn-be-fr",
    "sgn-be-nl",
    "sgn-ch-de",
];

/// Whether `tag` is a valid BCP 47 language tag as far as its form shows (RFC 5646): language, up to three extended
/// languages, script, region, distinct variants, extensions with distinct singletons and a private-use part, in that
/// order, case-insensitive; a private-use tag alone; or one of the irregular legacy tags. Whether a subtag is in
/// IANA's registry is not checked.
pub fn is_tag(tag: &str) -> bool {
    let lower = tag.to_ascii_lowercase();
    if IRREGULAR.contains(&lower.as_str()) {
        return true;
    }
    let subs: Vec<&str> = lower.split('-').collect();
    let n = subs.len();
    let alpha = |s: &str, lo, hi| (lo..=hi).contains(&s.len()) && s.bytes().all(|c| c.is_ascii_alphabetic());
    let alnum = |s: &str, lo, hi| (lo..=hi).contains(&s.len()) && s.bytes().all(|c| c.is_ascii_alphanumeric());
    let digits = |s: &str| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit());
    let mut seen: Vec<&str> = Vec::new();
    let mut i = 0;
    if subs[0] != "x" {
        if !(alpha(subs[0], 2, 3) || alpha(subs[0], 5, 8)) {
            return false;
        }
        i = 1;
        while i < n && i <= 3 && subs[0].len() <= 3 && alpha(subs[i], 3, 3) {
            i += 1;
        }
        if i < n && alpha(subs[i], 4, 4) {
            i += 1;
        }
        if i < n && (alpha(subs[i], 2, 2) || digits(subs[i])) {
            i += 1;
        }
        while i < n && (alnum(subs[i], 5, 8) || (alnum(subs[i], 4, 4) && subs[i].as_bytes()[0].is_ascii_digit())) {
            if seen.contains(&subs[i]) {
                return false;
            }
            seen.push(subs[i]);
            i += 1;
        }
    }
    while i < n && alnum(subs[i], 1, 1) {
        let private = subs[i] == "x";
        if seen.contains(&subs[i]) {
            return false;
        }
        seen.push(subs[i]);
        let (lo, start) = (if private { 1 } else { 2 }, i + 1);
        i = start;
        while i < n && alnum(subs[i], lo, 8) {
            i += 1;
        }
        if i == start || (private && i != n) {
            return false;
        }
    }
    i == n
}

/// The tags to try, in order, for `tag`: itself, each shorter prefix, then `en` (`es-MX`, `es`, `en`).
pub fn fallbacks(tag: &str) -> Vec<String> {
    let mut out = vec![tag.to_owned()];
    let mut t = tag;
    while let Some((base, _)) = t.rsplit_once('-') {
        out.push(base.to_owned());
        t = base;
    }
    if !out.iter().any(|t| t == "en") {
        out.push("en".to_owned());
    }
    out
}
