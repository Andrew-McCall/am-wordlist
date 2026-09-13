use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let len = am_wordlist::len();
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();

    let word = am_wordlist::get(pick(ts, len)).unwrap();
    println!("{word}");
}

/// Map a clock reading onto a word index.
///
/// The clock is run through SplitMix64's finalizer so that nearby launch
/// times land far apart in the list. Every step is wrapping `u64` arithmetic,
/// so this cannot overflow and behaves identically on 32- and 64-bit targets.
///
/// This is a convenience for the CLI, not a source of secrets: the input is
/// just a timestamp, so the output is guessable by anyone who knows roughly
/// when the command ran.
fn pick(micros: u128, len: usize) -> usize {
    let mut z = micros as u64;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^= z >> 31;
    (z % len as u64) as usize
}

#[cfg(test)]
mod tests {
    use super::pick;

    /// `as_micros()` is a `u128`; narrowing it must not overflow on any target.
    /// On a 32-bit target `ts as usize` truncates, and `ts * 19` then overflows
    /// for ~95% of clock values — a panic in debug builds.
    #[test]
    fn pick_stays_in_range_for_extreme_timestamps() {
        let len = 24_743;
        for micros in [0u128, 1, 1_789_298_589_412_326, u64::MAX as u128, u128::MAX] {
            let i = pick(micros, len);
            assert!(
                i < len,
                "index {i} out of range (len {len}) for micros {micros}"
            );
        }
    }

    /// The index must not be a linear function of the clock: `ts * 19 % len`
    /// moves by a constant stride of 19 for every consecutive microsecond,
    /// which makes the output predictable from an approximate launch time.
    #[test]
    fn pick_does_not_advance_by_a_constant_stride() {
        let len = 24_743;
        let base = 1_789_298_589_412_326u128;
        let idx: Vec<usize> = (0..256).map(|k| pick(base + k, len)).collect();
        let deltas: Vec<isize> = idx
            .windows(2)
            .map(|w| w[1] as isize - w[0] as isize)
            .collect();
        assert!(
            deltas.iter().any(|d| *d != deltas[0]),
            "indices advance by a constant stride of {}",
            deltas[0]
        );
    }
}
