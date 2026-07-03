# am-word

**am-word** is a fast, embedded word list you can index into. The whole list is
baked into the binary at compile time, with each word's byte range computed
ahead of time in `build.rs`, so lookups are a pure slice with zero runtime
initialization and zero allocation.

## Features

- Embeds a word list directly into your binary — no files to ship or load.
- O(1) indexed lookup returning a `&'static str` that borrows from the embedded blob.
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
am-word = "0.1"
```

Select a smaller, evenly-sampled list to shrink the binary (enable at most one;
if several are set, the smallest wins):

```toml
[dependencies]
am-word = { version = "0.1", default-features = false, features = ["size-10"] }
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
// Total number of words (also available as the `am_word::LEN` const).
let n = am_word::len();

// Look up by index — O(1), no allocation. Out-of-range returns `None`.
let word = am_word::get(0).unwrap();
assert!(am_word::get(n).is_none());

// Pick a random word.
let word = am_word::get(rng % am_word::LEN).unwrap();

// Iterate over every word in order.
for w in am_word::iter() {
    println!("{w}");
}
```

### Running the demo

The crate ships a small binary that prints a few sample lookups:
```bash
cargo run
```

### Regenerating the wordlists

Requires `wget`, `awk`, `cut`, and `grep`:
```bash
./update-wordlist.sh
```

### Output

`get` and `iter` yield the main words, one per entry, formatted as:
```
apple
banana
```

## Error Handling

- `get(index)` returns `None` for any out-of-range index rather than panicking.

## Use Cases

Ideal for anything needing a formatted word list with no runtime setup — random
word/passphrase generation, games, placeholder data, and CLI tools.

## License

MIT License
