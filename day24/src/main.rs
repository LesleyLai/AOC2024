use std::collections::{BTreeMap, HashSet};

const TEST_INPUT: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

const TEST_INPUT2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    output: &'a str,
    lhs: &'a str,
    rhs: &'a str,
    op: Operation,
}

fn parse_gate(line: &str) -> Gate {
    let (input, output) = line.split_once(" -> ").unwrap();
    let mut splitted = input.split(" ");
    let (lhs, op, rhs) = (
        splitted.next().unwrap(),
        splitted.next().unwrap(),
        splitted.next().unwrap(),
    );
    let op = match op {
        "AND" => Operation::And,
        "OR" => Operation::Or,
        "XOR" => Operation::Xor,
        _ => unreachable!(),
    };
    Gate {
        output,
        lhs,
        rhs,
        op,
    }
}

fn sort_gates<'a>(gates: &'a Vec<Gate>, mut inputs: HashSet<&'a str>) -> Vec<Gate<'a>> {
    let mut result: Vec<_> = Vec::with_capacity(gates.len());
    let mut gates = gates.clone();

    while !gates.is_empty() {
        for i in 0..gates.len() {
            let gate = gates[i];
            if inputs.contains(gate.lhs) && inputs.contains(gate.rhs) {
                assert!(inputs.insert(gate.output));
                result.push(gate);
                gates.swap_remove(i);

                break;
            }
        }
    }

    result
}

fn get_number(states: &BTreeMap<&str, u8>, start_with: char) -> usize {
    let mut result: usize = 0;
    for wire in states
        .keys()
        .filter(|wire| wire.starts_with(start_with))
        .rev()
    {
        assert!(states[*wire] <= 1);

        result <<= 1;
        result += states[wire] as usize;
    }
    result
}

fn simulate<'a>(mut states: BTreeMap<&'a str, u8>, gates: &'a Vec<Gate>) -> usize {
    for gate in gates {
        assert!(!states.contains_key(&gate.output));

        let lhs = *states.get(&gate.lhs).unwrap();
        let rhs = *states.get(&gate.rhs).unwrap();

        let result = match gate.op {
            Operation::And => lhs & rhs,
            Operation::Or => lhs | rhs,
            Operation::Xor => lhs ^ rhs,
        };
        states.insert(gate.output, result);
    }

    get_number(&states, 'z')
}

fn part1(input: &str) -> usize {
    let (system_inputs, gates) = input.split_once("\n\n").unwrap();

    let mut states = BTreeMap::new();
    for (wire, value) in system_inputs.lines().map(|s| s.split_once(": ").unwrap()) {
        let value: u8 = value.parse().unwrap();
        states.insert(wire, value);
    }

    let gates: Vec<_> = gates.lines().map(|line| parse_gate(line)).collect();
    let gates = sort_gates(&gates, states.keys().map(|s| *s).collect::<HashSet<&str>>());

    simulate(states, &gates)
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 4);
    assert_eq!(part1(TEST_INPUT2), 2024);

    assert_eq!(part1(INPUT), 69201640933606);

    // part 2: manually find
    let mut list = ["kfp", "hbs", "dhq", "z18", "z22", "pdg", "z27", "jcp"];
    list.sort();
    println!("{}", list.join(","))
}
