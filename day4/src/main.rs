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

use utils::{two_dimension_iter, Grid, Vec2, ALL_EIGHT_DIRECTIONS};

fn parse(input: &str) -> Grid<u8> {
    let nested: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    Grid::from_nested(&nested)
}

fn part1(grid: &Grid<u8>) -> usize {
    two_dimension_iter(grid.width, grid.height)
        .map(|pos| match grid.get(pos) {
            Some(b'X') => ALL_EIGHT_DIRECTIONS
                .iter()
                .filter(|&&dir| {
                    matches!(grid.get(pos + dir), Some(b'M'))
                        && matches!(grid.get(pos + dir * 2), Some(b'A'))
                        && matches!(grid.get(pos + dir * 3), Some(b'S'))
                })
                .count(),
            _ => 0,
        })
        .sum()
}

fn matches_x_mas(grid: &Grid<u8>, pos: Vec2) -> bool {
    let lr_diagonal = &[
        grid[pos],
        grid[pos + Vec2::new(1, 1)],
        grid[pos + Vec2::new(2, 2)],
    ];
    let rl_diagonal = &[
        grid[pos + Vec2::new(2, 0)],
        grid[pos + Vec2::new(1, 1)],
        grid[pos + Vec2::new(0, 2)],
    ];
    let permutations = [b"MAS", b"SAM"];
    permutations.contains(&lr_diagonal) && permutations.contains(&rl_diagonal)
}

fn part2(grid: &Grid<u8>) -> usize {
    two_dimension_iter(grid.width - 2, grid.height - 2)
        .filter(|pos| matches_x_mas(grid, *pos))
        .count()
}

fn main() {
    let test_grid = parse(&TEST_INPUT);
    let grid = parse(&INPUT);

    assert_eq!(part1(&test_grid), 18);
    assert_eq!(part1(&grid), 2543);
    assert_eq!(part2(&test_grid), 9);
    assert_eq!(part2(&grid), 1930);
}
