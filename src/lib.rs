pub mod point;
pub mod triangle;

pub use point::*;
pub use triangle::*;

/// Assumes the points are in counterclockwise order.
#[derive(Debug, Clone)]
pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon { points }
    }

    /// Panics if the number of points in the polygon is less than 3.
    pub fn is_ear(&self, tip: usize) -> bool {
        if self.points.len() == 3 {
            return true;
        }

        if self.interior_angle(tip) >= std::f32::consts::PI {
            return false;
        }

        let triangle = self.triangle(tip);

        for &point in &self.points {
            if point == triangle.a || point == triangle.b || point == triangle.c {
                continue;
            }

            if triangle.point_is_inside(point) {
                return false;
            }
        }

        true
    }

    /// Returns the index of the previous and the next point of tip.
    fn prev_next(&self, tip: usize) -> (usize, usize) {
        let mut prev_point = tip as isize - 1;
        let mut next_point = tip as isize + 1;

        if prev_point < 0 {
            prev_point = self.points.len() as isize - 1;
        }

        if next_point >= self.points.len() as isize {
            next_point = 0;
        }

        (prev_point as usize, next_point as usize)
    }

    /// Returns the interior angle of point index.
    pub fn interior_angle(&self, index: usize) -> f32 {
        let (prev_point, next_point) = self.prev_next(index);

        let ax = self.points[index].x - self.points[prev_point].x;
        let ay = self.points[index].y - self.points[prev_point].y;

        let bx = self.points[next_point].x - self.points[index].x;
        let by = self.points[next_point].y - self.points[index].y;

        let t1 = ay.atan2(ax);
        let t2 = by.atan2(bx);

        use std::f32::consts::PI;

        let mut angle = t2 - t1 + PI;

        while angle < 0. {
            angle += PI * 2.;
        }

        while angle > PI * 2. {
            angle -= PI * 2.;
        }

        angle
    }

    /// Panics if the number of points in the polygon is less than 3.
    pub fn triangle(&self, tip: usize) -> Triangle {
        let (prev_point, next_point) = self.prev_next(tip);

        Triangle {
            a: self.points[prev_point],
            b: self.points[tip],
            c: self.points[next_point]
        }
    }
}
