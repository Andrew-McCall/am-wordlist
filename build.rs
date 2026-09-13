//! Bakes the word list into the binary at compile time: a normalized blob plus
//! one start offset per word, so a lookup at runtime is just a slice.

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

    // Re-emit the list with exactly one '\n' after every word. Normalizing here
    // means stray CR, blank lines and a missing final newline never reach the
    // index, so a word's end is always the next word's start less one -- which
    // is what lets us store starts alone instead of (start, end) pairs.
    let mut blob = String::with_capacity(content.len());
    let mut starts = String::new();
    let mut count = 0usize;
    for line in content.lines() {
        let word = line.trim_end_matches('\r');
        if word.is_empty() {
            continue;
        }
        write!(starts, "{},", blob.len()).unwrap();
        blob.push_str(word);
        blob.push('\n');
        count += 1;
    }
    // Sentinel one past the final separator, so `blob[STARTS[i]..STARTS[i+1]-1]`
    // resolves every word including the last.
    write!(starts, "{},", blob.len()).unwrap();

    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out.join("wordblob.txt"), &blob).expect("write wordblob.txt");

    let code = format!(
        "pub static BLOB: &str = include_str!(concat!(env!(\"OUT_DIR\"), \"/wordblob.txt\"));\n\
         pub static STARTS: &[u32] = &[{starts}];\n\
         pub const WORD_COUNT: usize = {count};\n",
    );
    fs::write(out.join("worddata.rs"), code).expect("write worddata.rs");
}
