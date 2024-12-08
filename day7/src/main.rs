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

fn concat(first: isize, second: isize) -> isize {
    (first.to_string() + second.to_string().as_str())
        .parse()
        .unwrap()
}

fn search<const IS_PART2: bool>(answer: isize, numbers: &[isize]) -> bool {
    let mut stack = Vec::with_capacity(100);
    stack.push((numbers[0], &numbers[1..]));

    while let Some((acc, remaining)) = stack.pop() {
        if acc > answer {
            continue;
        }

        match &remaining {
            &[] => {
                if acc == answer {
                    return true;
                }
            }
            &[head, tail @ ..] => {
                stack.push((acc + head, tail));
                stack.push((acc * head, tail));
                if IS_PART2 {
                    stack.push((concat(acc, *head), tail));
                }
            }
        }
    }

    false
}

fn solve(equations: &[Equation], is_part2: bool) -> isize {
    equations
        .iter()
        .filter(|equation| {
            if is_part2 {
                search::<true>(equation.answer, &equation.numbers)
            } else {
                search::<false>(equation.answer, &equation.numbers)
            }
        })
        .map(|eq| eq.answer)
        .sum()
}

fn part1(equations: &[Equation]) -> isize {
    solve(equations, false)
}

fn part2(equations: &[Equation]) -> isize {
    solve(equations, true)
}

fn main() {
    let test_equations: Vec<_> = TEST_INPUT.lines().map(parse_equation).collect();
    let equations: Vec<_> = INPUT.lines().map(parse_equation).collect();

    assert_eq!(part1(&test_equations), 3749);
    assert_eq!(part1(&equations), 1399219271639);

    assert_eq!(part2(&test_equations), 11387);
    assert_eq!(part2(&equations), 275791737999003);
}
