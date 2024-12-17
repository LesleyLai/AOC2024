use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use utils::{Direction4, Grid, Vec2};

const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const TEST_INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: isize,
    location: Vec2,
    direction: Direction4,
}

impl State {
    fn new(cost: isize, location: Vec2, direction: Direction4) -> Self {
        State {
            cost,
            location,
            direction,
        }
    }

    fn start(location: Vec2) -> Self {
        Self {
            cost: 0,
            location,
            direction: Direction4::Right,
        }
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Reverse(self.cost).partial_cmp(&Reverse(other.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.cost).cmp(&Reverse(other.cost))
    }
}

#[allow(dead_code)]
fn print_state(state: &State, grid: &Grid<u8>) {
    println!("Cost: {}", state.cost);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let coord = Vec2::new(x, y);
            if coord == state.location {
                print!("{}", state.direction.ascii() as char);
            } else {
                print!("{}", grid[coord] as char);
            }
        }
        println!();
    }
    println!();
}

fn neighbors(state: State, grid: &Grid<u8>) -> impl Iterator<Item = State> + use<'_> {
    use std::iter::once_with;

    let State {
        cost,
        location,
        direction,
    } = state;

    let get_neighbor = move |new_dir, new_cost| {
        let new_dest = location + Vec2::from(new_dir);
        (grid[new_dest] == b'.').then_some(State::new(new_cost, new_dest, new_dir))
    };

    let forward = once_with(move || get_neighbor(direction, cost + 1));
    let left = once_with(move || get_neighbor(direction.turn_left(), cost + 1001));
    let right = once_with(move || get_neighbor(direction.turn_right(), cost + 1001));

    forward.chain(left).chain(right).filter_map(|s| s)
}

struct WorldState {
    grid: Grid<u8>,
    start: Vec2,
    end: Vec2,
}

fn parse_input(input: &str) -> WorldState {
    let mut grid = Grid::from_text(input);

    let start = grid.find(&b'S').unwrap();
    let end = grid.find(&b'E').unwrap();

    grid[start] = b'.';
    grid[end] = b'.';

    WorldState { grid, start, end }
}

struct DijkstraResult {
    // Mapping from move-out direction to move-in direction
    in_from_out: Grid<OutToInMapping>,
    best_cost: isize,
}

fn dijkstra(world_state: &WorldState) -> DijkstraResult {
    let WorldState { start, end, grid } = world_state;

    let mut priority_queue: BinaryHeap<State> = BinaryHeap::new();
    priority_queue.push(State::start(*start));

    let mut in_from_out: Grid<OutToInMapping> = Grid::with_same_shape_as(&grid);

    // Conceptually a HashMap<(Vec2, Direction4), isize>
    let mut cost_map: Grid<[isize; 4]> = Grid::with_value(grid.width, grid.height, [isize::MAX; 4]);
    let get_end_cost = |cost_map: &Grid<[isize; 4]>| *cost_map[*end].iter().min().unwrap();

    while let Some(state) = priority_queue.pop() {
        let State {
            cost,
            location,
            direction,
        } = state;

        if cost > get_end_cost(&cost_map) {
            break;
        }

        let old_cost = &mut cost_map[location][direction.index()];
        if cost > *old_cost {
            continue;
        }
        *old_cost = cost;

        if location == *end {
            continue;
        }

        for neighbor in neighbors(state, &grid) {
            if neighbor.cost < cost_map[neighbor.location][neighbor.direction.index()] {
                in_from_out[location].insert(neighbor.direction, direction);
                priority_queue.push(neighbor);
            }
        }
    }

    DijkstraResult {
        in_from_out,
        best_cost: get_end_cost(&cost_map),
    }
}

fn part1(input: &str) -> isize {
    dijkstra(&parse_input(input)).best_cost
}

fn count_tiles_on_best_paths(
    world_state: &WorldState,
    in_from_out: &Grid<OutToInMapping>,
) -> usize {
    let WorldState { grid, start, end } = world_state;

    let mut on_best_paths: Grid<bool> = Grid::with_same_shape_as(grid);
    on_best_paths[*end] = true;

    let mut stack = vec![];
    for dir in Direction4::all_directions() {
        let previous = *end - Vec2::from(*dir);

        // If the previous location does go to the end
        if in_from_out[previous].has_value(*dir) {
            stack.push((previous, dir));
        }
    }

    while let Some((location, &going_out_dir)) = stack.pop() {
        if on_best_paths[location] {
            continue;
        }
        on_best_paths[location] = true;

        if location == *start {
            continue;
        }

        for going_in_dir in in_from_out[location].lookup(going_out_dir) {
            stack.push((location - Vec2::from(*going_in_dir), going_in_dir));
        }
    }

    on_best_paths.iter().filter(|&&b| b).count()
}

// Mapping from move-out direction to (multiple) move-in direction
// It is conceptually a HashMap<Direction4, HashSet<Direction4>>

#[derive(Clone, Default)]
struct OutToInMapping {
    data: [u8; 4],
}

impl OutToInMapping {
    fn insert(&mut self, out_dir: Direction4, in_dir: Direction4) {
        self.data[out_dir.index()] |= in_dir.bit();
    }

    fn has_value(&self, out_dir: Direction4) -> bool {
        self.data[out_dir.index()] != 0
    }

    fn lookup(&self, out_dir: Direction4) -> impl Iterator<Item = &Direction4> {
        Direction4::all_directions()
            .iter()
            .filter(move |&&in_dir| self.data[out_dir.index()] & in_dir.bit() != 0)
    }
}

fn part2(input: &str) -> usize {
    let world_state = parse_input(input);

    let in_from_out = dijkstra(&world_state).in_from_out;

    count_tiles_on_best_paths(&world_state, &in_from_out)
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 7036);
    assert_eq!(part1(TEST_INPUT2), 11048);
    assert_eq!(part1(INPUT), 127520);

    assert_eq!(part2(TEST_INPUT), 45);
    assert_eq!(part2(TEST_INPUT2), 64);
    assert_eq!(part2(INPUT), 565);
}
