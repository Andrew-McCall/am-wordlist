//! Bakes each word's byte range into the binary at compile time, so lookups at
//! runtime are just a slice.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let src = PathBuf::from(&manifest).join("src");

    // Smallest size feature wins; none means the full list.
    let file = if env::var_os("CARGO_FEATURE_SIZE_5").is_some() {
        "wordlist_size5.txt"
    } else if env::var_os("CARGO_FEATURE_SIZE_10").is_some() {
        "wordlist_size10.txt"
    } else if env::var_os("CARGO_FEATURE_SIZE_15").is_some() {
        "wordlist_size15.txt"
    } else {
        "wordlist.txt"
    };

    let path = src.join(file);
    println!("cargo:rerun-if-changed={}", path.display());
    println!("cargo:rerun-if-changed=build.rs");

    let content = fs::read_to_string(&path).expect("read wordlist");
    let bytes = content.as_bytes();

    // One (start, end) range per non-empty line, ignoring stray CR and a
    // missing final newline.
    let mut spans = String::new();
    let mut count = 0usize;
    let mut start = 0usize;
    for i in 0..=bytes.len() {
        if i == bytes.len() || bytes[i] == b'\n' {
            let mut end = i;
            if end > start && bytes[end - 1] == b'\r' {
                end -= 1;
            }
            if end > start {
                write!(spans, "({start},{end}),").unwrap();
                count += 1;
            }
            start = i + 1;
        }
    }

    let code = format!(
        "pub static BLOB: &str = include_str!(r\"{path}\");\n\
         pub static SPANS: &[(u32, u32)] = &[{spans}];\n\
         pub const WORD_COUNT: usize = {count};\n",
        path = path.display(),
    );

    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("worddata.rs");
    fs::write(out, code).expect("write worddata.rs");
}
