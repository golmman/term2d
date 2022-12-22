use std::f64::consts::PI;
use std::ops::AddAssign;

use super::point::Point;
use super::rect::Rect;

pub struct Polygon {
    boundary: Rect,
    center: Point,
    vertices: Vec<Point>,
}

impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Self {
        let boundary = Polygon::calc_boundary(&vertices);
        let center = Point::new(
            boundary.pos.x + boundary.size.width() / 2,
            boundary.pos.y + boundary.size.height() / 2,
        );

        Self {
            boundary,
            center,
            vertices,
        }
    }

    pub fn new_star() -> Self {
        let mut vertices = Vec::new();

        let outer_radius = 10.0;
        let inner_radius = 5.0;
        let spikes = 5;

        for i in 0..spikes {
            let outer_angle = (2.0 * PI * i as f64) / spikes as f64;
            let inner_angle = outer_angle + PI / spikes as f64;
            let outer_x = (outer_radius * outer_angle.cos()) as i32;
            let outer_y = (outer_radius * outer_angle.sin()) as i32;
            let inner_x = (inner_radius * inner_angle.cos()) as i32;
            let inner_y = (inner_radius * inner_angle.sin()) as i32;

            vertices.push(Point::new(outer_x, outer_y));
            vertices.push(Point::new(inner_x, inner_y));
        }

        let boundary = Polygon::calc_boundary(&vertices);
        let center = Point::new(0, 0);

        Self {
            boundary,
            center,
            vertices,
        }
    }

    pub const fn boundary(&self) -> &Rect {
        &self.boundary
    }

    pub const fn center(&self) -> &Point {
        &self.center
    }

    pub fn vertices(&self) -> &[Point] {
        self.vertices.as_ref()
    }

    pub fn calc_boundary(vertices: &Vec<Point>) -> Rect {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for vertex in vertices {
            if vertex.x < min_x {
                min_x = vertex.x
            }
            if vertex.x > min_x {
                max_x = vertex.x
            }
            if vertex.y < min_y {
                min_y = vertex.y
            }
            if vertex.y > min_y {
                max_y = vertex.y
            }
        }

        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    pub fn is_inside(&self, p: &Point) -> bool {
        let mut c = false;
        let n = self.vertices.len();
        let Point { x, y } = *p;

        for i in 0..n {
            let Point { x: xi, y: yi } = self.vertices[i];
            let Point { x: xi1, y: yi1 } = self.vertices[(i + 1) % n];

            if ((yi > y) != (yi1 > y)) && (x < (xi1 - xi) * (y - yi) / (yi1 - yi) + xi) {
                c = !c;
            }
        }

        c
    }
}

impl AddAssign<&Point> for Polygon {
    fn add_assign(&mut self, rhs: &Point) {
        self.boundary += rhs;
        self.center += rhs;
        for i in 0..self.vertices.len() {
            self.vertices[i] += rhs;
        }
    }
}
