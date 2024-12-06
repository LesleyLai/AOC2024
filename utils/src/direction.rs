use crate::Vec2;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction4 {
    Up,
    Down,
    Left,
    Right,
}

impl Direction4 {
    pub const fn turn_right(self) -> Direction4 {
        use Direction4::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    pub const fn all_directions() -> [Direction4; 4] {
        [
            Direction4::Up,
            Direction4::Right,
            Direction4::Down,
            Direction4::Left,
        ]
    }
}

impl From<Direction4> for Vec2 {
    fn from(dir: Direction4) -> Self {
        use Direction4::*;
        match dir {
            Up => Vec2::new(0, -1),
            Down => Vec2::new(0, 1),
            Left => Vec2::new(-1, 0),
            Right => Vec2::new(1, 0),
        }
    }
}
