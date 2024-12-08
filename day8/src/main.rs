use std::collections::HashMap;
use utils::{Grid, Vec2};

const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

const INPUT: &str = include_str!("./input.txt");

struct Input {
    width: isize,
    height: isize,
    frequency_map: HashMap<u8, Vec<Vec2>>,
}

fn parse_input(input: &str) -> Input {
    let grid = Grid::from_text(&input);

    let mut frequency_map = HashMap::new();
    for (coord, c) in grid.enumerate() {
        if *c != b'.' {
            frequency_map.entry(*c).or_insert(vec![]).push(coord);
        }
    }

    Input {
        width: grid.width,
        height: grid.height,
        frequency_map,
    }
}

fn part1(input: &str) -> usize {
    let Input {
        width,
        height,
        frequency_map,
    } = parse_input(input);

    let mut antinode_grid = Grid::new(width, height);
    for (_, coords) in &frequency_map {
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                let (a, b) = (coords[i], coords[j]);
                assert_ne!(a, b);

                if let Some(slot) = antinode_grid.get_mut(a + (a - b)) {
                    *slot = true;
                }

                if let Some(slot) = antinode_grid.get_mut(b + (b - a)) {
                    *slot = true;
                }
            }
        }
    }

    antinode_grid.iter().filter(|&&b| b).count()
}

fn part2(input: &str) -> usize {
    let Input {
        width,
        height,
        frequency_map,
    } = parse_input(input);

    let mut antinode_grid = Grid::new(width, height);

    let mut walk_and_fill = |start, dir| {
        let mut current = start;
        while let Some(slot) = antinode_grid.get_mut(current) {
            *slot = true;
            current += dir;
        }
    };

    for (_, coords) in &frequency_map {
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                let (a, b) = (coords[i], coords[j]);
                let dist = b - a;
                let dir = Vec2::new(dist.x, dist.y);

                walk_and_fill(a, dir);
                walk_and_fill(a, -dir);
            }
        }
    }

    antinode_grid.iter().filter(|&&b| b).count()
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 14);
    assert_eq!(part1(INPUT), 259);

    assert_eq!(part2(TEST_INPUT), 34);
    assert_eq!(part2(INPUT), 927);
}
