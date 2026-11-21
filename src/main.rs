use std::fs;

/// Largest possible digit sum: numbers up to 10^15 have at most
/// 15 non-trivial digits (10^15 itself has digit sum 1), so 9 * 15.
const MAX_SUM: usize = 135;
/// Maximum number of free digits after a fixed prefix (16 digits - 1).
const MAX_LEN: usize = 15;

/// Table of suffix counts for a fixed divisor `s`:
/// `get(len, sum, rem)` = how many digit strings of length `len`
/// (leading zeros allowed) have digit sum `sum` and value ≡ `rem` (mod s).
struct SuffixTable {
    data: Vec<u64>,
    s: usize,
}

impl SuffixTable {
    fn new() -> Self {
        let size = (MAX_LEN + 1) * (MAX_SUM + 1) * MAX_SUM;
        SuffixTable { data: vec![0; size], s: 1 }
    }

    #[inline]
    fn idx(&self, len: usize, sum: usize, rem: usize) -> usize {
        (len * (self.s + 1) + sum) * self.s + rem
    }

    #[inline]
    fn get(&self, len: usize, sum: usize, rem: usize) -> u64 {
        self.data[self.idx(len, sum, rem)]
    }

    /// Rebuilds the table for divisor `s`, reusing the same memory.
    fn build(&mut self, s: usize) {
        self.s = s;
        let used = (MAX_LEN + 1) * (s + 1) * s;
        self.data[..used].fill(0);

        let start = self.idx(0, 0, 0);
        self.data[start] = 1;

        for len in 1..=MAX_LEN {
            // A string of length len - 1 has digit sum at most 9 * (len - 1).
            let max_prev_sum = s.min(9 * (len - 1));
            for sum in 0..=max_prev_sum {
                for rem in 0..s {
                    let c = self.get(len - 1, sum, rem);
                    if c == 0 {
                        continue;
                    }
                    // Appending digit d: new remainder = (rem * 10 + d) mod s.
                    let mut new_rem = (rem * 10) % s;
                    let max_digit = 9.min(s - sum);
                    for d in 0..=max_digit {
                        let i = self.idx(len, sum + d, new_rem);
                        self.data[i] += c;
                        new_rem += 1;
                        if new_rem == s {
                            new_rem = 0;
                        }
                    }
                }
            }
        }
    }
}

/// Counts numbers in [0, x] (given as its decimal digits) whose digit sum
/// is exactly `s` and which are divisible by `s`.
fn count_up_to(digits: &[usize], s: usize, table: &SuffixTable, pow10: &[usize]) -> u64 {
    let n = digits.len();
    let mut result = 0u64;
    let mut prefix_sum = 0usize;
    let mut prefix_rem = 0usize;

    for (i, &limit) in digits.iter().enumerate() {
        let free_len = n - 1 - i;
        // Put a digit smaller than x's digit here; the rest is free.
        for d in 0..limit {
            let sum = prefix_sum + d;
            if sum > s {
                break;
            }
            let rem = (prefix_rem * 10 + d) % s;
            // Need: rem * 10^free_len + suffix ≡ 0 (mod s).
            let needed_rem = (s - rem * pow10[free_len] % s) % s;
            result += table.get(free_len, s - sum, needed_rem);
        }
        // Keep x's own digit and continue with the tight prefix.
        prefix_sum += limit;
        if prefix_sum > s {
            return result;
        }
        prefix_rem = (prefix_rem * 10 + limit) % s;
    }

    // x itself.
    if prefix_sum == s && prefix_rem == 0 {
        result += 1;
    }
    result
}

fn to_digits(mut x: u64) -> Vec<usize> {
    let mut digits = Vec::new();
    while x > 0 {
        digits.push((x % 10) as usize);
        x /= 10;
    }
    digits.reverse();
    digits
}

fn main() {
    let input = fs::read_to_string("peluzanord.in").expect("cannot read peluzanord.in");
    let mut it = input.split_ascii_whitespace().map(|t| t.parse::<u64>().unwrap());
    let a = it.next().unwrap();
    let b = it.next().unwrap();

    let digits_b = to_digits(b);
    let digits_a = to_digits(a - 1); // empty when a - 1 == 0

    let mut table = SuffixTable::new();
    let mut total = 0u64;

    for s in 1..=MAX_SUM {
        table.build(s);

        // pow10[k] = 10^k mod s
        let mut pow10 = vec![0usize; MAX_LEN + 1];
        pow10[0] = 1 % s;
        for k in 1..=MAX_LEN {
            pow10[k] = pow10[k - 1] * 10 % s;
        }

        total += count_up_to(&digits_b, s, &table, &pow10);
        total -= count_up_to(&digits_a, s, &table, &pow10);
    }

    fs::write("peluzanord.out", format!("{}\n", total)).expect("cannot write peluzanord.out");
}
