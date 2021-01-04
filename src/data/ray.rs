use super::{Vec3, Point};

pub struct Ray<'a> {
    origin: &'a Point,
    direction: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point, direction: &'a Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin() + self.direction() * t
    }
}
