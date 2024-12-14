use std::iter::successors;
use utils::{Grid, Vec2};

const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

const INPUT: &str = include_str!("./input.txt");

// fn print_grid(grid: &Grid<isize>) {
//     for row in grid.rows() {
//         for &e in row {
//             if e == 0 {
//                 print!(".");
//             } else {
//                 print!("{}", e);
//             }
//         }
//         println!();
//     }
// }

fn parse_vec2(str: &str) -> Vec2 {
    let (x, y) = str.split_once("=").unwrap().1.split_once(",").unwrap();
    Vec2::new(x.parse().unwrap(), y.parse().unwrap())
}

fn part1(input: &str, width: isize, height: isize) -> isize {
    let mut upper_left = 0;
    let mut lower_left = 0;
    let mut upper_right = 0;
    let mut lower_right = 0;

    for line in input.lines() {
        let (position, velocity) = line.split_once(" ").unwrap();
        let mut position = parse_vec2(position);
        let velocity = parse_vec2(velocity);

        for _ in 0..100 {
            position = advance(position, velocity, width, height);
        }

        if position.y < height / 2 {
            if position.x < width / 2 {
                upper_left += 1;
            } else if position.x > width / 2 {
                upper_right += 1;
            }
        } else if position.y > height / 2 {
            if position.x < width / 2 {
                lower_left += 1;
            } else if position.x > width / 2 {
                lower_right += 1;
            }
        }
    }

    upper_left * upper_right * lower_left * lower_right
}

fn advance(mut position: Vec2, velocity: Vec2, width: isize, height: isize) -> Vec2 {
    position += velocity;

    while position.x >= width {
        position.x -= width;
    }
    while position.x < 0 {
        position.x += width;
    }
    while position.y >= height {
        position.y -= height;
    }
    while position.y < 0 {
        position.y += height;
    }

    position
}

fn part2(input: &str, width: isize, height: isize) -> isize {
    let mut grid: Grid<u8> = Grid::new(width, height);

    let mut positions = vec![];
    let mut velocities = vec![];

    for line in input.lines() {
        let (position, velocity) = line.split_once(" ").unwrap();
        positions.push(parse_vec2(position));
        velocities.push(parse_vec2(velocity));
    }

    for i in successors(Some(1), |i| Some(i + 1)) {
        for (position, &velocity) in positions.iter_mut().zip(velocities.iter()) {
            *position = advance(*position, velocity, width, height);
            grid[*position] += 1;
        }

        let has_repetition = grid.iter().find(|&&v| v > 1).is_some();
        if !has_repetition {
            return i;
        }

        // reset grid
        grid.fill(0);
    }

    unreachable!()
}

fn main() {
    assert_eq!(part1(TEST_INPUT, 11, 7), 12);
    assert_eq!(part1(INPUT, 101, 103), 214400550);

    assert_eq!(part2(INPUT, 101, 103), 8149);
}
