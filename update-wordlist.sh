#!/bin/sh
# Download the Moby thesaurus and build the embedded wordlists.
#
#   1. Keep the first word of each line (before the comma).
#   2. Drop any entry containing a hyphen or an uppercase letter.
#   3. Write the full list to src/wordlist.txt.
#   4. Emit evenly-sampled size tiers (5k/10k/15k) that build.rs can embed.
set -eu

URL="https://www.gutenberg.org/files/3202/files/mthesaur.txt"

# Resolve paths relative to this script so it works from any cwd.
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
SRC="$SCRIPT_DIR/src"

wget -O "$SCRIPT_DIR/mthesaur.txt" "$URL"

cut -d',' -f1 "$SCRIPT_DIR/mthesaur.txt" \
	| grep -v -- '-' \
	| grep -v '[A-Z]' \
	> "$SRC/wordlist.txt"

# Downsample the full list into evenly-spaced tiers. Each tier walks the full
# list at a constant stride so the sample stays spread across the alphabet.
for n in 5000 10000 15000; do
	awk -v n="$n" '
		{a[NR] = $0}
		END {
			t = NR
			if (n > t) n = t
			for (i = 0; i < n; i++) print a[int(i * t / n) + 1]
		}
	' "$SRC/wordlist.txt" > "$SRC/wordlist_size${n%000}.txt"
done

wc -l "$SRC/wordlist.txt" "$SRC"/wordlist_size*.txt
