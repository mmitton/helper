use crate::Integer;
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
        write!(f, "{}x{}", self.x, self.y)
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
            Self::new(self.x.wrapping_sub(T::ONE), self.y),
            Self::new(self.x.wrapping_add(T::ONE), self.y),
            Self::new(self.x, self.y.wrapping_sub(T::ONE)),
            Self::new(self.x, self.y.wrapping_add(T::ONE)),
        ]
    }
}
