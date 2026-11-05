//! Language tags (REQ-SET-10, AD-3): BCP 47's well-formed shape, and the fallback from a regional variant to its base
//! and then to English.

/// Whether `tag` is a well-formed BCP 47 language tag: language, up to three extended languages, script, region,
/// variants, extensions and a private-use part, in that order (irregular legacy tags are not accepted).
pub fn is_tag(tag: &str) -> bool {
    let subs: Vec<&str> = tag.split('-').collect();
    let n = subs.len();
    let alpha = |s: &str, lo, hi| {
        (lo..=hi).contains(&s.len()) && s.bytes().all(|c| c.is_ascii_alphabetic())
    };
    let alnum = |s: &str, lo, hi| {
        (lo..=hi).contains(&s.len()) && s.bytes().all(|c| c.is_ascii_alphanumeric())
    };
    let digits = |s: &str| s.len() == 3 && s.bytes().all(|c| c.is_ascii_digit());
    if !(alpha(subs[0], 2, 3) || alpha(subs[0], 5, 8)) {
        return false;
    }
    let mut i = 1;
    while i < n && i <= 3 && subs[0].len() <= 3 && alpha(subs[i], 3, 3) {
        i += 1;
    }
    if i < n && alpha(subs[i], 4, 4) {
        i += 1;
    }
    if i < n && (alpha(subs[i], 2, 2) || digits(subs[i])) {
        i += 1;
    }
    while i < n
        && (alnum(subs[i], 5, 8)
            || (alnum(subs[i], 4, 4) && subs[i].as_bytes()[0].is_ascii_digit()))
    {
        i += 1;
    }
    while i < n && alnum(subs[i], 1, 1) {
        let private = subs[i].eq_ignore_ascii_case("x");
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
