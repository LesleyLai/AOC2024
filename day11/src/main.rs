use std::collections::HashMap;

use utils::{count_digits, split_by_digit};

const TEST_INPUT: &str = "125 17";

const INPUT: &str = "1750884 193 866395 7 1158 31 35216 0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MemoKey {
    stone: usize,
    iteration_remaining: usize,
}

struct Memo {
    table: HashMap<MemoKey, usize>,
}

impl Memo {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    fn solve(&mut self, stone: usize, iteration_remaining: usize) -> usize {
        if iteration_remaining == 0 {
            return 1;
        }

        let key = MemoKey {
            stone,
            iteration_remaining,
        };

        if let Some(&count) = self.table.get(&key) {
            return count;
        }

        let result = {
            if stone == 0 {
                self.solve(1, iteration_remaining - 1)
            } else {
                let num_of_digits = count_digits(stone);
                if num_of_digits % 2 == 0 {
                    let (first, second) = split_by_digit(stone, num_of_digits / 2);
                    self.solve(first, iteration_remaining - 1)
                        + self.solve(second, iteration_remaining - 1)
                } else {
                    self.solve(stone * 2024, iteration_remaining - 1)
                }
            }
        };

        self.table.insert(key, result);

        result
    }
}

fn solve(input: &str, iteration_count: usize) -> usize {
    let stones: Vec<usize> = input.split(" ").map(|s| s.parse().unwrap()).collect();

    let mut memo = Memo::new();

    stones
        .iter()
        .map(|&stone| memo.solve(stone, iteration_count))
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, 6), 22);
    assert_eq!(solve(TEST_INPUT, 25), 55312);
    assert_eq!(solve(INPUT, 25), 231278);

    assert_eq!(solve(INPUT, 75), 274229228071551);
}
