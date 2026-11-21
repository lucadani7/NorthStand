# NorthStand

A Rust solution to [peluzanord](https://infoarena.ro/problema/peluzanord) from the infoarena problems archive.

The problem statement belongs to its authors and to infoarena, so it is not copied here. See the link above for the full statement.

## The task in short

Given an interval `[A, B]` with `1 <= A <= B <= 10^15`, count the numbers in it that are divisible by the sum of their own digits (also known as Harshad or Niven numbers).

## Approach

Checking every number is far too slow for intervals of size up to 10^15, so the solution uses **digit DP**.

- **Count prefixes instead of intervals.** Let `f(X)` be the count for `[1, X]`. The answer is `f(B) - f(A - 1)`.
- **Fix the digit sum.** Every number up to 10^15 has digit sum at most 135. For each candidate sum `s` from 1 to 135, count the numbers `<= X` whose digit sum is exactly `s` and which are divisible by `s`. Adding these counts gives `f(X)`.
- **Suffix table.** For a fixed `s`, `cnt[L][sum][r]` is the number of digit strings of length `L` (leading zeros allowed) with digit sum `sum` and value `≡ r (mod s)`. It is built digit by digit: appending digit `d` maps remainder `r` to `(10r + d) mod s`.
- **Walk the digits of X.** Every number `<= X` shares a prefix with `X`, then has one smaller digit, then free digits. For each such prefix, the required digit sum and remainder of the free part are known, so the count is a single table lookup:

  ```
  prefix * 10^L + suffix ≡ 0 (mod s)  =>  suffix ≡ -prefix * 10^L (mod s)
  ```

**Complexity:** about 130 million simple operations to build the tables for all `s`, and O(16 · 10) lookups per query. The largest input runs in under 0.1 s. Memory is about 2–3 MB, because one table is reused for every `s`.

## Building and running

Requires a recent stable Rust toolchain ([rustup.rs](https://rustup.rs)).

Like on infoarena, the program reads `peluzanord.in` and writes `peluzanord.out` in the current directory:

```bash
# create an input file with the sample test
echo "1 15" > peluzanord.in

# build in release mode and run
cargo run --release

# check the result
cat peluzanord.out   # expected: 11
```

Test files (`*.in`, `*.out`, `*.ok`) are ignored by git.

## Project layout

```
.
├── Cargo.toml
├── LICENSE
├── README.md
└── src/
    └── main.rs
```

## License

This project is licensed under the [Apache License 2.0](LICENSE).

The license covers the solution code only, not the problem statement.