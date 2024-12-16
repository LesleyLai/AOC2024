use utils::{Direction4, Grid, Vec2};

const TEST_INPUT1: &str = "AAAA
BBCD
BBCC
EEEC";

const TEST_INPUT2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const TEST_INPUT3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

const TEST_INPUT4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

const TEST_INPUT5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

const INPUT: &str = include_str!("./input.txt");

// returns (area, parameter)
fn flood_fill_calc_parameter(
    grid: &Grid<u8>,
    filled_grid: &mut Grid<isize>,
    start: Vec2,
    value: isize,
) -> (usize, usize) {
    let mut parameter = 0;
    let mut area = 1;

    filled_grid[start] = value;
    for dir in Direction4::all_directions() {
        let next = start + Vec2::from(dir);
        if grid.get(next) == Some(&grid[start]) && filled_grid[next] < 0 {
            let (next_area, next_parameter) =
                flood_fill_calc_parameter(grid, filled_grid, next, value);
            area += next_area;
            parameter += next_parameter;
        } else if filled_grid.get(next).is_none_or(|&x| x != value) {
            parameter += 1;
        }
    }

    (area, parameter)
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_text(input);

    let mut filled_grid: Grid<isize> = Grid::with_value(grid.width, grid.height, -1);

    let mut region_id = 0;
    let mut result = 0;
    for coord in grid.bound().iter() {
        if filled_grid[coord] < 0 {
            let (area, parameter) =
                flood_fill_calc_parameter(&grid, &mut filled_grid, coord, region_id);
            result += area * parameter;
            region_id += 1;
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_text(input);

    let mut filled_grid: Grid<isize> = Grid::with_value(grid.width, grid.height, -1);

    let mut areas = vec![];
    let mut regions = vec![]; // start point of each region
    for (coord, &_) in grid.enumerate() {
        if filled_grid[coord] < 0 {
            let (area, _) = flood_fill_calc_parameter(
                &grid,
                &mut filled_grid,
                coord,
                isize::try_from(regions.len()).unwrap(),
            );
            areas.push(area);
            regions.push(coord);
        }
    }

    let mut sides_grid: Grid<u8> = Grid::new(grid.width, grid.height);
    for (coord, &region_id) in filled_grid.enumerate() {
        for dir in Direction4::all_directions() {
            let next = coord + Vec2::from(dir);
            if filled_grid.get(next) != Some(&region_id) {
                sides_grid[coord] += dir.bit();
            }
        }
    }

    let mut result = 0;
    // for each region
    for (i, &start) in regions.iter().enumerate() {
        let region_id = filled_grid[start];

        let mut up_wall_count = 0;
        let mut down_wall_count = 0;
        {
            for y in 0..grid.height {
                let mut in_up_wall = false;
                let mut in_down_wall = false;
                for x in 0..grid.width {
                    if filled_grid[(x, y)] == region_id {
                        let sides_bits = sides_grid[(x, y)];

                        let current_in_up_wall = (sides_bits & Direction4::Up.bit()) != 0;
                        if !in_up_wall && current_in_up_wall {
                            up_wall_count += 1;
                        }
                        in_up_wall = current_in_up_wall;

                        let current_in_down_wall = (sides_bits & Direction4::Down.bit()) != 0;
                        if !in_down_wall && current_in_down_wall {
                            down_wall_count += 1;
                        }
                        in_down_wall = current_in_down_wall;
                    } else {
                        in_up_wall = false;
                        in_down_wall = false;
                    }
                }
            }
        }

        let mut left_wall_count = 0;
        let mut right_wall_count = 0;

        for x in 0..grid.width {
            let mut in_left_wall = false;
            let mut in_right_wall = false;
            for y in 0..grid.height {
                if filled_grid[(x, y)] == region_id {
                    let sides_bits = sides_grid[(x, y)];

                    let current_in_left_wall = (sides_bits & Direction4::Left.bit()) != 0;
                    if !in_left_wall && current_in_left_wall {
                        left_wall_count += 1;
                    }
                    in_left_wall = current_in_left_wall;

                    let current_in_right_wall = (sides_bits & Direction4::Right.bit()) != 0;

                    if !in_right_wall && current_in_right_wall {
                        right_wall_count += 1;
                    }
                    in_right_wall = current_in_right_wall;
                } else {
                    in_left_wall = false;
                    in_right_wall = false;
                }
            }
        }

        let sides = up_wall_count + down_wall_count + left_wall_count + right_wall_count;

        result += sides * areas[i];
    }

    result
}

fn main() {
    assert_eq!(part1(TEST_INPUT1), 140);
    assert_eq!(part1(TEST_INPUT2), 772);
    assert_eq!(part1(TEST_INPUT3), 1930);

    assert_eq!(part1(INPUT), 1489582);

    assert_eq!(part2(TEST_INPUT1), 80);
    assert_eq!(part2(TEST_INPUT2), 436);

    assert_eq!(part2(TEST_INPUT4), 236);
    assert_eq!(part2(TEST_INPUT5), 368);

    assert_eq!(part2(INPUT), 914966);
}
