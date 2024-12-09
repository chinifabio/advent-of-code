use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn clockwise(&self) -> Self {
        Point::new(-self.y, self.x)
    }

    pub fn anti_clockwise(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    pub fn is_on_rect(&self, direction: &Point, point: &Point) -> bool {
        let Point { x: dx, y: dy } = direction;
        if *dx == 0 {
            (point.x == self.x) && ((point.y - self.y) * dy >= 0)
        } else if *dy == 0 {
            (point.y == self.y) && ((point.x - self.x) * dx >= 0)
        } else {
            false
            // (point.x - self.x) * dy == (point.y - self.y) * dx && (point.x - self.x) * dx >= 0 && (point.y - self.y) * dy >= 0
        }
    }
}

impl<T: Into<i32>> From<(T, T)> for Point {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_on_rect_vertical() {
        let origin = Point::new(0, 0);
        let direction = UP;
        let point_on_rect = Point::new(0, -5);
        let point_off_rect = Point::new(1, -5);

        assert!(origin.is_on_rect(&direction, &point_on_rect));
        assert!(!origin.is_on_rect(&direction, &point_off_rect));
    }

    #[test]
    fn test_is_on_rect_horizontal() {
        let origin = Point::new(0, 0);
        let direction = RIGHT;
        let point_on_rect = Point::new(5, 0);
        let point_off_rect = Point::new(5, 1);

        assert!(origin.is_on_rect(&direction, &point_on_rect));
        assert!(!origin.is_on_rect(&direction, &point_off_rect));
    }

    #[test]
    fn test_is_on_rect_diagonal() {
        let origin = Point::new(0, 0);
        let direction = Point::new(1, 1);
        let point_on_rect = Point::new(5, 5);
        let point_off_rect = Point::new(5, 6);

        assert!(!origin.is_on_rect(&direction, &point_on_rect));
        assert!(!origin.is_on_rect(&direction, &point_off_rect));
    }

    #[test]
    fn test_is_on_rect_negative_direction() {
        let origin = Point::new(0, 0);
        let direction = LEFT;
        let point_on_rect = Point::new(-5, 0);
        let point_off_rect = Point::new(-5, 1);

        assert!(origin.is_on_rect(&direction, &point_on_rect));
        assert!(!origin.is_on_rect(&direction, &point_off_rect));
    }
}
