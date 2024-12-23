use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;
use std::iter::Successors;
use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");

fn next_number(mut seed: isize) -> isize {
    seed ^= seed * 64;
    seed %= 16777216;

    seed ^= seed >> 5;
    seed %= 16777216;

    seed ^= seed * 2048;
    seed %= 16777216;

    seed
}

fn rand(seed: isize) -> Successors<isize, fn(&isize) -> Option<isize>> {
    std::iter::successors(Some(seed), |&seed| Some(next_number(seed)))
}

fn part1(input: &[isize]) -> isize {
    input
        .iter()
        .map(|&seed| rand(seed).skip(2000).next().unwrap())
        .sum()
}

fn find_sell_price(seed: isize, sequence: (isize, isize, isize, isize)) -> Option<isize> {
    let prices = rand(seed).map(|seed| seed % 10).take(2000);
    let price_diff = prices.clone().tuple_windows().map(|(a, b)| b - a);
    price_diff
        .tuple_windows()
        .zip(prices.skip(4))
        .find(|((a, b, c, d), _)| (*a, *b, *c, *d) == sequence)
        .map(|(_, price)| price)
}

fn find_total_price(input: &[isize], sequence: (isize, isize, isize, isize)) -> isize {
    input
        .iter()
        .filter_map(|&seed| find_sell_price(seed, sequence))
        .sum()
}

fn part2(input: &[isize]) -> isize {
    let mut sequence_candidates = HashSet::new();
    for &seed in input {
        let prices = rand(seed).map(|seed| seed % 10).take(2000);
        let price_diff = prices.tuple_windows().map(|(a, b)| b - a);
        sequence_candidates.extend(price_diff.tuple_windows::<(_, _, _, _)>())
    }
    let sequence_candidates: Vec<_> = sequence_candidates.iter().collect();

    let best_sequence = sequence_candidates
        .par_iter()
        .max_by_key(|sequence| find_total_price(input, ***sequence))
        .unwrap();

    find_total_price(input, **best_sequence)
}

fn main() {
    let input: Vec<_> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

    assert_eq!(part1(&[1, 10, 100, 2024]), 37327623);
    assert_eq!(part1(&input), 15335183969);

    assert_eq!(part2(&[1, 2, 3, 2024]), 23);

    let now = Instant::now();
    assert_eq!(part2(&input), 1696);
    println!("Part 2: {}s", now.elapsed().as_secs_f64());
}
