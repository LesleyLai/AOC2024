use itertools::Itertools;
use std::process::exit;
use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone)]
struct Graph<'a> {
    data: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add(&mut self, first: &'a str, second: &'a str) {
        self.data.entry(first).or_default().insert(second);
        self.data.entry(second).or_default().insert(first);
    }

    fn nodes(&self) -> impl '_ + Iterator<Item = &'a str> {
        self.data.keys().map(|s| *s)
    }

    fn neighbors(&self, node: &str) -> impl '_ + Iterator<Item = &'a str> {
        self.data[node].iter().map(|s| *s)
    }

    fn iter(&self) -> impl '_ + Iterator<Item = (&&'a str, &HashSet<&'a str>)> {
        self.data.iter()
    }

    fn is_connected_to(&self, node1: &str, node2: &str) -> bool {
        self.data[node1].contains(node2)
    }
}

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
        graph.add(&line[0..2], &line[3..5]);
    }
    graph
}

fn part1(graph: &Graph) -> usize {
    let mut three_connected: HashSet<[&str; 3]> = HashSet::new();
    for node in graph.nodes() {
        for neighbor in graph.neighbors(node) {
            for neighbors_neighbor in graph.neighbors(neighbor) {
                if graph.is_connected_to(node, neighbors_neighbor) {
                    let mut array = [node, neighbor, neighbors_neighbor];
                    array.sort();
                    three_connected.insert(array);
                }
            }
        }
    }

    three_connected
        .iter()
        .filter(|&array| array.iter().any(|&node| node.starts_with('t')))
        .count()
}

fn part2(graph: &Graph) -> String {
    let max_possible_size = graph
        .iter()
        .map(|(_, neighbors)| neighbors.len())
        .max()
        .unwrap();

    for i in (2..=max_possible_size).rev() {
        for node in graph.nodes() {
            let neighbors_with_self = once(node).chain(graph.neighbors(node));

            for mut combination in neighbors_with_self.combinations(i) {
                let mut all_neighbor = combination
                    .iter()
                    .tuple_combinations()
                    .all(|(a, b)| graph.is_connected_to(a, b));

                if all_neighbor {
                    combination.sort();
                    return combination.join(",");
                }
            }
        }
    }
    unreachable!()
}

fn main() {
    let test_input_graph = parse_input(TEST_INPUT);
    let input_graph = parse_input(INPUT);

    assert_eq!(part1(&test_input_graph), 7);
    assert_eq!(part1(&input_graph), 1170);

    assert_eq!(part2(&test_input_graph), "co,de,ka,ta");
    assert_eq!(
        part2(&input_graph),
        "bo,dd,eq,ik,lo,lu,ph,ro,rr,rw,uo,wx,yg"
    );
}
