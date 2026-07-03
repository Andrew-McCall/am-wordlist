# am-wordlist

**am-wordlist** is a fast, embedded word list you can index into. The whole list is
baked into the binary at compile time, with each word's byte range computed
ahead of time in `build.rs`, so lookups are a pure slice with zero runtime
initialization and zero allocation.

## Features

- Embeds a word list directly into your binary — no files to ship or load.
- O(1) indexed lookup returning a `&'static str` that borrows from the embedded blob.
- Range lookups (`get_range`) yield multiple words as an allocation-free iterator.
- Zero runtime initialization and zero allocation; byte ranges are computed at build time.
- Selectable list sizes (`size-5`, `size-10`, `size-15`) to trade coverage for binary size.

## Requirements

- Rust with edition 2024 support.

### Input File Format

The embedded lists are generated from the [Moby thesaurus](https://www.gutenberg.org/files/3202/files/mthesaur.txt),
whose lines look like:
```
word, association1, association2,...
```
Example:
```
apple, fruit, tree, orchard
banana, fruit, yellow
```
`update-wordlist.sh` keeps the first word of each line, drops any entry with a
hyphen or uppercase letter, and writes the result to `src/wordlist.txt`.

## Usage

### Adding the dependency

```toml
[dependencies]
am-wordlist = "1.0"
```

Select a smaller, evenly-sampled list to shrink the binary (enable at most one;
if several are set, the smallest wins):

```toml
[dependencies]
am-wordlist = { version = "1.0", default-features = false, features = ["size-10"] }
```

| Feature | Approx. words |
| --- | --- |
| *(default)* | full list ~25,000 |
| `size-15` | ~15,000 |
| `size-10` | ~10,000 |
| `size-5` | ~5,000 |

> **Note:** the subsets are sampled evenly across the alphabetical list purely
> for size — they are **not** frequency-ranked "common word" lists.

### In code

```rust
// Total number of words (also available as the `am_wordlist::LEN` const).
let n = am_wordlist::len();

// Look up by index — O(1), no allocation. Out-of-range returns `None`.
let word = am_wordlist::get(0).unwrap();
assert!(am_wordlist::get(n).is_none());

// Pick a random word.
let word = am_wordlist::get(rng % am_wordlist::LEN).unwrap();

// Slice out multiple words at once as an allocation-free iterator. Accepts any
// range (`a..b`, `a..=b`, `a..`, `..b`, `..`); out-of-range returns `None`.
let some: Vec<&str> = am_wordlist::get_range(10..20).unwrap().collect();

// Iterate over every word in order.
for w in am_wordlist::iter() {
    println!("{w}");
}
```

### The `am-word` binary

The crate ships a small binary named `am-word` that prints a single
pseudo-random word to stdout — a handy one-shot word generator:

```bash
cargo run                          # from a checkout of this repo
am-word                            # after `cargo install am-wordlist`
```

Each run prints exactly one word, e.g.:
```
orchard
```

### Regenerating the wordlists

Requires `wget`, `awk`, `cut`, and `grep`:
```bash
./update-wordlist.sh
```

### Output

`get`, `get_range`, and `iter` yield the main words, one per entry, formatted as:
```
apple
banana
```

## Error Handling

- `get(index)` returns `None` for any out-of-range index rather than panicking.
- `get_range(range)` returns `None` for an out-of-bounds range (or one whose
  start is past its end) rather than panicking; empty ranges yield nothing.

## Use Cases

Ideal for anything needing a formatted word list with no runtime setup — random
word/passphrase generation, games, placeholder data, and CLI tools.

## License

MIT License
