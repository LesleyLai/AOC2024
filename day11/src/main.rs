use std::collections::HashMap;

use utils::{count_digits, split_by_digit};

const TEST_INPUT: &str = "125 17";

const INPUT: &str = "1750884 193 866395 7 1158 31 35216 0";

fn split_if_even_digits(stone: usize) -> Option<(usize, usize)> {
    let num_of_digits = count_digits(stone);
    (num_of_digits % 2 == 0).then(|| split_by_digit(stone, num_of_digits / 2))
}

fn solve_impl(memo_table: &mut HashMap<usize, usize>, stone: usize, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let key = stone << 8 | depth;
    if let Some(&count) = memo_table.get(&key) {
        return count;
    }

    let result = {
        if stone == 0 {
            solve_impl(memo_table, 1, depth - 1)
        } else if let Some((first, second)) = split_if_even_digits(stone) {
            solve_impl(memo_table, first, depth - 1) + solve_impl(memo_table, second, depth - 1)
        } else {
            solve_impl(memo_table, stone * 2024, depth - 1)
        }
    };

    memo_table.insert(key, result);

    result
}

fn solve(input: &str, depth: usize) -> usize {
    let stones: Vec<usize> = input.split(" ").map(|s| s.parse().unwrap()).collect();

    assert!(depth < 256);

    let mut memo = HashMap::new();
    stones
        .iter()
        .map(|&stone| solve_impl(&mut memo, stone, depth))
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, 6), 22);
    assert_eq!(solve(TEST_INPUT, 25), 55312);
    assert_eq!(solve(INPUT, 25), 231278);

    assert_eq!(solve(INPUT, 75), 274229228071551);
}
