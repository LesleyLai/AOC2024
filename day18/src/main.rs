use std::collections::VecDeque;
use utils::{Direction4, Grid, Vec2};

const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> impl Iterator<Item = Vec2> + '_ {
    input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        Vec2::new(x.parse().unwrap(), y.parse().unwrap())
    })
}

fn bfs(grid: &Grid<u8>, visited: &mut Grid<bool>) -> Option<usize> {
    let start = Vec2::new(0, 0);
    let end = Vec2::new(grid.width - 1, grid.height - 1);

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((coord, cost)) = queue.pop_front() {
        if coord == end {
            return Some(cost);
        }

        if visited[coord] {
            continue;
        }
        visited[coord] = true;

        for &dir in Direction4::all_directions() {
            let next = coord + Vec2::from(dir);
            if grid.get(next) == Some(&b'.') && !visited[next] {
                queue.push_back((next, cost + 1));
            }
        }
    }

    None
}

fn part1(input: &str, grid_size: isize, byte_count: usize) -> usize {
    let mut grid = Grid::with_value(grid_size, grid_size, b'.');

    for coord in parse_input(input).take(byte_count) {
        grid[coord] = b'#';
    }

    let mut visited = Grid::with_same_shape_as(&grid);
    bfs(&grid, &mut visited).unwrap()
}

fn part2(input: &str, grid_size: isize) -> Vec2 {
    let mut grid = Grid::with_value(grid_size, grid_size, b'.');
    let mut visited = Grid::with_same_shape_as(&grid);

    for coord in parse_input(input) {
        grid[coord] = b'#';

        visited.fill(false);
        if bfs(&grid, &mut visited).is_none() {
            return coord;
        }
    }
    unreachable!()
}

fn main() {
    assert_eq!(part1(TEST_INPUT, 7, 12), 22);
    assert_eq!(part1(INPUT, 71, 1024), 330);

    assert_eq!(part2(TEST_INPUT, 7), Vec2::new(6, 1));
    assert_eq!(part2(INPUT, 71), Vec2::new(10, 38));
}
