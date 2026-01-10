use std::fs::read_to_string;

const fn digits10(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

const fn pow10(exp: u32) -> u64 {
    let mut v = 1u64;
    let mut i = 0u32;
    while i < exp {
        v *= 10;
        i += 1;
    }
    v
}

fn has_repeating_pattern(id: u64, chunk_digits: u32) -> bool {
    let base = pow10(chunk_digits);
    let pattern = id % base;
    let mut n = id / base;

    while n > 0 {
        if n % base != pattern {
            return false;
        }
        n /= base;
    }
    true
}

// PART 1: tylko połówki (np. 1212, 38593859)
fn is_invalid(id: u64) -> Option<u64> {
    if id < 11 {
        return None;
    }

    let digits = digits10(id);
    (digits.is_multiple_of(2) && has_repeating_pattern(id, digits / 2)).then_some(id)
}

// PART 2: dowolne powtarzające się bloki (np. 111, 1212, 123123)
fn is_invalid_part2(id: u64) -> Option<u64> {
    if id < 11 {
        return None;
    }

    let digits = digits10(id);
    (1..=digits / 2)
        .filter(|&chunk| digits.is_multiple_of(chunk))
        .any(|chunk| has_repeating_pattern(id, chunk))
        .then_some(id)
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let ranges: Vec<(u64, u64)> = input
        .split(',')
        .map(|part| {
            let (lhs, rhs) = part.split_once('-').expect("zły format");
            (
                lhs.trim().parse::<u64>().expect("nie liczba"),
                rhs.trim().parse::<u64>().expect("nie liczba"),
            )
        })
        .collect();

    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for (min, max) in ranges {
        let counter: u64 = (min..=max).filter_map(is_invalid).sum();
        cnt1 += counter;
        let counter2: u64 = (min..=max).filter_map(is_invalid_part2).sum();
        cnt2 += counter2;
    }
    dbg!(cnt1);
    dbg!(cnt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id() {
        assert_eq!(is_invalid(1), None);
        assert_eq!(is_invalid(110), None);
        assert_eq!(is_invalid(11), Some(11));
        assert_eq!(is_invalid(1212), Some(1212));
        assert_eq!(is_invalid(222222), Some(222222));
        assert_eq!(is_invalid(38593859), Some(38593859));
        assert_eq!(is_invalid(1188511885), Some(1188511885));
    }

    #[test]
    fn test_invalid_part2() {
        assert_eq!(is_invalid_part2(1), None);
        assert_eq!(is_invalid_part2(110), None);
        assert_eq!(is_invalid_part2(1300130013), None);
        assert_eq!(is_invalid_part2(112112112), Some(112112112));
    }

    #[test]
    fn test_invalid_range() {
        let ranges = [
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
        ];

        let sum_all: u64 = ranges
            .into_iter()
            .map(|(min, max)| (min..=max).filter_map(is_invalid).sum::<u64>())
            .sum();
        assert_eq!(sum_all, 1227775554);
    }

    #[test]
    fn test_part2() {
        let collect =
            |min: u64, max: u64| -> Vec<u64> { (min..=max).filter_map(is_invalid_part2).collect() };

        assert_eq!(collect(11, 22), vec![11, 22]);
        assert_eq!(collect(95, 115), vec![99, 111]);
        assert_eq!(collect(998, 1012), vec![999, 1010]);
        assert_eq!(collect(1188511880, 1188511890), vec![1188511885]);
        assert_eq!(collect(222220, 222224), vec![222222]);
        assert_eq!(collect(1698522, 1698528), Vec::<u64>::new());
        assert_eq!(collect(446443, 446449), vec![446446]);
        assert_eq!(collect(38593856, 38593862), vec![38593859]);
        assert_eq!(collect(565653, 565659), vec![565656]);
        assert_eq!(collect(824824821, 824824827), vec![824824824]);
        assert_eq!(collect(2121212118, 2121212124), vec![2121212121]);

        let ranges = [
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ];

        let sum_all: u64 = ranges
            .into_iter()
            .map(|(min, max)| (min..=max).filter_map(is_invalid_part2).sum::<u64>())
            .sum();
        assert_eq!(sum_all, 4174379265);
    }
}
