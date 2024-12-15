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

fn print_grid(grid: &Grid<u8>) {
    for row in grid.rows() {
        for &e in row {
            if e == 0 {
                print!(".");
            } else {
                print!("{}", e);
            }
        }
        println!();
    }
}

fn parse_vec2(str: &str) -> Vec2 {
    let (x, y) = str.split_once("=").unwrap().1.split_once(",").unwrap();
    Vec2::new(x.parse().unwrap(), y.parse().unwrap())
}

fn parse_line(line: &str) -> (Vec2, Vec2) {
    let (position, velocity) = line.split_once(" ").unwrap();
    (parse_vec2(position), parse_vec2(velocity))
}

fn part1(input: &str, width: isize, height: isize) -> isize {
    let (mut upper_left, mut lower_left, mut upper_right, mut lower_right) = (0, 0, 0, 0);

    for (mut position, velocity) in input.lines().map(parse_line) {
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

    // Wrap around if out of bound
    position.x = position.x.rem_euclid(width);
    position.y = position.y.rem_euclid(height);

    position
}

fn part2(input: &str, width: isize, height: isize) -> isize {
    let (mut positions, velocities): (Vec<_>, Vec<_>) = input.lines().map(parse_line).unzip();

    let mut grid: Grid<u8> = Grid::new(width, height);
    for i in successors(Some(1), |i| Some(i + 1)) {
        for (position, &velocity) in positions.iter_mut().zip(velocities.iter()) {
            *position = advance(*position, velocity, width, height);
            grid[*position] += 1;
        }

        let has_repetition = grid.iter().find(|&&v| v > 1).is_some();
        if !has_repetition {
            print_grid(&grid);
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
