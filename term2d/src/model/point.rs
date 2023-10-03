use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn width(&self) -> i32 {
        self.x
    }

    pub const fn height(&self) -> i32 {
        self.y
    }

    pub const fn is_contained(&self, size: &Self) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }

        if self.x >= size.width() || self.y >= size.height() {
            return false;
        }

        true
    }

    pub const fn half(&self) -> Self {
        Self::new(self.x / 2, self.y / 2)
    }

    pub const fn left(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    pub const fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    pub const fn up(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    pub const fn up_left(&self) -> Self {
        Self::new(self.x - 1, self.y - 1)
    }

    pub const fn up_right(&self) -> Self {
        Self::new(self.x + 1, self.y - 1)
    }

    pub const fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    pub const fn down_left(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }

    pub const fn down_right(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }

    pub fn rotate(&self, center: &Point, angle: f32) -> Self {
        let x0 = center.x as f32;
        let y0 = center.y as f32;
        let x1 = self.x as f32;
        let y1 = self.y as f32;
        let rx = (x1 - x0) * angle.cos() - (y1 - y0) * angle.sin() + x0;
        let ry = (x1 - x0) * angle.sin() + (y1 - y0) * angle.cos() + y0;

        Point::new(rx as i32, ry as i32)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from(p: (i32, i32)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

impl From<(u32, u32)> for Point {
    fn from(p: (u32, u32)) -> Self {
        Self::from((p.0 as i32, p.1 as i32))
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod add {
        use super::*;

        #[test]
        fn it_adds_point_and_point() {
            let a = Point::new(2, 3);
            let b = Point::new(5, 7);
            assert_eq!(a + b, Point::new(7, 10));
        }

        #[test]
        fn it_adds_point_and_ref_point() {
            let a = Point::new(2, 3);
            let b = &Point::new(5, 7);
            assert_eq!(a + b, Point::new(7, 10));
        }

        #[test]
        fn it_adds_ref_point_and_point() {
            let a = &Point::new(2, 3);
            let b = Point::new(5, 7);
            assert_eq!(a + b, Point::new(7, 10));
        }

        #[test]
        fn it_adds_ref_point_and_ref_point() {
            let a = &Point::new(2, 3);
            let b = &Point::new(5, 7);
            assert_eq!(a + b, Point::new(7, 10));
        }

        #[test]
        fn it_adds_three_ref_points() {
            let a = &Point::new(0, 0);
            let b = &Point::new(1, 2);
            let c = &Point::new(1, 2);
            assert_eq!(a + b + c, Point::new(2, 4));
        }
    }

    mod addassign {
        use super::*;

        #[test]
        fn it_addassigns_a_point() {
            let mut a = Point::new(2, 3);
            a += Point::new(5, 7);
            assert_eq!(a, Point::new(7, 10));
        }

        #[test]
        fn it_addassigns_a_ref_point() {
            let mut a = Point::new(2, 3);
            a += &Point::new(5, 7);
            assert_eq!(a, Point::new(7, 10));
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn it_subs_point_and_point() {
            let a = Point::new(2, 3);
            let b = Point::new(5, 7);
            assert_eq!(a - b, Point::new(-3, -4));
        }

        #[test]
        fn it_subs_point_and_ref_point() {
            let a = Point::new(2, 3);
            let b = &Point::new(5, 7);
            assert_eq!(a - b, Point::new(-3, -4));
        }

        #[test]
        fn it_subs_ref_point_and_point() {
            let a = &Point::new(2, 3);
            let b = Point::new(5, 7);
            assert_eq!(a - b, Point::new(-3, -4));
        }

        #[test]
        fn it_subs_ref_point_and_ref_point() {
            let a = &Point::new(2, 3);
            let b = &Point::new(5, 7);
            assert_eq!(a - b, Point::new(-3, -4));
        }

        #[test]
        fn it_subs_three_ref_points() {
            let a = &Point::new(0, 0);
            let b = &Point::new(1, 2);
            let c = &Point::new(1, 2);
            assert_eq!(a - b - c, Point::new(-2, -4));
        }
    }

    mod subassign {
        use super::*;

        #[test]
        fn it_subassigns_a_point() {
            let mut a = Point::new(2, 3);
            a -= Point::new(5, 7);
            assert_eq!(a, Point::new(-3, -4));
        }

        #[test]
        fn it_subassigns_a_ref_point() {
            let mut a = Point::new(2, 3);
            a -= &Point::new(5, 7);
            assert_eq!(a, Point::new(-3, -4));
        }
    }
}
