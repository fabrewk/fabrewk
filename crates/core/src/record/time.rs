//! The `at` field (REQ-REG-2): UTC in RFC 3339, informative only.

use std::time::{SystemTime, UNIX_EPOCH};

/// Now, as `YYYY-MM-DDTHH:MM:SSZ`.
pub fn now_utc() -> String {
    format_utc(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs()),
    )
}

fn format_utc(secs: u64) -> String {
    let (days, rem) = (secs / 86_400, secs % 86_400);
    // Days to a civil date (H. Hinnant's algorithm), for days on or after 1970-01-01.
    let z = days + 719_468;
    let (era, doe) = (z / 146_097, z % 146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + u64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3_600,
        rem % 3_600 / 60,
        rem % 60
    )
}

/// Whether `s` is a UTC timestamp in RFC 3339: `YYYY-MM-DDTHH:MM:SS`, an optional fraction, then `Z`.
pub fn is_utc_timestamp(s: &str) -> bool {
    let b = s.as_bytes();
    let num = |from: usize, to: usize| -> Option<u32> {
        let part = b.get(from..to)?;
        part.iter()
            .all(u8::is_ascii_digit)
            .then(|| part.iter().fold(0, |n, d| n * 10 + u32::from(d - b'0')))
    };
    let fixed = [(4, b'-'), (7, b'-'), (10, b'T'), (13, b':'), (16, b':')]
        .iter()
        .all(|&(i, c)| b.get(i) == Some(&c));
    let (Some(year), Some(month), Some(day), Some(hour), Some(min), Some(sec)) = (
        num(0, 4),
        num(5, 7),
        num(8, 10),
        num(11, 13),
        num(14, 16),
        num(17, 19),
    ) else {
        return false;
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let date_ok = (1..=12).contains(&month) && day >= 1 && day <= days[month as usize - 1];
    let tail = &b[19..];
    let tail_ok = match tail {
        [b'Z'] => true,
        [b'.', frac @ .., b'Z'] => {
            (1..=9).contains(&frac.len()) && frac.iter().all(u8::is_ascii_digit)
        }
        _ => false,
    };
    fixed && date_ok && hour < 24 && min < 60 && sec <= 60 && tail_ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn req_reg_2_at_is_rfc3339_utc() {
        assert!(is_utc_timestamp(&now_utc()));
        for good in [
            "2026-11-05T10:00:00Z",
            "2024-02-29T23:59:60Z",
            "2026-01-01T00:00:00.123456789Z",
        ] {
            assert!(is_utc_timestamp(good), "{good}");
        }
        for bad in [
            "2026-11-05T10:00:00",
            "2026-11-05 10:00:00Z",
            "2026-11-05T10:00:00+00:00",
            "2025-02-29T00:00:00Z",
            "2026-13-01T00:00:00Z",
            "2026-00-01T00:00:00Z",
            "2026-04-31T00:00:00Z",
            "2026-11-05T24:00:00Z",
            "2026-11-05T10:60:00Z",
            "2026-11-05T10:00:61Z",
            "2026-11-05T10:00:00.Z",
            "2026-11-05T10:00:00.1234567890Z",
            "2026-11-0xT10:00:00Z",
            "2026-11-05T10:00:00ZZ",
            "",
        ] {
            assert!(!is_utc_timestamp(bad), "{bad}");
        }
    }

    #[test]
    fn req_reg_2_at_civil_dates() {
        for (secs, text) in [
            (0, "1970-01-01T00:00:00Z"),
            (951_782_400, "2000-02-29T00:00:00Z"),
            (1_709_251_199, "2024-02-29T23:59:59Z"),
            (1_762_300_800, "2025-11-05T00:00:00Z"),
            (4_102_444_800, "2100-01-01T00:00:00Z"),
            (4_107_542_399, "2100-02-28T23:59:59Z"),
            (253_402_300_799, "9999-12-31T23:59:59Z"),
        ] {
            assert_eq!(format_utc(secs), text);
        }
    }
}
