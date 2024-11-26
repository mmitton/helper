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

    pub fn cardinal_neighbors(&self) -> [Self; 6] {
        [
            Self {
                x: self.x.wrapping_sub(T::ONE),
                y: self.y,
                z: self.z,
            },
            Self {
                x: self.x.wrapping_add(T::ONE),
                y: self.y,
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y.wrapping_sub(T::ONE),
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y.wrapping_add(T::ONE),
                z: self.z,
            },
            Self {
                x: self.x,
                y: self.y,
                z: self.z.wrapping_sub(T::ONE),
            },
            Self {
                x: self.x,
                y: self.y,
                z: self.z.wrapping_add(T::ONE),
            },
        ]
    }
}
