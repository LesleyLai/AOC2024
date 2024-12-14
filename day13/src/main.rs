use utils::Vec2;

const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

const INPUT: &str = include_str!("./input.txt");

fn parse_vec2(line: &str, splitter: &str) -> Vec2 {
    let (first, second) = line.split_once(": ").unwrap().1.split_once(", ").unwrap();
    let x: isize = first.split_once(splitter).unwrap().1.parse().unwrap();
    let y: isize = second.split_once(splitter).unwrap().1.parse().unwrap();
    Vec2::new(x, y)
}

struct Parser<'a> {
    lines: std::str::Lines<'a>,
}

fn parse(input: &str) -> Parser {
    let lines = input.lines();
    Parser { lines }
}

impl<'a> Iterator for Parser<'a> {
    type Item = (Vec2, Vec2, Vec2);

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let button_a = parse_vec2(line, "+");
            let button_b = parse_vec2(self.lines.next().unwrap(), "+");
            let prize = parse_vec2(self.lines.next().unwrap(), "=");
            self.lines.next();

            (button_a, button_b, prize)
        })
    }
}

fn solve(button_a: Vec2, button_b: Vec2, prize: Vec2) -> isize {
    // Solve linear equation button_a * a + button_b * b = prize
    // Cramer's rule
    let det = button_a.x * button_b.y - button_b.x * button_a.y;
    let det_a = prize.x * button_b.y - button_b.x * prize.y;
    let det_b = button_a.x * prize.y - prize.x * button_a.y;

    let a = det_a / det;
    let b = det_b / det;

    if button_a * a + button_b * b == prize {
        a * 3 + b
    } else {
        0
    }
}

fn part1(input: &str) -> isize {
    parse(input)
        .map(|(button_a, button_b, prize)| solve(button_a, button_b, prize))
        .sum()
}

fn part2(input: &str) -> isize {
    parse(input)
        .map(|(button_a, button_b, prize)| {
            let prize = prize + Vec2::new(10000000000000, 10000000000000);
            solve(button_a, button_b, prize)
        })
        .sum()
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 480);
    assert_eq!(part1(INPUT), 29517);

    assert_eq!(part2(&TEST_INPUT), 875318608908);
    assert_eq!(part2(&INPUT), 103570327981381);
}
