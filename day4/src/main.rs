const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

const INPUT: &str = include_str!("input.txt");

use utils::Grid;

fn parse(input: &str) -> Grid<u8> {
    let nested: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    Grid::from_nested(&nested)
}

fn matches(grid: &Grid<u8>, current: (isize, isize), direction: (isize, isize)) -> bool {
    let (x, y) = current;
    let (dx, dy) = direction;

    if !matches!(grid.get((x + dx, y + dy)), Some(b'M')) {
        return false;
    }

    if !matches!(grid.get((x + dx * 2, y + dy * 2)), Some(b'A')) {
        return false;
    }

    if !matches!(grid.get((x + dx * 3, y + dy * 3)), Some(b'S')) {
        return false;
    }

    true
}

fn matches2(grid: &Grid<u8>, current: (isize, isize)) -> bool {
    let (x, y) = current;

    let a = (matches!(grid.get((x, y)), Some(b'M'))
        && matches!(grid.get((x + 1, y + 1)), Some(b'A'))
        && matches!(grid.get((x + 2, y + 2)), Some(b'S')))
        || (matches!(grid.get((x, y)), Some(b'S'))
            && matches!(grid.get((x + 1, y + 1)), Some(b'A'))
            && matches!(grid.get((x + 2, y + 2)), Some(b'M')));

    let b = (matches!(grid.get((x + 2, y)), Some(b'M'))
        && matches!(grid.get((x + 1, y + 1)), Some(b'A'))
        && matches!(grid.get((x, y + 2)), Some(b'S')))
        || (matches!(grid.get((x + 2, y)), Some(b'S'))
            && matches!(grid.get((x + 1, y + 1)), Some(b'A'))
            && matches!(grid.get((x, y + 2)), Some(b'M')));

    a && b
}

fn part1(grid: &Grid<u8>) -> usize {
    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some(b'X') = grid.get((x, y)) {
                if matches(&grid, (x, y), (-1, -1)) {
                    count += 1;
                }
                if matches(&grid, (x, y), (-1, 0)) {
                    count += 1;
                }
                if matches(&grid, (x, y), (-1, 1)) {
                    count += 1;
                }
                if matches(&grid, (x, y), (0, -1)) {
                    count += 1;
                }
                if matches(&grid, (x, y), (0, 1)) {
                    count += 1;
                }
                if matches(&grid, (x, y), (1, -1)) {
                    count += 1;
                }
                if matches(&grid, (x, y), (1, 0)) {
                    count += 1;
                }
                if matches(&grid, (x, y), (1, 1)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(grid: &Grid<u8>) -> usize {
    let mut count = 0;
    for y in 0..(grid.height - 2) {
        for x in 0..(grid.width - 2) {
            if matches2(&grid, (x, y)) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let test_grid = parse(&TEST_INPUT);
    let grid = parse(&INPUT);

    assert_eq!(part1(&test_grid), 18);
    assert_eq!(part1(&grid), 2543);
    assert_eq!(part2(&test_grid), 9);
    assert_eq!(part2(&grid), 1930);
}
