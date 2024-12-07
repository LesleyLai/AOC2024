const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Equation {
    answer: isize,
    numbers: Vec<isize>,
}

fn parse_equation(line: &str) -> Equation {
    let (answer, numbers) = line.split_once(':').unwrap();
    let answer: isize = answer.parse().unwrap();
    let numbers = numbers
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    Equation { answer, numbers }
}

fn search(answer: isize, numbers: &[isize]) -> bool {
    search_impl(answer, numbers[0], &numbers[1..])
}

fn search_impl(answer: isize, acc: isize, remaining: &[isize]) -> bool {
    if remaining.len() == 0 {
        return answer == acc;
    }

    search_impl(answer, acc + remaining[0], &remaining[1..])
        || search_impl(answer, acc * remaining[0], &remaining[1..])
}

fn part1(input: &str) -> isize {
    let equations: Vec<_> = input.lines().map(parse_equation).collect();

    let mut result = 0;
    for equation in &equations {
        let find_path = search(equation.answer, &equation.numbers);
        if find_path {
            result += equation.answer;
        }
    }

    result
}

fn concat(first: isize, second: isize) -> isize {
    (first.to_string() + second.to_string().as_str())
        .parse()
        .unwrap()
}

fn search2(answer: isize, numbers: &[isize]) -> bool {
    search2_impl(answer, numbers[0], &numbers[1..])
}

fn search2_impl(answer: isize, acc: isize, remaining: &[isize]) -> bool {
    if remaining.len() == 0 {
        return answer == acc;
    }

    search2_impl(answer, acc + remaining[0], &remaining[1..])
        || search2_impl(answer, acc * remaining[0], &remaining[1..])
        || search2_impl(answer, concat(acc, remaining[0]), &remaining[1..])
}

fn part2(input: &str) -> isize {
    let equations: Vec<_> = input.lines().map(parse_equation).collect();

    let mut result = 0;
    for equation in &equations {
        let find_path = search2(equation.answer, &equation.numbers);
        if find_path {
            result += equation.answer;
        }
    }

    result
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 3749);
    assert_eq!(part1(INPUT), 1399219271639);

    assert_eq!(part2(TEST_INPUT), 11387);
    assert_eq!(part2(INPUT), 275791737999003);
}
