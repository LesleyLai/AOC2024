use std::collections::{HashMap, HashSet};

const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let patterns: Vec<_> = lines.next().unwrap().split(", ").collect();

    lines.next();
    let designs: Vec<_> = lines.collect();
    (patterns, designs)
}

fn possible<'a>(design: &'a str, patterns: &[&str], memo_table: &mut HashSet<&'a str>) -> bool {
    if design.is_empty() {
        return true;
    }

    if memo_table.contains(design) {
        return false;
    }

    let result = patterns.iter().any(|p| {
        design.starts_with(p) && possible(design.strip_prefix(p).unwrap(), patterns, memo_table)
    });

    if !result {
        memo_table.insert(design);
    }

    result
}

fn part1(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut memo_table: HashSet<&str> = HashSet::new();

    designs
        .iter()
        .filter(|design| possible(design, &patterns, &mut memo_table))
        .count()
}

fn combinations<'a>(
    design: &'a str,
    patterns: &[&str],
    memo_table: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&result) = memo_table.get(design) {
        return result;
    }

    let result = patterns
        .iter()
        .filter(|&p| design.starts_with(p))
        .map(|p| combinations(design.strip_prefix(p).unwrap(), patterns, memo_table))
        .sum();

    memo_table.insert(design, result);

    result
}

fn part2(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut memo_table: HashMap<&str, usize> = HashMap::new();

    designs
        .iter()
        .map(|&design| combinations(design, &patterns, &mut memo_table))
        .sum()
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 6);
    assert_eq!(part1(INPUT), 272);

    assert_eq!(part2(TEST_INPUT), 16);
    assert_eq!(part2(INPUT), 1041529704688380);
}
