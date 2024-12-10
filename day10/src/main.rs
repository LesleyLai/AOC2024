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

fn push_next_locations(stack: &mut Vec<Vec2>, current: Vec2, grid: &Grid<u8>) {
    for dir in Direction4::all_directions() {
        let next = current + Vec2::from(dir);
        if grid.get(next).is_some_and(|n| *n == grid[current] + 1) {
            stack.push(next);
        }
    }
}

fn find_score(grid: &Grid<u8>, start: Vec2) -> usize {
    let mut stack = Vec::new();
    let mut visited = Grid::new(grid.width, grid.height);

    stack.push(start);

    let mut result = 0;
    while let Some(current) = stack.pop() {
        if !matches!(visited.get(current), Some(false)) {
            continue;
        }
        visited[current] = true;

        if grid[current] == 9 {
            result += 1;
        } else {
            push_next_locations(&mut stack, current, grid);
        }
    }

    result
}

fn find_rating(grid: &Grid<u8>, start: Vec2) -> usize {
    let mut stack = Vec::new();
    stack.push(start);

    let mut result = 0;
    while let Some(current) = stack.pop() {
        if grid[current] == 9 {
            result += 1;
        } else {
            push_next_locations(&mut stack, current, grid);
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

fn solve<const IS_PART2: bool>(input: &str) -> usize {
    let grid = parse(input);

    grid.enumerate()
        .filter(|(_, &height)| height == 0) // is trailhead
        .map(|(coord, _)| {
            if IS_PART2 {
                find_rating(&grid, coord)
            } else {
                find_score(&grid, coord)
            }
        })
        .sum()
}

fn part1(input: &str) -> usize {
    solve::<false>(input)
}

fn part2(input: &str) -> usize {
    solve::<true>(input)
}

fn main() {
    assert_eq!(part1(TEST_INPUT1), 1);
    assert_eq!(part1(TEST_INPUT2), 36);
    assert_eq!(part1(INPUT), 574);

    assert_eq!(part2(TEST_INPUT2), 81);
    assert_eq!(part2(INPUT), 1238);
}
