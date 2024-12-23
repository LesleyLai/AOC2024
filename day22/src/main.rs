use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::ParallelSlice;
use std::collections::{HashMap, HashSet};
use std::iter::Successors;
use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");

fn next_number(mut seed: isize) -> isize {
    seed ^= (seed << 6) & ((1 << 24) - 1);
    seed ^= (seed >> 5) & ((1 << 24) - 1);
    seed ^= (seed << 11) & ((1 << 24) - 1);
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

fn generate_prices_from_sequence_map(
    seed: isize,
    potential_sequences: &mut HashSet<[isize; 4]>,
) -> HashMap<[isize; 4], isize> {
    let prices: Vec<_> = rand(seed).map(|seed| seed % 10).take(2000).collect();
    let price_diff: Vec<_> = prices.par_windows(2).map(|w| w[1] - w[0]).collect();

    let mut map = HashMap::new();

    for i in 0..(prices.len() - 4) {
        let sequence: [isize; 4] = price_diff[i..i + 4].try_into().unwrap();
        potential_sequences.insert(sequence);
        if !map.contains_key(&sequence) {
            map.insert(sequence, prices[i + 4]);
        }
    }

    map
}

fn part2(input: &[isize]) -> isize {
    let mut potential_sequences = HashSet::new();
    let prices_from_sequence: Vec<_> = input
        .iter()
        .map(|seed| generate_prices_from_sequence_map(*seed, &mut potential_sequences))
        .collect();

    let max_sum = potential_sequences
        .par_iter()
        .map(move |sequence| {
            prices_from_sequence
                .par_iter()
                .map(|price_from_sequence| price_from_sequence.get(sequence).unwrap_or(&0))
                .sum()
        })
        .max()
        .unwrap();

    max_sum
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
