use crate::Integer;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::{cmp::Ordering, fmt::Display};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Point2D<T: Integer, const INVERT_SORT: bool = false, const REVERSE_SORT: bool = false> {
    pub x: T,
    pub y: T,
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> Ord
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = if !INVERT_SORT {
            // Sort X then Y
            if self.x != other.x {
                self.x.cmp(&other.x)
            } else {
                self.y.cmp(&other.y)
            }
        } else {
            // Sort Y then X
            if self.y != other.y {
                self.y.cmp(&other.y)
            } else {
                self.x.cmp(&other.x)
            }
        };

        if REVERSE_SORT {
            match cmp {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            }
        } else {
            cmp
        }
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> PartialOrd
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> Display
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool>
    Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn manhattan_dist(&self, rhs: &Self) -> T {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    pub fn cardinal_neighbors(&self) -> [Self; 4] {
        [
            Self::new(self.x, self.y.wrapping_sub(T::ONE)),
            Self::new(self.x.wrapping_sub(T::ONE), self.y),
            Self::new(self.x.wrapping_add(T::ONE), self.y),
            Self::new(self.x, self.y.wrapping_add(T::ONE)),
        ]
    }

    pub fn diagonal_neighbors(&self) -> [Self; 4] {
        [
            Self::new(self.x.wrapping_sub(T::ONE), self.y.wrapping_sub(T::ONE)),
            Self::new(self.x.wrapping_add(T::ONE), self.y.wrapping_sub(T::ONE)),
            Self::new(self.x.wrapping_sub(T::ONE), self.y.wrapping_add(T::ONE)),
            Self::new(self.x.wrapping_add(T::ONE), self.y.wrapping_add(T::ONE)),
        ]
    }

    pub fn all_neighbors(&self) -> [Self; 8] {
        [
            Self::new(self.x.wrapping_sub(T::ONE), self.y.wrapping_sub(T::ONE)),
            Self::new(self.x, self.y.wrapping_sub(T::ONE)),
            Self::new(self.x.wrapping_add(T::ONE), self.y.wrapping_sub(T::ONE)),
            Self::new(self.x.wrapping_sub(T::ONE), self.y),
            Self::new(self.x.wrapping_add(T::ONE), self.y),
            Self::new(self.x.wrapping_sub(T::ONE), self.y.wrapping_add(T::ONE)),
            Self::new(self.x, self.y.wrapping_add(T::ONE)),
            Self::new(self.x.wrapping_add(T::ONE), self.y.wrapping_add(T::ONE)),
        ]
    }

    pub fn scale(&self, v: T) -> Self {
        Self {
            x: self.x * v,
            y: self.y * v,
        }
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> Add
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> Sub
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> Mul
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> Div
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> AddAssign
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> SubAssign
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> MulAssign
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Integer, const INVERT_SORT: bool, const REVERSE_SORT: bool> DivAssign
    for Point2D<T, INVERT_SORT, REVERSE_SORT>
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

#[cfg(test)]
mod test {
    use super::Point2D;

    fn all_directions() -> Vec<Point2D<isize>> {
        let mut directions = Vec::new();
        for y in -1..=1 {
            for x in -1..=1 {
                if y == 0 && x == 0 {
                    continue;
                }
                directions.push(Point2D::new(x, y));
            }
        }
        directions
    }

    fn is_diagonal(p: &Point2D<isize>) -> bool {
        !is_cardinal(p)
    }

    fn is_cardinal(p: &Point2D<isize>) -> bool {
        p.x == 0 || p.y == 0
    }

    #[test]
    fn test_all_directions() {
        let p = Point2D::<isize>::new(0, 0);
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
        let p = Point2D::<isize>::new(0, 0);
        let directions: Vec<Point2D<isize>> = all_directions()
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
        let p = Point2D::<isize>::new(0, 0);
        let directions: Vec<Point2D<isize>> = all_directions()
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
