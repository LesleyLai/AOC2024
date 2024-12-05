use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

const INPUT: &str = include_str!("input.txt");

type RuleGraph = HashMap<isize, HashSet<isize>>;
type Update = Vec<isize>;

fn parse(input: &str) -> (RuleGraph, Vec<Update>) {
    let mut lines = input.lines();

    let mut rule_graph = HashMap::new(); // key|value
    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }
        let (first, second) = line.split_once('|').unwrap();
        let (first, second): (isize, isize) = (first.parse().unwrap(), second.parse().unwrap());

        rule_graph
            .entry(first)
            .or_insert(HashSet::new())
            .insert(second);
    }

    let updates = lines
        .map(|line| {
            line.split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    (rule_graph, updates)
}

fn is_valid(update: &[isize], rule_graph: &HashMap<isize, HashSet<isize>>) -> bool {
    for (i, x) in update.iter().enumerate() {
        for y in update[i + 1..].iter() {
            if rule_graph.get(y).is_some_and(|set| set.contains(x)) {
                return false;
            }
        }
    }
    true
}

fn sort_by_graph(update: &mut [isize], rule_graph: &RuleGraph) {
    update.sort_by(|x, y| {
        if rule_graph.get(&y).is_some_and(|set| set.contains(&x)) {
            Ordering::Less
        } else if rule_graph.get(&x).is_some_and(|set| set.contains(&y)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

fn part1(rule_graph: &RuleGraph, updates: &[Update]) -> isize {
    updates
        .iter()
        .filter(|update| is_valid(&update, &rule_graph))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(rule_graph: &RuleGraph, updates: &[Update]) -> isize {
    let mut sum = 0;
    for update in updates {
        if !is_valid(&update, &rule_graph) {
            let mut update = update.clone();
            sort_by_graph(&mut update, &rule_graph);
            sum += update[update.len() / 2]
        }
    }
    sum
}

fn main() {
    let (test_rules, test_updates) = parse(TEST_INPUT);
    let (rules, updates) = parse(INPUT);

    assert_eq!(part1(&test_rules, &test_updates), 143);
    assert_eq!(part1(&rules, &updates), 4957);

    assert_eq!(part2(&test_rules, &test_updates), 123);
    assert_eq!(part2(&rules, &updates), 6938);
}
