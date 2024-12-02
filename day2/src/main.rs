const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

const INPUT: &str = include_str!("./input.txt");

fn is_safe(report: &[i32]) -> bool {
    let mut differences = report.windows(2).map(|w| w[1] - w[0]);
    differences.clone().all(|d| d >= 1 && d <= 3) || differences.all(|d| d <= -1 && d >= -3)
}

fn is_safe_tolerate(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut report_clone = report.to_vec();
        report_clone.remove(i);
        if is_safe(&report_clone) {
            return true;
        }
    }
    false
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

fn part2(input: &str) -> usize {
    let reports = parse(input);

    reports
        .iter()
        .filter(|report| is_safe(&report) || is_safe_tolerate(&report))
        .count()
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 2);
    assert_eq!(part1(INPUT), 670);

    assert_eq!(part2(TEST_INPUT), 4);
    assert_eq!(part2(INPUT), 700);
}
