pub mod point;
pub mod triangle;

pub use point::*;
pub use triangle::*;

#[derive(Debug, Clone)]
pub struct Polygon {
    points: Vec<Point>,
}
