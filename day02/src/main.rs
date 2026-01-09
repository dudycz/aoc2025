use std::fs::read_to_string;

const fn divider(id: u64) -> u64 {
    if id < 1_000 {
        10
    } else if id < 100_000 {
        100
    } else if id < 10_000_000 {
        1_000
    } else if id < 1_000_000_000 {
        10_000
    } else {
        100_000
    }
}

fn is_invalid(id: u64) -> Option<u64> {
    let d = divider(id);
    (id % (d + 1) == 0).then_some(id)
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
    for (min, max) in ranges {
        let counter: u64 = (min..max + 1).filter_map(is_invalid).sum();
        cnt1 += counter;
    }
    dbg!(cnt1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id() {
        assert_eq!(is_invalid(1), None);
        assert_eq!(is_invalid(10), None);
        assert_eq!(is_invalid(11), Some(11));
        assert_eq!(is_invalid(1212), Some(1212));
        assert_eq!(is_invalid(222222), Some(222222));
        assert_eq!(is_invalid(38593859), Some(38593859));
        assert_eq!(is_invalid(1188511885), Some(1188511885));
    }

    #[test]
    fn test_invalid_range() {
        let s: u64 = (11..22 + 1).filter_map(is_invalid).sum();
        assert_eq!(s, 33);
        let s: u64 = (1188511880..1188511890 + 1).filter_map(is_invalid).sum();
        assert_eq!(s, 1188511885);
    }
}

// Mam zakres: iteruję się w nim w poszukiwaniu invalid_id
// invalid_id to taki ID który składa się z identycznych par
// a) konwersja do stringa + split w połowie
// b) dzielenie i reszta z dzielenia przez odpowiednią liczbę: 10^n
// >10 <100
// >1000 <10000
// >100000 <1000000

//
// 11-22 has two invalid IDs, 11 and 22.
// 95-115 has one invalid ID, 99.
// 998-1012 has one invalid ID, 1010.
// 1188511880-1188511890 has one invalid ID, 1188511885.
// 222220-222224 has one invalid ID, 222222.
// 1698522-1698528 contains no invalid IDs.
// 446443-446449 has one invalid ID, 446446.
// 38593856-38593862 has one invalid ID, 38593859.
//
// Adding up all the invalid IDs in this example produces 1227775554.
