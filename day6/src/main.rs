use utils::{Grid, Vec2};

const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
const INPUT: &str = include_str!("./input.txt");

fn parse(input: &str) -> Grid<u8> {
    let nested: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    Grid::from_nested(&nested)
}

// fn print_grid(grid: &Grid<u8>) {
//     for y in 0..grid.height {
//         for x in 0..grid.width {
//             print!("{}", grid[Vec2::new(x, y)] as char);
//         }
//         println!();
//     }
// }

fn turn_right(direction: Vec2) -> Vec2 {
    match direction {
        Vec2 { x: 0, y: -1 } => Vec2::new(1, 0),
        Vec2 { x: 1, y: 0 } => Vec2::new(0, 1),
        Vec2 { x: 0, y: 1 } => Vec2::new(-1, 0),
        Vec2 { x: -1, y: 0 } => Vec2::new(0, -1),
        _ => panic!("Invalid direction"),
    }
}

fn part1(input: &str) -> usize {
    let mut grid = parse(&input);

    let mut current = Vec2::new(0, 0);
    'outer: for y in 0..grid.height {
        for x in 0..grid.width {
            match grid[Vec2::new(x, y)] {
                b'^' => {
                    current = Vec2::new(x, y);
                    break 'outer;
                }
                _ => {}
            }
        }
    }
    grid[current] = b'x';

    let mut direction = Vec2::new(0, -1);
    while grid.get(current).is_some() {
        while grid.get(current + direction) == Some(&b'#') {
            direction = turn_right(direction);
        }
        grid[current] = b'X';

        current = current + direction;
    }

    let mut distinct_positions = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid[Vec2::new(x, y)] == b'X' {
                distinct_positions += 1;
            }
        }
    }

    distinct_positions
}

fn in_a_loop(grid: &Grid<u8>, start: Vec2) -> bool {
    let mut direction = Vec2::new(0, -1);

    let mut current = start;

    let mut trace_grid: Grid<u8> = Grid::new(grid.width, grid.height);

    while !grid.is_out_of_bound(current) {
        while grid.get(current + direction) == Some(&b'#') {
            direction = turn_right(direction);
        }

        // Repeat
        if trace_grid[current] & 0b1 != 0 && direction == Vec2::new(0, -1) {
            return true;
        }
        if trace_grid[current] & 0b10 != 0 && direction == Vec2::new(1, 0) {
            return true;
        }
        if trace_grid[current] & 0b100 != 0 && direction == Vec2::new(0, 1) {
            return true;
        }
        if trace_grid[current] & 0b1000 != 0 && direction == Vec2::new(-1, 0) {
            return true;
        }

        trace_grid[current] += match direction {
            Vec2 { x: 0, y: -1 } => 0b1,
            Vec2 { x: 1, y: 0 } => 0b10,
            Vec2 { x: 0, y: 1 } => 0b100,
            Vec2 { x: -1, y: 0 } => 0b1000,
            _ => panic!("Invalid direction"),
        };

        current = current + direction;
    }

    false
}

fn part2(input: &str) -> usize {
    let grid = parse(&input);

    let mut start = Vec2::new(0, 0);
    'outer: for y in 0..grid.height {
        for x in 0..grid.width {
            match grid[Vec2::new(x, y)] {
                b'^' => {
                    start = Vec2::new(x, y);
                    break 'outer;
                }
                _ => {}
            }
        }
    }

    let start = start;

    let mut position_count = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            let location = Vec2::new(x, y);
            if grid[location] == b'.' {
                let mut modified_grid = grid.clone();
                modified_grid[location] = b'#';
                if in_a_loop(&modified_grid, start) {
                    position_count += 1;
                }
            }
        }
    }

    position_count
}

fn main() {
    assert_eq!(part1(&TEST_INPUT), 41);
    assert_eq!(part1(&INPUT), 4776);

    assert_eq!(part2(&TEST_INPUT), 6);
    assert_eq!(part2(&INPUT), 1586);
}
