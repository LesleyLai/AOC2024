use utils::{Direction4, Grid, Vec2};

const TEST_INPUT: &str = "029A
980A
179A
456A
379A";

const DIR_KEYPAD: [u8; 6] = [0, b'^', b'A', b'<', b'V', b'>'];

enum DirKeypadAction {
    Move(Direction4),
}

// fn shorted_path_move(result: &[u8]) -> Box<[u8]> {
//     let mut dir_keypad = [0, b'^', b'A', b'<', b'V', b'>'];
//
//     let mut dir_keypad: Grid<u8> = Grid::new(3, 2);
//     dir_keypad[(1, 0)] = b'^';
//
//     let current_position = Vec2::new(2, 0);
// }

fn main() {}
