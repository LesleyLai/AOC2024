use std::collections::HashMap;

const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let (first, second) = line.split_once("   ").unwrap();

        list1.push(first.parse().unwrap());
        list2.push(second.parse().unwrap());
    }
    (list1, list2)
}

fn part1(input: &str) -> i32 {
    let (mut list1, mut list2) = parse_input(input);

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn part2(input: &str) -> i32 {
    let (list1, list2) = parse_input(input);

    let mut occurrence_counts = HashMap::new();
    for num in list2 {
        *occurrence_counts.entry(num).or_insert(0) += 1;
    }

    list1.iter().fold(0, |acc, num| {
        acc + num * occurrence_counts.get(&num).unwrap_or(&0)
    })
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 11);
    assert_eq!(part1(INPUT), 1506483);

    assert_eq!(part2(TEST_INPUT), 31);
    assert_eq!(part2(INPUT), 23126924);
}
