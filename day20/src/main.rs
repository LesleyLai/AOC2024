use std::collections::VecDeque;
use utils::{Direction4, Grid, Vec2};

#[allow(dead_code)]
const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> (Grid<u8>, Vec2, Vec2) {
    let mut grid = Grid::from_text(input);
    let start = grid.find(&b'S').unwrap();
    let end = grid.find(&b'E').unwrap();
    grid[start] = b'.';
    grid[end] = b'.';
    (grid, start, end)
}

fn compute_cost_from(grid: &Grid<u8>, start: Vec2) -> Grid<isize> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut cost_grid = Grid::with_value(grid.width, grid.height, isize::MAX);

    while let Some((coord, cost)) = queue.pop_front() {
        if cost_grid[coord] <= cost {
            continue;
        }
        cost_grid[coord] = cost;

        for &dir in Direction4::all_directions() {
            let next = coord + Vec2::from(dir);
            if grid.get(next) == Some(&b'.') {
                queue.push_back((next, cost + 1));
            }
        }
    }

    cost_grid
}

// Cheat candidates are in a diamond shape
fn cheat_candidates(cheat_seconds: isize) -> impl Iterator<Item = Vec2> {
    (-cheat_seconds..=cheat_seconds).flat_map(move |dy| {
        ((-cheat_seconds + dy.abs())..=(cheat_seconds - dy.abs())).map(move |dx| Vec2::new(dx, dy))
    })
}

fn solve(
    grid: &Grid<u8>,
    cost_from_start: &Grid<isize>,
    cost_to_end: &Grid<isize>,
    cheat_seconds: isize,
    min_cheat_save: isize,
) -> usize {
    let mut good_cheat_count = 0;

    let coords = grid
        .enumerate()
        .filter(|(_, &elem)| elem == b'.')
        .map(|(coord, _)| coord);

    for coord in coords {
        for cheat in cheat_candidates(cheat_seconds) {
            let after_cheat_coord = coord + Vec2::from(cheat);
            if grid.get(after_cheat_coord) != Some(&b'.') {
                continue;
            }
            let non_cheat_cost = cost_from_start[coord] + cost_to_end[coord];
            let cheat_cost = cheat.x.abs()
                + cheat.y.abs()
                + cost_from_start[coord]
                + cost_to_end[after_cheat_coord];

            if non_cheat_cost - cheat_cost >= min_cheat_save {
                good_cheat_count += 1;
            }
        }
    }

    good_cheat_count
}

fn main() {
    let (grid, start, end) = parse_input(INPUT);

    let cost_from_start = compute_cost_from(&grid, start);
    let cost_to_end = compute_cost_from(&grid, end);

    assert_eq!(solve(&grid, &cost_from_start, &cost_to_end, 2, 100), 1426);

    assert_eq!(
        solve(&grid, &cost_from_start, &cost_to_end, 20, 100),
        1000697
    );
}
