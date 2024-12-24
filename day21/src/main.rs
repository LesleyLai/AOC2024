extern crate core;

use std::time::Instant;
use utils::{next_permutation, Direction4, Grid, Vec2};

const TEST_INPUT: &str = "029A
980A
179A
456A
379A";

const INPUT: &str = "480A
965A
140A
341A
285A";

fn is_valid_move(grid: &Grid<u8>, mut start: Vec2, moves: &[Direction4]) -> bool {
    for mov in moves {
        start += Vec2::from(*mov);
        if grid.get(start).is_none_or(|e| e == &b'#') {
            return false;
        }
    }
    true
}

fn find_all_paths(grid: &Grid<u8>, current_pos: Vec2, next_pos: Vec2) -> Vec<Vec<u8>> {
    if current_pos == next_pos {
        return vec![vec![b'A']];
    }

    assert_ne!(current_pos, next_pos);

    let diff = next_pos - current_pos;
    let mut moves = Vec::with_capacity(diff.x.abs() as usize + diff.y.abs() as usize);
    if diff.x < 0 {
        moves.extend(std::iter::repeat_n(Direction4::Left, diff.x.abs() as usize));
    }
    if diff.x > 0 {
        moves.extend(std::iter::repeat_n(Direction4::Right, diff.x as usize));
    }
    if diff.y < 0 {
        moves.extend(std::iter::repeat_n(Direction4::Up, diff.y.abs() as usize));
    }
    if diff.y > 0 {
        moves.extend(std::iter::repeat_n(Direction4::Down, diff.y as usize));
    }

    let moves = moves;
    let mut permutation = moves.clone();
    let mut permutations = vec![];

    loop {
        if is_valid_move(grid, current_pos, &permutation) {
            let mut result: Vec<u8> = permutation.iter().map(|d| d.ascii()).collect();
            result.push(b'A');
            permutations.push(result);
        }

        next_permutation(&mut permutation);

        if permutation == moves {
            break;
        }
    }

    permutations
}

fn possible_actions(grid: &Grid<u8>, input_actions: &[u8]) -> Vec<Vec<u8>> {
    let mut actions = vec![];

    let mut pos = grid.find(&b'A').unwrap();
    for action in input_actions {
        let next_pos = grid.find(&action).unwrap();
        let paths = find_all_paths(&grid, pos, next_pos);

        pos = next_pos;

        if actions.is_empty() {
            actions = paths;
        } else {
            let old_actions = actions;
            actions = vec![];
            for old_action in &old_actions {
                for path in &paths {
                    let mut new_action = old_action.clone();
                    new_action.append(&mut path.clone());
                    actions.push(new_action);
                }
            }
        }
    }

    actions
}

fn shortest_sequence(
    keypad_grid: &Grid<u8>,
    directional_grid: &Grid<u8>,
    input_actions: &[u8],
) -> usize {
    let actions = possible_actions(&keypad_grid, input_actions);

    let mut actions2 = vec![];
    for action in &actions {
        actions2.extend(possible_actions(&directional_grid, &action));
    }

    let mut actions3 = vec![];
    for action in &actions2 {
        actions3.extend(possible_actions(&directional_grid, &action));
    }

    actions3
        .iter()
        .min_by_key(|action| action.len())
        .unwrap()
        .len()
}

fn part1(input: &str) -> usize {
    let now = Instant::now();

    let keypad_grid = Grid::from_text("789\n456\n123\n#0A");
    let directional_grid = Grid::from_text("#^A\n<v>");

    let mut result = 0;
    for line in input.lines() {
        let num: String = line.chars().filter(|b| b.is_ascii_digit()).collect();
        let num: usize = num.parse().unwrap();
        result += num * shortest_sequence(&keypad_grid, &directional_grid, line.as_bytes());
    }

    println!("Part 1: {}s", now.elapsed().as_secs_f64());

    result
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 126384);
    assert_eq!(part1(INPUT), 152942);
}
