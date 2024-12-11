use std::collections::HashMap;

const TEST_INPUT: &str = "125 17";

const INPUT: &str = "1750884 193 866395 7 1158 31 35216 0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MemoKey {
    stone: usize,
    iteration_remaining: usize,
}

fn solve_impl(
    memo_table: &mut HashMap<MemoKey, usize>,
    stone: usize,
    iteration_remaining: usize,
) -> usize {
    if iteration_remaining == 0 {
        return 1;
    }

    let key = MemoKey {
        stone,
        iteration_remaining,
    };

    if let Some(&count) = memo_table.get(&key) {
        return count;
    }

    let result;

    if stone == 0 {
        result = solve_impl(memo_table, 1, iteration_remaining - 1);
    } else {
        let string = stone.to_string();
        if string.len() % 2 == 0 {
            result = solve_impl(
                memo_table,
                string[0..string.len() / 2].parse().unwrap(),
                iteration_remaining - 1,
            ) + solve_impl(
                memo_table,
                string[string.len() / 2..].parse().unwrap(),
                iteration_remaining - 1,
            );
        } else {
            result = solve_impl(memo_table, stone * 2024, iteration_remaining - 1);
        }
    }

    memo_table.insert(key, result);

    result
}

fn solve(input: &str, iteration_count: usize) -> usize {
    let stones: Vec<usize> = input.split(" ").map(|s| s.parse().unwrap()).collect();

    let mut memo_table = HashMap::new();

    stones
        .iter()
        .map(|&stone| solve_impl(&mut memo_table, stone, iteration_count))
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, 6), 22);
    assert_eq!(solve(TEST_INPUT, 25), 55312);
    assert_eq!(solve(INPUT, 25), 231278);

    assert_eq!(solve(INPUT, 75), 274229228071551);
}
