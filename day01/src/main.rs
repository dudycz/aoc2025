use std::fs::read_to_string;

fn rotate(current_pos: i32, input: &str) -> (i32, i32) {
    let dir = input.chars().next().unwrap();
    let value = input[1..].parse::<i32>().unwrap_or(0);

    let total_offset = match dir {
        'R' => current_pos + value,
        'L' => current_pos - value,
        _ => current_pos,
    };

    let cycles = if total_offset > current_pos {
        (total_offset - 1).div_euclid(100) - current_pos.div_euclid(100)
    } else if total_offset < current_pos {
        (current_pos - 1).div_euclid(100) - total_offset.div_euclid(100)
    } else {
        0
    };

    let pos = total_offset.rem_euclid(100);

    (pos, cycles)
}

fn main() {
    let mut pos = 50;
    let mut zero_cnt = 0;
    let mut cycle_cnt = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let (new_pos, cycles) = rotate(pos, line);
        cycle_cnt += cycles;
        if new_pos == 0 {
            zero_cnt += 1;
        }
        pos = new_pos
    }
    dbg!(zero_cnt);
    dbg!(cycle_cnt + zero_cnt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo() {
        assert_eq!(rotate(20, "L16"), (4, 0));
        assert_eq!(rotate(20, "R16"), (36, 0));
        assert_eq!(rotate(10, "L16"), (94, 1));
        assert_eq!(rotate(90, "R16"), (6, 1));
        assert_eq!(rotate(50, "R1000"), (50, 10));
        assert_eq!(rotate(10, "L110"), (0, 1));
        assert_eq!(rotate(0, "L5"), (95, 0));
        assert_eq!(rotate(52, "R48"), (0, 0));
    }
}
