use regex::Regex;

const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

const TEST_INPUT2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
const INPUT: &str = include_str!("input.txt");

pub fn solve(input: &str) -> (i32, i32) {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|(do|don't)\(\)").unwrap();

    let mut first_answer = 0;
    let mut second_answer = 0;
    let mut enabled = true;
    for c in regex.captures_iter(input) {
        if let Some(d) = c.get(3) {
            enabled = d.as_str() == "do";
        } else {
            let x: i32 = c[1].parse().unwrap();
            let y: i32 = c[2].parse().unwrap();
            first_answer += x * y;
            if enabled {
                second_answer += x * y;
            }
        }
    }

    (first_answer, second_answer)
}

fn main() {
    assert_eq!(solve(TEST_INPUT), (161, 161));
    assert_eq!(solve(TEST_INPUT2), (161, 48));
    assert_eq!(solve(INPUT), (170068701, 78683433));
}
