use utils::{Direction4, Grid, Vec2};

const TEST_INPUT1: &str = "0123
1234
8765
9876";

const TEST_INPUT2: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

const INPUT: &str = include_str!("./input.txt");

fn find_all_trails<const DEDUPLICATE_SAME_DEST: bool>(grid: &Grid<u8>, start: Vec2) -> usize {
    let mut stack = Vec::new();

    let mut visited = Grid::new(grid.width, grid.height);

    stack.push(start);

    let mut result = 0;
    while let Some(current) = stack.pop() {
        if DEDUPLICATE_SAME_DEST {
            if !matches!(visited.get(current), Some(false)) {
                continue;
            }
            visited[current] = true;
        }

        if grid[current] == 9 {
            result += 1;
        } else {
            Direction4::all_directions()
                .iter()
                .map(|&dir| current + Vec2::from(dir))
                .filter(|&next| grid.get(next).is_some_and(|val| *val == grid[current] + 1))
                .for_each(|next| stack.push(next))
        }
    }

    result
}

fn parse(input: &str) -> Grid<u8> {
    let nested: Vec<_> = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|c| *c - b'0').collect())
        .collect();
    Grid::from_nested(&nested)
}

fn solve<const IS_PART1: bool>(input: &str) -> usize {
    let grid = parse(input);

    grid.enumerate()
        .filter(|(_, &height)| height == 0) // is trailhead
        .map(|(coord, _)| find_all_trails::<IS_PART1>(&grid, coord))
        .sum()
}

fn part1(input: &str) -> usize {
    solve::<true>(input)
}

fn part2(input: &str) -> usize {
    solve::<false>(input)
}

fn main() {
    assert_eq!(part1(TEST_INPUT1), 1);
    assert_eq!(part1(TEST_INPUT2), 36);
    assert_eq!(part1(INPUT), 574);

    assert_eq!(part2(TEST_INPUT2), 81);
    assert_eq!(part2(INPUT), 1238);
}
