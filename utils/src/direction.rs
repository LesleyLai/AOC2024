use crate::Vec2;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Direction4 {
    Up,
    Down,
    Left,
    Right,
}

impl Direction4 {
    pub fn index(self) -> usize {
        self as usize
    }

    pub const fn turn_left(self) -> Direction4 {
        use Direction4::*;
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }
    pub const fn turn_right(self) -> Direction4 {
        use Direction4::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    pub const fn all_directions() -> &'static [Direction4; 4] {
        &[
            Direction4::Up,
            Direction4::Right,
            Direction4::Down,
            Direction4::Left,
        ]
    }

    // Gets a unique bit for each direction (can be used as a bitmask)
    pub const fn bit(self) -> u8 {
        match self {
            Direction4::Up => 1,
            Direction4::Down => 2,
            Direction4::Left => 4,
            Direction4::Right => 8,
        }
    }

    // Returns an ascii character that can visualize the character
    pub const fn ascii(self) -> u8 {
        match self {
            Direction4::Up => b'^',
            Direction4::Down => b'v',
            Direction4::Left => b'<',
            Direction4::Right => b'>',
        }
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

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Direction8 {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction8 {
    pub const fn all_directions() -> [Direction8; 8] {
        [
            Direction8::Up,
            Direction8::Down,
            Direction8::Left,
            Direction8::Right,
            Direction8::UpLeft,
            Direction8::UpRight,
            Direction8::DownLeft,
            Direction8::DownRight,
        ]
    }
}

impl From<Direction8> for Vec2 {
    fn from(dir: Direction8) -> Self {
        use Direction8::*;
        match dir {
            Up => Vec2::new(0, -1),
            Down => Vec2::new(0, 1),
            Left => Vec2::new(-1, 0),
            Right => Vec2::new(1, 0),
            UpLeft => Vec2::new(-1, -1),
            UpRight => Vec2::new(1, -1),
            DownLeft => Vec2::new(-1, 1),
            DownRight => Vec2::new(1, 1),
        }
    }
}
