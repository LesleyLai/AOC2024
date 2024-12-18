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

use utils::{two_dimension_iter, Direction8, Grid, Vec2};

fn part1(grid: &Grid<u8>) -> usize {
    grid.bound()
        .iter()
        .map(|pos| match grid.get(pos) {
            Some(b'X') => Direction8::all_directions()
                .iter()
                .filter(|&&dir| {
                    matches!(grid.get(pos + Vec2::from(dir)), Some(b'M'))
                        && matches!(grid.get(pos + Vec2::from(dir) * 2), Some(b'A'))
                        && matches!(grid.get(pos + Vec2::from(dir) * 3), Some(b'S'))
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
    let test_grid = Grid::from_text(&TEST_INPUT);
    let grid = Grid::from_text(&INPUT);

    assert_eq!(part1(&test_grid), 18);
    assert_eq!(part1(&grid), 2543);
    assert_eq!(part2(&test_grid), 9);
    assert_eq!(part2(&grid), 1930);
}
