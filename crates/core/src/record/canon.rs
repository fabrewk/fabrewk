//! Canonical JSON (REQ-REG-3): UTF-8, object keys sorted by code point, no insignificant whitespace, integers only,
//! the shortest string escapes, one final newline. The form is defined here, not by a parser's options: a file is
//! canonical exactly when parsing it and encoding the value again gives the same bytes.

use serde_json::Value;
use sha2::{Digest, Sha256};

/// The canonical file of a value, final newline included; `None` if the value holds a number that is not an integer.
pub fn encode(value: &Value) -> Option<Vec<u8>> {
    let mut out = String::new();
    write(value, &mut out).then(|| {
        out.push('\n');
        out.into_bytes()
    })
}

/// The value of a canonical file; `None` for any file that is not exactly the canonical form of its value.
pub fn decode(bytes: &[u8]) -> Option<Value> {
    let value: Value = serde_json::from_slice(bytes).ok()?;
    (encode(&value)? == bytes).then_some(value)
}

/// The SHA-256 of `bytes`, as 64 lowercase hexadecimal digits.
pub fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    bytes
        .iter()
        .flat_map(|b| {
            [
                DIGITS[usize::from(b >> 4)] as char,
                DIGITS[usize::from(b & 15)] as char,
            ]
        })
        .collect()
}

fn write(value: &Value, out: &mut String) -> bool {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => match (n.as_i64(), n.as_u64()) {
            (Some(i), _) => out.push_str(&i.to_string()),
            (None, Some(u)) => out.push_str(&u.to_string()),
            (None, None) => return false,
        },
        Value::String(s) => string(s, out),
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                if !write(item, out) {
                    return false;
                }
            }
            out.push(']');
        }
        Value::Object(map) => {
            // Sorted here, whatever order the map keeps: `str` orders by bytes, for UTF-8 code-point order.
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            out.push('{');
            for (i, (key, item)) in entries.into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                string(key, out);
                out.push(':');
                if !write(item, out) {
                    return false;
                }
            }
            out.push('}');
        }
    }
    true
}

fn string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use serde_json::json;

    #[test]
    fn req_reg_3_form() {
        let v = json!({"b": [1, -2, true, null], "a": "x \u{2028}\u{85}\r\u{1}\u{1f}\u{7f}\"\\/é", "é": {}, "Z": 18446744073709551615u64});
        let bytes = encode(&v).unwrap();
        assert_eq!(
            String::from_utf8(bytes.clone()).unwrap(),
            "{\"Z\":18446744073709551615,\"a\":\"x \u{2028}\u{85}\\r\\u0001\\u001f\u{7f}\\\"\\\\/é\",\"b\":[1,-2,true,null],\"é\":{}}\n"
        );
        assert_eq!(decode(&bytes), Some(v));
    }

    #[test]
    fn refusal_record_noncanonical_forms() {
        for bad in [
            &b"{\"b\":1,\"a\":2}\n"[..],
            b"{\"a\": 1}\n",
            b"{\"a\":1.0}\n",
            b"{\"a\":1e2}\n",
            b"{\"a\":-0}\n",
            b"{\"a\":\"\\u0041\"}\n",
            b"{\"a\":\"\\/\"}\n",
            b"{\"a\":\"\\u000a\"}\n",
            b"{\"a\":1}",
            b"{\"a\":1}\n\n",
            b"{\"a\":1,\"a\":1}\n",
            b"{\"a\":18446744073709551616}\n",
            b"{\"a\":\"\xff\"}\n",
            b"{\"a\":\"\\ud800\"}\n",
            b"",
        ] {
            assert_eq!(decode(bad), None, "{}", String::from_utf8_lossy(bad));
        }
        assert_eq!(encode(&json!({"a": 0.5})), None);
    }

    fn any_value() -> impl Strategy<Value = Value> {
        let leaf = prop_oneof![
            Just(Value::Null),
            any::<bool>().prop_map(Value::Bool),
            any::<i64>().prop_map(Value::from),
            any::<u64>().prop_map(Value::from),
            any::<String>().prop_map(Value::String),
        ];
        leaf.prop_recursive(4, 64, 8, |inner| {
            prop_oneof![
                proptest::collection::vec(inner.clone(), 0..8).prop_map(Value::Array),
                proptest::collection::btree_map(any::<String>(), inner, 0..8)
                    .prop_map(|m| Value::Object(m.into_iter().collect())),
            ]
        })
    }

    proptest! {
        // REQ-REG-3: any value, its strings drawn from every Unicode scalar value, serializes canonically, parses
        // back equal and re-serializes byte-identical.
        #[test]
        fn req_reg_3_round_trip(v in any_value()) {
            let bytes = encode(&v).unwrap();
            let back = decode(&bytes).unwrap();
            prop_assert_eq!(&back, &v);
            prop_assert_eq!(encode(&back).unwrap(), bytes);
        }
    }
}
