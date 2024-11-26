use crate::Integer;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Point3D<T: Integer> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Integer> Display for Point3D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl<T: Integer> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn manhattan_dist(&self, rhs: &Self) -> T {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs() + (self.z - rhs.z).abs()
    }
}
