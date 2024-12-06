use utils::{two_dimension_iter, Direction4, Grid, Vec2};

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

fn turn_right_until_not_facing_wall(
    grid: &Grid<u8>,
    pos: Vec2,
    mut direction: Direction4,
) -> Direction4 {
    while grid.get(pos + Vec2::from(direction)) == Some(&b'#') {
        direction = direction.turn_right();
    }
    direction
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::from_text(&input);

    let mut current = grid.find(&b'^').unwrap();
    grid[current] = b'x';

    let mut direction = Direction4::Up;
    while grid.get(current).is_some() {
        direction = turn_right_until_not_facing_wall(&grid, current, direction);
        grid[current] = b'X';
        current = current + Vec2::from(direction);
    }

    grid.iter().filter(|&&c| c == b'X').count()
}

fn in_a_loop(grid: &mut Grid<u8>, start: Vec2) -> bool {
    let mut current = start;
    let mut direction = Direction4::Up;

    // Use 0 rather than '.' to represent empty spots
    grid[start] = 0;
    for coord in two_dimension_iter(grid.width, grid.height) {
        if grid.get(coord) == Some(&b'.') {
            grid[coord] = 0;
        }
    }

    let direction_to_bit = |direction: Direction4| match direction {
        Direction4::Up => 0b1,
        Direction4::Right => 0b10,
        Direction4::Down => 0b100,
        Direction4::Left => 0b1000,
    };

    while !grid.is_out_of_bound(current) {
        direction = turn_right_until_not_facing_wall(&grid, current, direction);

        if Direction4::all_directions()
            .iter()
            .any(|&dir| grid[current] & direction_to_bit(dir) != 0 && direction == dir)
        {
            return true;
        }

        grid[current] += direction_to_bit(direction);

        current = current + Vec2::from(direction);
    }

    false
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_text(&input);
    let start = grid.find(&b'^').unwrap();

    grid.enumerate()
        .filter_map(|(coord, &content)| (content == b'.').then(|| coord))
        .filter(|&coord| {
            let mut modified_grid = grid.clone();
            modified_grid[coord] = b'#';
            in_a_loop(&mut modified_grid, start)
        })
        .count()
}

fn main() {
    assert_eq!(part1(&TEST_INPUT), 41);
    assert_eq!(part1(&INPUT), 4776);

    assert_eq!(part2(&TEST_INPUT), 6);
    assert_eq!(part2(&INPUT), 1586);
}
