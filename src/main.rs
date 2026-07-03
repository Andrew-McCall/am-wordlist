use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let len = am_wordlist::len();
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as usize;

    let word = am_wordlist::get(ts * 19 % len).unwrap();
    println!("{word}");
}
