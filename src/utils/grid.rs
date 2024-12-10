use std::ops::{Index, IndexMut};

use super::point::Vec2;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.data[y * self.width + x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn from_matrix(matrix: Vec<Vec<T>>) -> Self {
        let height = matrix.len();
        let width = matrix.get(0).map_or(0, |row| row.len());
        let data = matrix.into_iter().flatten().collect();
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(width * height, data.len());
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn contains(&self, point: &Vec2) -> bool {
        point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Vec2> + use<'_, T> {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| Vec2 {
                x: x as i32,
                y: y as i32,
            })
        })
    }
}

impl<T> Index<Vec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vec2) -> &Self::Output {
        assert!(self.contains(&index), "Point not in grid");
        &self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl<T> IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        assert!(self.contains(&index), "Point not in grid");
        &mut self.data[index.y as usize * self.width + index.x as usize]
    }
}
