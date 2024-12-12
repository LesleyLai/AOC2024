#![allow(dead_code)]

use crate::vec2::Vec2;
use crate::{two_dimension_iter, Bound};
use std::mem::swap;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    data: Box<[T]>,
}

impl<T> Grid<T> {
    pub fn is_out_of_bound(self: &Self, pos: Vec2) -> bool {
        pos.x < 0 || pos.x >= self.width || pos.y < 0 || pos.y >= self.height
    }

    /// Flip rows
    pub fn flip_x(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width / 2 {
                self.data.swap(
                    (y * self.width + x) as usize,
                    ((y + 1) * self.width - x - 1) as usize,
                );
            }
        }
    }

    pub fn transpose(&mut self) {
        for y in 0..self.height {
            for x in (y + 1)..self.width {
                self.data
                    .swap((x * self.width + y) as usize, (y * self.width + x) as usize);
            }
        }
        swap(&mut self.width, &mut self.height);
    }

    pub fn get(&self, pos: impl Into<Vec2>) -> Option<&T> {
        let pos = pos.into();
        if self.is_out_of_bound(pos) {
            None
        } else {
            Some(&self.data[(pos.y * self.width + pos.x) as usize])
        }
    }

    pub fn get_mut(&mut self, pos: impl Into<Vec2>) -> Option<&mut T> {
        let pos = pos.into();
        if self.is_out_of_bound(pos) {
            None
        } else {
            Some(&mut self.data[(pos.y * self.width + pos.x) as usize])
        }
    }

    pub fn rows(self: &Self) -> GridRowIter<T> {
        GridRowIter { grid: self, y: 0 }
    }

    pub fn columns(self: &Self) -> GridColIter<T> {
        GridColIter { grid: self, x: 0 }
    }

    pub fn iter(self: &Self) -> GridIter<T> {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn enumerate(self: &Self) -> GridEnumerateIter<T> {
        GridEnumerateIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(self: &Self, elem: &T) -> Option<Vec2> {
        two_dimension_iter(self.width, self.height).find(|coord| self[*coord] == *elem)
    }
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(width: isize, height: isize) -> Grid<T> {
        Grid {
            width,
            height,
            data: vec![Default::default(); (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn with_value(width: isize, height: isize, value: T) -> Grid<T> {
        Grid {
            width,
            height,
            data: vec![value; (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn from_nested(input: &[Vec<T>]) -> Grid<T> {
        let width = input.first().unwrap().len();
        let height = input.len();

        let mut data = vec![Default::default(); width * height].into_boxed_slice();
        for (y, row) in input.iter().enumerate() {
            let begin = y * width;
            data[begin..begin + width].clone_from_slice(row);
        }

        Grid {
            width: width as isize,
            height: height as isize,
            data,
        }
    }

    pub fn bound(&self) -> Bound {
        Bound::new(Vec2::new(0, 0), Vec2::new(self.width, self.height))
    }
}

impl Grid<u8> {
    pub fn from_text(text: &str) -> Grid<u8> {
        let nested: Vec<_> = text.lines().map(|line| line.as_bytes().to_vec()).collect();
        Grid::from_nested(&nested)
    }
}

impl<T, Coord: Into<Vec2>> std::ops::Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, coord: Coord) -> &Self::Output {
        self.get(coord).unwrap()
    }
}

impl<T, Coord: Into<Vec2>> std::ops::IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        self.get_mut(coord).unwrap()
    }
}

pub struct GridRowIter<'a, T> {
    grid: &'a Grid<T>,
    y: isize,
}

impl<'a, T> Iterator for GridRowIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height {
            None
        } else {
            let begin = (self.y * self.grid.width) as usize;
            let end = begin + self.grid.width as usize;

            let res = &self.grid.data[begin..end];
            self.y += 1;
            Some(res)
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.y += n as isize;
        self.next()
    }
}

pub struct GridColIter<'a, T> {
    grid: &'a Grid<T>,
    x: isize,
}

impl<'a, T> Iterator for GridColIter<'a, T> {
    type Item = std::iter::StepBy<std::slice::Iter<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.grid.width {
            None
        } else {
            let res = self.grid.data[(self.x as usize)..]
                .iter()
                .step_by(self.grid.width as usize);
            self.x += 1;
            Some(res)
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.x += n as isize;
        self.next()
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: isize,
    y: isize,
}

impl<'a, T> Iterator for crate::GridIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.grid.get((self.x, self.y));
        res.map(|item| {
            if self.x + 1 >= self.grid.width {
                self.x = 0;
                self.y += 1;
            } else {
                self.x += 1;
            }

            item
        })
    }
}

pub struct GridEnumerateIter<'a, T> {
    grid: &'a Grid<T>,
    x: isize,
    y: isize,
}

impl<'a, T> Iterator for GridEnumerateIter<'a, T> {
    type Item = (Vec2, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.grid.get((self.x, self.y));
        let coord = (self.x, self.y).into();
        res.map(|item| {
            if self.x + 1 >= self.grid.width {
                self.x = 0;
                self.y += 1;
            } else {
                self.x += 1;
            }

            (coord, item)
        })
    }
}
