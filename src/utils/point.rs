use std::iter::Iterator;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub const UP: Vec2 = Vec2 { x: 0, y: -1 };
pub const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
pub const LEFT: Vec2 = Vec2 { x: -1, y: 0 };
pub const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn clockwise(&self) -> Self {
        Vec2::new(-self.y, self.x)
    }

    pub fn anti_clockwise(&self) -> Self {
        Vec2::new(self.y, -self.x)
    }

    /// Returns true if the point is on the rectangle defined by the origin and the direction.
    pub fn is_on_semirect(&self, direction: &Vec2, point: &Vec2) -> bool {
        let Vec2 { x: dx, y: dy } = direction;
        if *dx == 0 {
            (point.x == self.x) && ((point.y - self.y) * dy >= 0)
        } else if *dy == 0 {
            (point.y == self.y) && ((point.x - self.x) * dx >= 0)
        } else {
            false
            // (point.x - self.x) * dy == (point.y - self.y) * dx && (point.x - self.x) * dx >= 0 && (point.y - self.y) * dy >= 0
        }
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Vec2::new(
            (self.x as f64 / length).round() as i32,
            (self.y as f64 / length).round() as i32,
        )
    }

    pub fn interpolate(&self, other: &Vec2) -> Vec2Iterator {
        Vec2Iterator::new(*self, *other)
    }
}

impl<T: Into<i32>> From<(T, T)> for Vec2 {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

pub struct Vec2Iterator {
    start: Vec2,
    end: Vec2,
    count: usize,
    done: bool,
    direction: Vec2,
}

impl Vec2Iterator {
    fn new(start: Vec2, end: Vec2) -> Self {
        Self {
            start,
            end,
            count: 0,
            done: false,
            direction: (end - start).normalize(),
        }
    }
}

impl Iterator for Vec2Iterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let next = self.start + self.direction * self.count as i32;
        if next == self.end {
            self.done = true;
        }
        self.count += 1;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_on_rect_vertical() {
        let origin = Vec2::new(0, 0);
        let direction = UP;
        let point_on_rect = Vec2::new(0, -5);
        let point_off_rect = Vec2::new(1, -5);

        assert!(origin.is_on_semirect(&direction, &point_on_rect));
        assert!(!origin.is_on_semirect(&direction, &point_off_rect));
    }

    #[test]
    fn test_is_on_rect_horizontal() {
        let origin = Vec2::new(0, 0);
        let direction = RIGHT;
        let point_on_rect = Vec2::new(5, 0);
        let point_off_rect = Vec2::new(5, 1);

        assert!(origin.is_on_semirect(&direction, &point_on_rect));
        assert!(!origin.is_on_semirect(&direction, &point_off_rect));
    }

    #[test]
    fn test_is_on_rect_diagonal() {
        let origin = Vec2::new(0, 0);
        let direction = Vec2::new(1, 1);
        let point_on_rect = Vec2::new(5, 5);
        let point_off_rect = Vec2::new(5, 6);

        assert!(!origin.is_on_semirect(&direction, &point_on_rect));
        assert!(!origin.is_on_semirect(&direction, &point_off_rect));
    }

    #[test]
    fn test_is_on_rect_negative_direction() {
        let origin = Vec2::new(0, 0);
        let direction = LEFT;
        let point_on_rect = Vec2::new(-5, 0);
        let point_off_rect = Vec2::new(-5, 1);

        assert!(origin.is_on_semirect(&direction, &point_on_rect));
        assert!(!origin.is_on_semirect(&direction, &point_off_rect));
    }

    #[test]
    fn test_vec2_add() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, 4);
        let result = a + b;
        assert_eq!(result, Vec2::new(4, 6));
    }

    #[test]
    fn test_vec2_add_assign() {
        let mut a = Vec2::new(1, 2);
        let b = Vec2::new(3, 4);
        a += b;
        assert_eq!(a, Vec2::new(4, 6));
    }

    #[test]
    fn test_vec2_sub() {
        let a = Vec2::new(5, 7);
        let b = Vec2::new(3, 4);
        let result = a - b;
        assert_eq!(result, Vec2::new(2, 3));
    }

    #[test]
    fn test_vec2_sub_assign() {
        let mut a = Vec2::new(5, 7);
        let b = Vec2::new(3, 4);
        a -= b;
        assert_eq!(a, Vec2::new(2, 3));
    }

    #[test]
    fn test_vec2_mul() {
        let a = Vec2::new(2, 3);
        let result = a * 3;
        assert_eq!(result, Vec2::new(6, 9));
    }

    #[test]
    fn test_vec2_mul_assign() {
        let mut a = Vec2::new(2, 3);
        a *= 3;
        assert_eq!(a, Vec2::new(6, 9));
    }

    #[test]
    fn test_vec2_length() {
        let a = Vec2::new(3, 4);
        assert_eq!(a.length(), 5.0);
    }

    #[test]
    fn test_vec2_clockwise() {
        let a = Vec2::new(1, 2);
        let result = a.clockwise();
        assert_eq!(result, Vec2::new(-2, 1));
    }

    #[test]
    fn test_vec2_anti_clockwise() {
        let a = Vec2::new(1, 2);
        let result = a.anti_clockwise();
        assert_eq!(result, Vec2::new(2, -1));
    }

    #[test]
    fn test_vec2_interpolate() {
        let start = Vec2::new(0, 0);
        let end = Vec2::new(3, 3);
        let mut iter = start.interpolate(&end);
        assert_eq!(iter.next(), Some(Vec2::new(0, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(1, 1)));
        assert_eq!(iter.next(), Some(Vec2::new(2, 2)));
        assert_eq!(iter.next(), Some(Vec2::new(3, 3)));
        assert_eq!(iter.next(), None);
    }
}
