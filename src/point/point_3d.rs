use crate::Integer;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
        let x0 = self.x.wrapping_sub(T::ONE);
        let x1 = self.x;
        let x2 = self.x.wrapping_add(T::ONE);
        let y0 = self.y.wrapping_sub(T::ONE);
        let y1 = self.y;
        let y2 = self.y.wrapping_add(T::ONE);
        let z0 = self.z.wrapping_sub(T::ONE);
        let z1 = self.z;
        let z2 = self.z.wrapping_add(T::ONE);
        [
            Self::new(x1, y1, z0),
            Self::new(x1, y0, z1),
            Self::new(x0, y1, z1),
            Self::new(x2, y1, z1),
            Self::new(x1, y2, z1),
            Self::new(x1, y1, z2),
        ]
    }

    pub fn diagonal_neighbors(&self) -> [Self; 20] {
        let x0 = self.x.wrapping_sub(T::ONE);
        let x1 = self.x;
        let x2 = self.x.wrapping_add(T::ONE);
        let y0 = self.y.wrapping_sub(T::ONE);
        let y1 = self.y;
        let y2 = self.y.wrapping_add(T::ONE);
        let z0 = self.z.wrapping_sub(T::ONE);
        let z1 = self.z;
        let z2 = self.z.wrapping_add(T::ONE);
        [
            Self::new(x0, y0, z0),
            Self::new(x1, y0, z0),
            Self::new(x2, y0, z0),
            Self::new(x0, y1, z0),
            // Self::new(x1, y1, z0),
            Self::new(x2, y1, z0),
            Self::new(x0, y2, z0),
            Self::new(x1, y2, z0),
            Self::new(x2, y2, z0),
            Self::new(x0, y0, z1),
            // Self::new(x1, y0, z1),
            Self::new(x2, y0, z1),
            // Self::new(x0, y1, z1),
            // Self::new(x2, y1, z1),
            Self::new(x0, y2, z1),
            // Self::new(x1, y2, z1),
            Self::new(x2, y2, z1),
            Self::new(x0, y0, z2),
            Self::new(x1, y0, z2),
            Self::new(x2, y0, z2),
            Self::new(x0, y1, z2),
            // Self::new(x1, y1, z2),
            Self::new(x2, y1, z2),
            Self::new(x0, y2, z2),
            Self::new(x1, y2, z2),
            Self::new(x2, y2, z2),
        ]
    }

    pub fn all_neighbors(&self) -> [Self; 26] {
        let x0 = self.x.wrapping_sub(T::ONE);
        let x1 = self.x;
        let x2 = self.x.wrapping_add(T::ONE);
        let y0 = self.y.wrapping_sub(T::ONE);
        let y1 = self.y;
        let y2 = self.y.wrapping_add(T::ONE);
        let z0 = self.z.wrapping_sub(T::ONE);
        let z1 = self.z;
        let z2 = self.z.wrapping_add(T::ONE);
        [
            Self::new(x0, y0, z0),
            Self::new(x1, y0, z0),
            Self::new(x2, y0, z0),
            Self::new(x0, y1, z0),
            Self::new(x1, y1, z0),
            Self::new(x2, y1, z0),
            Self::new(x0, y2, z0),
            Self::new(x1, y2, z0),
            Self::new(x2, y2, z0),
            Self::new(x0, y0, z1),
            Self::new(x1, y0, z1),
            Self::new(x2, y0, z1),
            Self::new(x0, y1, z1),
            Self::new(x2, y1, z1),
            Self::new(x0, y2, z1),
            Self::new(x1, y2, z1),
            Self::new(x2, y2, z1),
            Self::new(x0, y0, z2),
            Self::new(x1, y0, z2),
            Self::new(x2, y0, z2),
            Self::new(x0, y1, z2),
            Self::new(x1, y1, z2),
            Self::new(x2, y1, z2),
            Self::new(x0, y2, z2),
            Self::new(x1, y2, z2),
            Self::new(x2, y2, z2),
        ]
    }

    pub fn scale(&mut self, v: T) -> Self {
        Self {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
        }
    }
}

impl<T: Integer> Add for Point3D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Integer> Sub for Point3D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Integer> Mul for Point3D<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.y * rhs.z)
    }
}

impl<T: Integer> Div for Point3D<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T: Integer> AddAssign for Point3D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Integer> SubAssign for Point3D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Integer> MulAssign for Point3D<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Integer> DivAssign for Point3D<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

#[cfg(test)]
mod test {
    use super::Point3D;

    fn all_directions() -> Vec<Point3D<isize>> {
        let mut directions = Vec::new();
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if z == 0 && y == 0 && x == 0 {
                        continue;
                    }
                    directions.push(Point3D::new(x, y, z));
                }
            }
        }
        directions
    }

    fn is_diagonal(p: &Point3D<isize>) -> bool {
        !is_cardinal(p)
    }

    fn is_cardinal(p: &Point3D<isize>) -> bool {
        (p.x == 0 && (p.y == 0 || p.z == 0)) || (p.y == 0 && p.z == 0)
    }

    #[test]
    fn test_all_directions() {
        let p = Point3D::<isize>::new(0, 0, 0);
        let directions = all_directions();
        let p_neighbors = p.all_neighbors();

        assert_eq!(directions.len(), p_neighbors.len());
        directions
            .iter()
            .zip(p_neighbors.iter())
            .for_each(|(d, n)| {
                println!("neighbor: {n}  direction: {d}");
                assert_eq!(n, d);
            });
    }

    #[test]
    fn test_cardinal() {
        let p = Point3D::<isize>::new(0, 0, 0);
        let directions: Vec<Point3D<isize>> = all_directions()
            .iter()
            .copied()
            .filter(is_cardinal)
            .collect();
        let p_neighbors = p.cardinal_neighbors();

        assert_eq!(directions.len(), p_neighbors.len());
        directions
            .iter()
            .zip(p_neighbors.iter())
            .for_each(|(d, n)| {
                println!("neighbor: {n}  direction: {d}");
                assert_eq!(n, d);
            });
    }

    #[test]
    fn test_diagonal() {
        let p = Point3D::<isize>::new(0, 0, 0);
        let directions: Vec<Point3D<isize>> = all_directions()
            .iter()
            .copied()
            .filter(is_diagonal)
            .collect();
        let p_neighbors = p.diagonal_neighbors();

        assert_eq!(directions.len(), p_neighbors.len());
        directions
            .iter()
            .zip(p_neighbors.iter())
            .for_each(|(d, n)| {
                println!("neighbor: {n}  direction: {d}");
                assert_eq!(n, d);
            });
    }
}
