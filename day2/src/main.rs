const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

const INPUT: &str = include_str!("./input.txt");

fn is_safe(report: &[i32]) -> bool {
    report
        .windows(2)
        .all(|w| w[1] < w[0] && w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
        || report
            .windows(2)
            .all(|w| w[1] > w[0] && w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect()
}

fn part1(input: &str) -> usize {
    let reports = parse(input);
    reports.iter().filter(|report| is_safe(&report)).count()
}

fn part2(input: &str) -> i32 {
    let reports = parse(input);

    let mut count = 0;
    for report in reports {
        if is_safe(&report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let mut numbers_clone = report.clone();
                numbers_clone.remove(i);
                if is_safe(&numbers_clone) {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 2);
    assert_eq!(part1(INPUT), 670);

    assert_eq!(part2(TEST_INPUT), 4);
    assert_eq!(part2(INPUT), 700);
}
