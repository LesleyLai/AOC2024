#![allow(dead_code)]

use std::mem::swap;

type Point = (isize, isize);

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    data: Box<[T]>,
}

impl<T> Grid<T> {
    fn is_out_of_bound(self: &Self, (x, y): Point) -> bool {
        x < 0 || x >= self.width || y < 0 || y >= self.height
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

    pub fn get(&self, (x, y): Point) -> Option<&T> {
        if self.is_out_of_bound((x, y)) {
            None
        } else {
            Some(&self.data[(y * self.width + x) as usize])
        }
    }

    pub fn get_mut(&mut self, (x, y): Point) -> Option<&mut T> {
        if self.is_out_of_bound((x, y)) {
            None
        } else {
            Some(&mut self.data[(y * self.width + x) as usize])
        }
    }

    pub fn rows(self: &Self) -> GridRowIter<T> {
        GridRowIter { grid: self, y: 0 }
    }

    pub fn columns(self: &Self) -> GridColIter<T> {
        GridColIter { grid: self, x: 0 }
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
}

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        self.get(point).unwrap()
    }
}

impl<T> std::ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        self.get_mut(point).unwrap()
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
