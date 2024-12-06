use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<isize> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Create a 2-dimensional iterator of integer coordinates that iterates from (0, 0) to (width - 1, height - 1)
pub fn two_dimension_iter(width: isize, height: isize) -> impl Iterator<Item = Vec2> {
    (0..(height)).flat_map(move |y| (0..width).map(move |x| Vec2::new(x, y)))
}
