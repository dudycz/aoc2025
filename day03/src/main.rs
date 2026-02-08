use std::fs::read_to_string;

fn max_joltage(bank: &str, n: usize) -> u64 {
    let mut ret = vec![0_u64; n];
    let digits: Vec<u64> = bank.bytes().map(|b| (b - b'0') as u64).collect();

    for w in digits.windows(n) {
        for i in 0..n {
            if w[i] > ret[i] {
                ret[i..].copy_from_slice(&w[i..]);
                break;
            }
        }
    }

    ret.iter().fold(0_u64, |acc, &d| acc * 10 + d)
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let part1 = input.lines().map(|line| max_joltage(line, 2)).sum::<u64>();
    dbg!(part1);

    let part2 = input.lines().map(|line| max_joltage(line, 12)).sum::<u64>();
    dbg!(part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        assert_eq!(max_joltage("987654321111111", 2), 98);
        assert_eq!(max_joltage("811111111111119", 2), 89);
        assert_eq!(max_joltage("234234234234278", 2), 78);
        assert_eq!(max_joltage("818181911112111", 2), 92);
        assert_eq!(max_joltage("12121", 3), 221);
    }
}
