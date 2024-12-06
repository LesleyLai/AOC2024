use crate::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bound {
    pub base: Vec2,
    pub size: Vec2,
}

impl Bound {
    pub fn new(base: Vec2, size: Vec2) -> Self {
        Self { base, size }
    }

    // Create a 2-dimensional iterator from base to (base.x + size.x - 1, base.y + size.y - 1)
    pub fn iter(&self) -> impl Iterator<Item = Vec2> + use<'_> {
        (self.base.y..self.base.y + self.size.y).flat_map(move |y| {
            (self.base.x..self.base.x + self.size.x).map(move |x| Vec2::new(x, y))
        })
    }
}
