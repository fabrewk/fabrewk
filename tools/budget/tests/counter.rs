//! The counter's fixture tests (REQ-BUD-1; core-specification 11.6).

use budget::budget::{Ring, RingKind};
use budget::count::{CountError, count_source, scan_source};
use budget::rings::{Error, count_rings};
use std::fs;
use std::path::{Path, PathBuf};

fn count(src: &str) -> usize {
    count_source(src).unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn req_bud_1_blank_lines_are_not_counted() {
    assert_eq!(count(""), 0);
    assert_eq!(count("\n\n   \n\t\n"), 0);
    assert_eq!(count("fn a() {}\n\n\nfn b() {}\n"), 2);
    assert_eq!(count("fn a() {}\r\n\r\n"), 1);
    assert_eq!(count("fn a() {}"), 1);
}

#[test]
fn req_bud_1_line_comments_are_not_counted() {
    assert_eq!(count("// a\n/// b\n//! c\n//// d\nfn a() {}\n"), 1);
    assert_eq!(count("fn a() {} // trailing\n"), 1);
    assert_eq!(count("    // indented\n"), 0);
}

#[test]
fn req_bud_1_block_comments_nested_are_not_counted() {
    let src = "/* one\n   /* nested\n   still */\n   still outer\n*/\nfn a() {}\n";
    assert_eq!(count(src), 1);
    assert_eq!(count("/** doc\n */\n/*! inner\n */\n"), 0);
    assert_eq!(count("/**/\n/***/\n"), 0);
    assert_eq!(count("let x = 1; /* c */\n"), 1);
    assert_eq!(count("/* c */ let x = 1;\n"), 1);
    assert_eq!(count("/* c\n */ let x = 1;\n"), 1);
    assert_eq!(count("let x = /* c\n */ 1;\n"), 2);
}

#[test]
fn req_bud_1_comment_markers_inside_strings_are_code() {
    assert_eq!(count("let s = \"// not a comment\";\n"), 1);
    assert_eq!(count("let s = \"/* not a comment\";\nlet t = 1;\n"), 2);
    assert_eq!(count("let s = \"a\n// inside the string\nb\";\n"), 3);
    assert_eq!(count("let s = \"esc \\\" // still string\";\n"), 1);
    assert_eq!(count("let s = \"\\\\\"; // comment after a string ending in a backslash\n"), 1);
}

#[test]
fn req_bud_1_raw_byte_and_c_strings() {
    assert_eq!(count("let s = r\"// x\";\n"), 1);
    assert_eq!(count("let s = r#\"a \" // b\n/* c\"#;\n"), 2);
    assert_eq!(count("let s = r##\"a \"# b\"##; // c\n"), 1);
    assert_eq!(count("let s = b\"/*\";\nlet t = br\"/*\";\nlet u = c\"/*\";\nlet v = cr#\"/*\"#;\n"), 4);
    assert_eq!(count("let r#type = 1; // raw identifier\n"), 1);
}

#[test]
fn req_bud_1_blank_lines_inside_a_string_are_not_counted() {
    assert_eq!(count("let s = \"a\n\n   \nb\";\n"), 2);
}

#[test]
fn req_bud_1_characters_and_lifetimes() {
    assert_eq!(count("let q = '\"'; // a quote char\nlet c = 1;\n"), 2);
    assert_eq!(count("let q = '\\''; /* c */\n"), 1);
    assert_eq!(count("let q = b'\\\\'; /* c */\n"), 1);
    assert_eq!(count("let e = 'é'; // c\n"), 1);
    assert_eq!(count("let u = '\\u{1F600}'; // c\n"), 1);
    assert_eq!(count("fn f<'a>(x: &'a str) -> &'a str { x } // \"\n"), 1);
    assert_eq!(count("'outer: loop { break 'outer; }\n// c\n"), 1);
}

#[test]
fn req_bud_1_multibyte_characters_in_literals() {
    // Read wrongly, each `'x'` would leave `','` as a character and open a string at `"` running to the next line.
    for c in ['é', '€', '😀'] {
        assert_eq!(count(&format!("let a = ['{c}','\"'];\n// comment \"\n")), 1, "{c}");
    }
}

#[test]
fn req_bud_1_cfg_test_modules_are_not_counted() {
    let src = "fn a() {}\n#[cfg(test)]\nmod tests {\n    fn t() { let s = \"}\"; let c = '}'; }\n    mod inner { fn u() {} }\n}\nfn b() {}\n";
    assert_eq!(count(src), 2);
    let src = "#[cfg(test)] mod tests { fn t() {} }\nfn b() {}\n";
    assert_eq!(count(src), 1);
    let src = "#[allow(dead_code)]\n#[cfg(test)]\n#[path_is_not_used]\npub(crate) mod tests {\n fn t() {}\n}\n";
    assert_eq!(count(src), 0);
    let src = "# [ cfg ( test ) ]\npub mod tests {\n fn t() {}\n}\n";
    assert_eq!(count(src), 0);
    let src = "#[cfg(test)]\nmod tests {\n fn t() {}\n} fn after() {}\n";
    assert_eq!(count(src), 1);
    let src = "#[cfg(test)]\nmod unit_tests_2 {\n fn t() {}\n}\n";
    assert_eq!(count(src), 0);
}

#[test]
fn req_bud_1_other_cfg_items_are_counted() {
    assert_eq!(count("#[cfg(test)]\nfn only_in_tests() {}\n"), 2);
    assert_eq!(count("#[cfg(not(test))]\nmod real {\n fn r() {}\n}\n"), 4);
    assert_eq!(count("#[cfg(all(test, unix))]\nmod t {\n}\n"), 3);
    assert_eq!(count("#[cfg(test)]\nuse std::fmt;\n"), 2);
    assert_eq!(count("#![cfg_attr(test, allow(unused))]\nfn a() {}\n"), 2);
}

#[test]
fn req_bud_1_out_of_line_test_module_is_an_error() {
    assert_eq!(count_source("#[cfg(test)]\nmod tests;\n"), Err(CountError::OutOfLineTestModule { line: 1 }));
}

#[test]
fn req_bud_1_malformed_sources_are_errors() {
    assert_eq!(count_source("fn a() {}\n/* open"), Err(CountError::Unterminated { what: "block comment", line: 2 }));
    assert_eq!(count_source("let s = \"open"), Err(CountError::Unterminated { what: "string", line: 1 }));
    assert_eq!(count_source("let s = r#\"open\"\n"), Err(CountError::Unterminated { what: "raw string", line: 1 }));
    assert!(count_source("let c = '\\").is_err());
    let messages = [
        CountError::Unterminated { what: "string", line: 4 },
        CountError::Unbalanced { line: 5 },
        CountError::OutOfLineTestModule { line: 6 },
    ]
    .map(|e| e.to_string());
    assert_eq!(messages[0], "line 4: string does not end");
    assert_eq!(messages[1], "line 5: bracket without its match");
    assert!(messages[2].starts_with("line 6: #[cfg(test)] module declared out of line"));
    assert_eq!(count_source("#[cfg(test)]\nmod t {\n"), Err(CountError::Unbalanced { line: 2 }));
    assert_eq!(count_source("#[cfg(test\n"), Err(CountError::Unbalanced { line: 1 }));
}

/// A fixture workspace under Cargo's temporary directory.
struct Fixture(PathBuf);

impl Fixture {
    fn new(name: &str) -> Self {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("counter").join(name);
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Fixture(dir)
    }

    fn file(&self, path: &str, content: &str) -> &Self {
        let p = self.0.join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, content).unwrap();
        self
    }

    fn package(&self, dir: &str) -> &Self {
        self.file(&format!("{dir}/Cargo.toml"), "[package]\nname = \"x\"\n")
    }
}

const CORE: Ring = Ring { name: "core", kind: RingKind::Core, dirs: &["crates/core"] };
const TWO_LINES: &str = "fn a() {}\nfn b() {}\n";

#[test]
fn req_bud_1_tests_benches_and_examples_are_not_counted() {
    let f = Fixture::new("uncounted");
    f.package("crates/core")
        .file("crates/core/src/lib.rs", TWO_LINES)
        .file("crates/core/src/deep/tests/mod.rs", TWO_LINES)
        .file("crates/core/tests/t.rs", TWO_LINES)
        .file("crates/core/benches/b.rs", TWO_LINES)
        .file("crates/core/examples/e.rs", TWO_LINES)
        .file("crates/core/build.rs", "fn main() {}\n")
        .file("crates/core/README.md", "fn not_rust() {}\n");
    let counts = count_rings(&f.0, &[CORE]).unwrap();
    assert_eq!(counts.len(), 1);
    assert_eq!((counts[0].name, counts[0].files, counts[0].lines), ("core", 3, 5));
}

#[test]
fn req_bud_1_counts_per_ring() {
    let f = Fixture::new("per-ring");
    f.package("crates/core")
        .file("crates/core/src/lib.rs", TWO_LINES)
        .package("crates/bin")
        .file("crates/bin/src/main.rs", "fn main() {}\n")
        .package("crates/adapter")
        .file("crates/adapter/src/lib.rs", TWO_LINES)
        .file("tools/budget/src/main.rs", TWO_LINES);
    let rings = [
        Ring { name: "core", kind: RingKind::Core, dirs: &["crates/core", "crates/bin"] },
        Ring { name: "adapter", kind: RingKind::ToolAdapter, dirs: &["crates/adapter"] },
    ];
    let counts = count_rings(&f.0, &rings).unwrap();
    let measures: Vec<_> = counts.iter().map(|c| (c.name, c.lines, c.files, c.limit())).collect();
    assert_eq!(measures, [("core", 3, 2, 13_100), ("adapter", 2, 1, 600)]);
}

#[test]
fn req_bud_1_code_in_no_ring_is_an_error() {
    let f = Fixture::new("unassigned-crate");
    f.package("crates/core").file("crates/core/src/lib.rs", TWO_LINES).package("crates/stray");
    assert!(matches!(count_rings(&f.0, &[CORE]), Err(Error::Unassigned(p)) if p.ends_with("crates/stray")));

    let f = Fixture::new("unassigned-file");
    f.package("crates/core").file("crates/core/src/lib.rs", TWO_LINES).file("src/main.rs", TWO_LINES);
    assert!(matches!(count_rings(&f.0, &[CORE]), Err(Error::Unassigned(p)) if p.ends_with("src/main.rs")));

    let f = Fixture::new("unassigned-root-file");
    f.package("crates/core").file("crates/core/src/lib.rs", TWO_LINES).file("build.rs", TWO_LINES);
    assert!(matches!(count_rings(&f.0, &[CORE]), Err(Error::Unassigned(p)) if p.ends_with("build.rs")));
}

#[test]
fn req_bud_1_overlapping_or_missing_rings_are_errors() {
    let f = Fixture::new("overlap");
    f.package("crates/core").file("crates/core/src/lib.rs", TWO_LINES);
    let rings = [CORE, Ring { name: "inner", kind: RingKind::Installer, dirs: &["crates/core/src"] }];
    assert!(matches!(count_rings(&f.0, &rings), Err(Error::Overlap(..))));
    let rings = [Ring { name: "gone", kind: RingKind::Installer, dirs: &["crates/gone"] }];
    assert!(matches!(count_rings(&f.0, &rings), Err(Error::MissingDir(_))));
}

#[test]
fn req_bud_1_symbolic_links_are_errors() {
    let f = Fixture::new("symlink");
    f.package("crates/core").file("crates/core/src/lib.rs", TWO_LINES).file("elsewhere/x.rs", TWO_LINES);
    std::os::unix::fs::symlink(f.0.join("elsewhere"), f.0.join("crates/core/src/linked")).unwrap();
    assert!(matches!(count_rings(&f.0, &[CORE]), Err(Error::Symlink(_))));
}

#[test]
fn req_bud_1_unreadable_sources_are_errors() {
    let f = Fixture::new("not-utf8");
    f.package("crates/core");
    fs::create_dir_all(f.0.join("crates/core/src")).unwrap();
    fs::write(f.0.join("crates/core/src/lib.rs"), [0xff, 0xfe, b'\n']).unwrap();
    assert!(matches!(count_rings(&f.0, &[CORE]), Err(Error::NotUtf8(_))));

    let f = Fixture::new("malformed");
    f.package("crates/core").file("crates/core/src/lib.rs", "/* open\n");
    assert!(matches!(count_rings(&f.0, &[CORE]), Err(Error::Count(..))));
}

#[test]
fn req_bud_1_slashes_and_stars_that_open_no_comment() {
    assert_eq!(count("let x = a / b * c;\nlet y = (*p);\nlet z = &*q / 2;\n"), 3);
}

#[test]
fn req_bud_1_escapes_and_raw_bodies() {
    // A backslash escapes the quote in a plain string, so the string runs to the next line.
    assert_eq!(count("let s = \"a\\\"\n// b\";\n"), 2);
    // In a raw string a backslash is plain, so the string ends on its own line.
    assert_eq!(count("let s = br\"\\\";\n// \"\n"), 1);
    assert_eq!(count("let s = cr\"\\\";\n// \"\n"), 1);
    // The body of a raw string may start with the hashes of its own terminator.
    assert_eq!(count("let s = r#\"# // x\n\"#;\n"), 2);
}

#[test]
fn req_bud_1_closing_quote_of_a_character_is_consumed() {
    assert_eq!(count("let a = ['x','\"'];\n// comment \"\n"), 1);
}

#[test]
fn req_bud_1_attribute_followed_by_an_array() {
    assert_eq!(count("fn f() {\n    #[allow(unused)]\n    [1, 2];\n}\n"), 4);
}

#[test]
fn req_bud_1_cfg_test_on_an_impl_is_counted() {
    assert_eq!(count("#[cfg(test)]\nimpl Foo {\n    fn t() {}\n}\n"), 4);
}

#[test]
fn req_bud_1_a_manifest_without_package_is_no_package() {
    let f = Fixture::new("virtual-manifest");
    f.package("crates/core")
        .file("crates/core/src/lib.rs", TWO_LINES)
        .file("crates/core/sub/Cargo.toml", "[workspace]\n")
        .file("crates/core/sub/tests/t.rs", TWO_LINES);
    let counts = count_rings(&f.0, &[CORE]).unwrap();
    assert_eq!((counts[0].files, counts[0].lines), (2, 4));
}

#[test]
fn req_bud_1_ring_errors_name_their_path() {
    let p = PathBuf::from("/w/x.rs");
    let messages = [
        Error::NotUtf8(p.clone()),
        Error::Symlink(p.clone()),
        Error::MissingDir(p.clone()),
        Error::Unassigned(p.clone()),
        Error::Count(p.clone(), CountError::Unbalanced { line: 3 }),
        Error::Io(p, std::io::Error::other("denied")),
        Error::Overlap("crates/a", "crates/a/b"),
    ]
    .map(|e| e.to_string());
    assert_eq!(
        messages,
        [
            "/w/x.rs: not UTF-8",
            "/w/x.rs: symbolic link inside the program's code",
            "/w/x.rs: a ring's directory that does not exist",
            "/w/x.rs: Rust code in no ring",
            "/w/x.rs: line 3: bracket without its match",
            "/w/x.rs: denied",
            "ring directories crates/a and crates/a/b overlap",
        ]
    );
}

#[test]
fn req_bud_1_a_closing_quote_never_starts_another_literal() {
    // Read wrongly, the closing quote of 'a' would pair with `"` and the string would not open.
    assert_eq!(count("let s = 'a'\"'\n// x\";\n"), 2);
}

#[test]
fn req_bud_1_hash_after_an_attribute_in_a_macro() {
    assert_eq!(count("quote! {\n    #[derive(Debug)] #name\n}\n"), 3);
}

#[test]
fn req_bud_1_cfg_test_inside_a_macro_is_counted() {
    // A macro may turn its input into production code: Codex's case in its first review of part 7a.
    let src = "macro_rules! production {\n    (#[cfg(test)] mod $n:ident { $($body:tt)* }) => { mod $n { $($body)* } };\n}\n\
               production! {\n    #[cfg(test)]\n    mod live {\n        pub fn answer() -> u32 {\n            42\n        }\n    }\n}\n\
               fn main() {\n    println!(\"{}\", live::answer());\n}\n";
    assert_eq!(count(src), 14);
    assert_eq!(count("m![#[cfg(test)] mod t { fn x() {} }];\n"), 1);
    assert_eq!(count("m!(\n#[cfg(test)]\nmod t {\n}\n);\n"), 5);
    // Outside any macro the module is still excluded, and `!` as negation opens no macro.
    assert_eq!(count("if !ready {\n    go();\n}\n#[cfg(test)]\nmod t {\n    fn x() {}\n}\n"), 3);
    assert_eq!(count("m! { x }\n#[cfg(test)]\nmod t {\n    fn x() {}\n}\n"), 1);
}

#[test]
fn req_bud_1_include_and_path_attributes_are_errors() {
    let hidden = |src: &str| match count_source(src) {
        Err(CountError::HiddenSource { what, line }) => (what, line),
        other => panic!("{src:?}: {other:?}"),
    };
    assert_eq!(hidden("fn a() {}\ninclude!(\"live.inc\");\n"), ("include", 2));
    assert_eq!(hidden("m! { include ! (\"x.rs\") }\n"), ("include", 1));
    // An alias or a raw name of `include` is the same macro: Codex's case in its second review of part 7a.
    assert_eq!(hidden("use std::include as load;\nload!(\"hidden.inc\");\n"), ("include", 1));
    assert_eq!(hidden("r#include!(\"x.inc\");\n"), ("include", 1));
    assert_eq!(hidden("#[path = \"../../tools/live.rs\"]\nmod live;\n"), ("a path attribute", 1));
    assert_eq!(hidden("#[cfg_attr(unix, path = \"x.rs\")]\nmod x;\n"), ("a path attribute", 1));
    assert_eq!(hidden("fn a() {}\n#![path = \"x\"]\n"), ("a path attribute", 2));
    assert_eq!(hidden("#[r#path = \"x\"]\nmod x;\n"), ("a path attribute", 1));
    // An attribute a macro builds could be a path attribute.
    let generated = "macro_rules! load {\n    ($attr:ident) => {\n        #[$attr = \"hidden.inc\"]\n        mod hidden;\n    };\n}\nload!(path);\n";
    assert_eq!(hidden(generated), ("an attribute built by a macro", 3));
    assert_eq!(hidden("m! { $(#[$meta])* }\n"), ("an attribute built by a macro", 1));
    assert_eq!(
        CountError::HiddenSource { what: "include", line: 2 }.to_string(),
        "line 2: include brings in code the counter does not see; not allowed in a ring"
    );
    // Data, not code, may still be included; a path that is a string or a variable is no attribute.
    assert_eq!(count("const T: &str = include_str!(\"t.txt\");\nlet path = 1;\n#[doc = \"path = x\"]\nfn a() {}\n"), 4);
}

#[test]
fn req_bud_1_data_files_are_collected() {
    let scan = scan_source("const A: &str = include_str!(\"a.toml\");\nconst B: &[u8] = include_bytes!(\"img/b.png\");\n\
                            const C: &str = r#include_str!(\"c.txt\");\nconst D: &str = include_str!(concat!(\"d\", \".txt\"));\n\
                            const E: &str = include_str!(\"e\\\\.txt\");\n// include_str!(\"f.txt\")\n")
        .unwrap();
    assert_eq!(scan.lines, 5);
    assert_eq!(scan.data, ["a.toml", "img/b.png", "c.txt"]);
}

#[test]
fn req_bud_1_raw_identifiers_are_one_token() {
    // Codex's case in its second review of part 7a: a raw macro name hid the pattern of `macro_rules!`.
    let src = "macro_rules! r#production {\n    (\n        #[cfg(test)]\n        mod live {\n            fn answer() -> u32 { 42 }\n        }\n    ) => { fn answer() -> u32 { 42 } };\n}\n\
               production! { #[cfg(test)] mod live { fn answer() -> u32 { 42 } } }\nfn main() { println!(\"{}\", answer()); }\n";
    assert_eq!(count(src), 10);
    assert_eq!(count("r#m! {\n#[cfg(test)]\nmod t {\n}\n}\n"), 5);
    assert_eq!(count("#[cfg(test)]\nmod r#type {\n}\n"), 0);
}

#[test]
fn req_bud_1_what_opens_a_macro_and_what_does_not() {
    // A test module inside these is counted: each is a macro's input.
    for name in ["_m", "ñ", "m"] {
        assert_eq!(count(&format!("{name}! {{\n#[cfg(test)]\nmod t {{\n}}\n}}\n")), 5, "{name}");
    }
    let rules = "macro_rules! m {\n    () => {\n        #[cfg(test)]\n        mod t {\n        }\n    };\n}\n";
    assert_eq!(count(rules), 7);
    assert_eq!(count(&format!("fn a() {{}}\n{rules}")), 8);
    // Not macros: negation before a block or a condition, and a plain module; their test modules are excluded.
    let not_block = "let x = (!{\n    #[cfg(test)]\n    mod t {\n    }\n    true\n});\n";
    assert_eq!(count(not_block), 3);
    assert_eq!(count("if !ready {\n    #[cfg(test)]\n    mod t {\n    }\n}\n"), 2);
    let outer = "mod outer {\n    #[cfg(test)]\n    mod t {\n        fn x() {}\n    }\n}\n";
    assert_eq!(count(outer), 2);
}

#[test]
fn req_bud_1_an_attribute_run_ends_at_the_first_non_attribute() {
    assert_eq!(count("#[derive(Debug)] #name\n"), 1);
    assert_eq!(count("#[inline]\nv[cfg(test)]\nmod t {\n}\n"), 4);
}

#[test]
fn req_bud_1_data_files_with_any_delimiter_and_no_function() {
    let scan = scan_source(
        "const A: &str = include_str![\"a.txt\"];\nconst B: &[u8] = include_bytes! {\"b.bin\"};\n\
                            fn f() { include_str(\"c.txt\"); }\nlet include_str = [\"d.txt\"];\n",
    )
    .unwrap();
    assert_eq!(scan.data, ["a.txt", "b.bin"]);
}

#[test]
fn req_bud_1_a_lone_r_is_an_ordinary_identifier() {
    assert_eq!(count("m!{r}x\n"), 1);
    assert_eq!(count("#[cfg(test)]\nmod t {\n    fn f() -> u8 { let r = 1; r}\n}\nfn g() {}\n"), 1);
}

#[test]
fn req_bud_1_raw_string_data_paths() {
    // Codex's case in its third review of part 7a; an escaped path or an alias of the macro stays refused as data.
    let scan = scan_source(
        "const A: &str = include_str!(r#\"a.txt\"#);\nconst B: &str = include_str!(r\"b.txt\");\n\
         const C: &str = include_str!(r##\"c\"#.txt\"##);\nconst D: &str = include_str!(\"\\x64.txt\");\n",
    )
    .unwrap();
    assert_eq!(scan.data, ["a.txt", "b.txt", "c\"#.txt"]);
}
