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

fn part1(input: &str) -> isize {
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

    let mut updates = vec![];
    for line in lines {
        let update: Vec<isize> = line.split(",").map(|s| s.parse().unwrap()).collect();
        updates.push(update);
    }

    let mut sum = 0;
    for update in updates {
        if is_valid(&update, &rule_graph) {
            sum += update[update.len() / 2]
        }
    }
    sum
}

fn part2(input: &str) -> isize {
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

    let mut sum = 0;
    for line in lines {
        let mut update: Vec<isize> = line.split(",").map(|s| s.parse().unwrap()).collect();

        let valid = is_valid(&update, &rule_graph);

        if !valid {
            for i in 0..update.len() {
                for j in i + 1..update.len() {
                    let x = update[i];
                    let y = update[j];
                    if rule_graph.get(&y).is_some_and(|set| set.contains(&x)) {
                        (update[i], update[j]) = (y, x);
                    }
                }
            }

            sum += update[update.len() / 2]
        }
    }
    sum
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 143);
    assert_eq!(part1(INPUT), 4957);

    assert_eq!(part2(TEST_INPUT), 123);
    assert_eq!(part2(INPUT), 6938);
}
