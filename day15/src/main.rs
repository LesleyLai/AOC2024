use utils::{Direction4, Grid, Vec2};

const TEST_INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

const TEST_INPUT2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

const TEST_INPUT3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

const INPUT: &str = include_str!("./input.txt");

fn direction_from_char(mov: u8) -> Direction4 {
    match mov {
        b'^' => Direction4::Up,
        b'v' => Direction4::Down,
        b'<' => Direction4::Left,
        b'>' => Direction4::Right,
        _ => unreachable!(),
    }
}

// fn print_map(map: &Grid<u8>, robot_pos: Vec2) {
//     for y in 0..map.height {
//         for x in 0..map.width {
//             let coord = Vec2::new(x, y);
//             if coord == robot_pos {
//                 print!("{}", "@")
//             } else {
//                 print!("{}", map[coord] as char);
//             }
//         }
//         println!();
//     }
//     println!();
// }

fn parse_moves(s: &str) -> Vec<u8> {
    s.lines()
        .flat_map(|line| line.bytes().into_iter())
        .collect()
}

fn part1(input: &str) -> isize {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut map = Grid::from_text(map);
    let moves = parse_moves(moves);

    let mut current = map.find(&b'@').unwrap();
    map[current] = b'.';

    for mov in moves {
        let dir = direction_from_char(mov);
        let next = current + Vec2::from(dir);

        if map[next] == b'.' {
            current = next;
        } else if map[next] == b'O' {
            let mut cursor = next;
            let mut box_count = 0;
            while map[cursor] == b'O' {
                // Move forward until hit a none box
                cursor += Vec2::from(dir);
                box_count += 1;
            }
            if map[cursor] == b'.' {
                // Can push box
                map[next] = b'.';
                current = next;

                let mut cursor = next + Vec2::from(dir);
                for _ in 0..box_count {
                    map[cursor] = b'O';
                    cursor += Vec2::from(dir);
                }
            }
        }
    }

    map.enumerate()
        .filter(|(_, &c)| c == b'O')
        .map(|(coord, _)| coord.x + 100 * coord.y)
        .sum()
}

// return false if can't move box
// Also accumulate boxes to move in the process
fn calculate_move(
    boxes_to_move: &mut Vec<Vec2>,
    map: &Grid<u8>,
    pos: Vec2,
    dir: Direction4,
) -> bool {
    match map[pos] {
        b'.' => true,
        b'#' => false,
        b'[' | b']' => {
            let box_location = if map[pos] == b'[' {
                pos
            } else {
                pos + Vec2::new(-1, 0)
            };

            if dir == Direction4::Up || dir == Direction4::Down {
                if calculate_move(boxes_to_move, map, box_location + Vec2::from(dir), dir)
                    && calculate_move(
                        boxes_to_move,
                        map,
                        box_location + Vec2::new(1, 0) + Vec2::from(dir),
                        dir,
                    )
                {
                    boxes_to_move.push(box_location);
                    true
                } else {
                    false
                }
            } else if dir == Direction4::Left {
                if calculate_move(boxes_to_move, map, box_location + Vec2::from(dir), dir) {
                    boxes_to_move.push(box_location);
                    true
                } else {
                    false
                }
            } else {
                if calculate_move(
                    boxes_to_move,
                    map,
                    box_location + Vec2::new(1, 0) + Vec2::from(dir),
                    dir,
                ) {
                    boxes_to_move.push(box_location);
                    true
                } else {
                    false
                }
            }
        }
        _ => unreachable!(),
    }
}

fn move_boxes(map: &mut Grid<u8>, dir: Direction4, boxes_to_move: &[Vec2]) {
    for &b in boxes_to_move {
        map[b] = b'.';
        map[b + Vec2::new(1, 0)] = b'.';
    }

    for &b in boxes_to_move {
        map[b + Vec2::from(dir)] = b'[';
        map[b + Vec2::new(1, 0) + Vec2::from(dir)] = b']';
    }
}

fn part2(input: &str) -> isize {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let nested: Vec<Vec<u8>> = map
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .flat_map(|&c| match c {
                    b'#' => b"##",
                    b'O' => b"[]",
                    b'.' => b"..",
                    b'@' => b"@.",
                    _ => unreachable!(),
                })
                .map(|c| *c)
                .collect()
        })
        .collect();
    let mut map = Grid::from_nested(&nested);
    let moves = parse_moves(moves);

    let mut current = map.find(&b'@').unwrap();
    map[current] = b'.';

    //print_map(&map, current);

    for mov in moves {
        let dir = direction_from_char(mov);
        let next = current + Vec2::from(dir);

        if map[next] == b'.' {
            current = next;
        } else if map[next] == b'[' || map[next] == b']' {
            let mut boxes_to_move = Vec::new();
            let can_move = calculate_move(&mut boxes_to_move, &map, next, dir);
            if can_move {
                move_boxes(&mut map, dir, &boxes_to_move);
                current = next;
            }
        }
    }

    map.enumerate()
        .filter(|(_, &c)| c == b'[')
        .map(|(coord, _)| coord.x + 100 * coord.y)
        .sum()
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 2028);
    assert_eq!(part1(TEST_INPUT2), 10092);
    assert_eq!(part1(INPUT), 1438161);

    assert_eq!(part2(TEST_INPUT2), 9021);

    assert_eq!(part2(TEST_INPUT3), 618);
    assert_eq!(part2(INPUT), 1437981);
}
