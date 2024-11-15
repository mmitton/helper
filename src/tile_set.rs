use std::fmt::Display;

use crate::Integer;

#[derive(Default)]
pub struct TileSet<T: Integer> {
    tiles: Vec<Tile<T>>,
}

impl<T: Integer> TileSet<T> {
    pub fn add_tile(&mut self, tile: Tile<T>) {
        let mut tiles = vec![(0, tile)];
        let mut remaining = Vec::new();

        'remaining: while let Some((skip, tile)) = tiles.pop() {
            for (at, t) in self.tiles.iter().enumerate().skip(skip) {
                if tile.overlap_remaining_into(t, &mut remaining) {
                    // tile did not overlap on anything before t (at)
                    // and therefore none of the remaining area will overlap
                    // with anything prior to t (at)
                    for t in remaining.drain(..) {
                        tiles.push((at + 1, t));
                    }
                    continue 'remaining;
                }
            }

            // Nothing overlapped with this remaining tile, add it to the list
            self.tiles.push(tile);
        }
    }

    pub fn remove_tile(&mut self, _tile: Tile<T>) {
        todo!("remove_tile not done yet");
    }

    pub fn iter(&self) -> std::slice::Iter<Tile<T>> {
        self.into_iter()
    }
}

impl<'a, T: Integer> IntoIterator for &'a TileSet<T> {
    type Item = &'a Tile<T>;
    type IntoIter = std::slice::Iter<'a, Tile<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.as_slice().iter()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile<T: Integer> {
    s: Point<T>,
    e: Point<T>,
}

impl<T: Integer> Display for Tile<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.s, self.e)
    }
}

impl<T: Integer> Tile<T> {
    pub fn new(s: Point<T>, e: Point<T>) -> Self {
        Self { s, e }
    }

    pub fn area(&self) -> T {
        (self.e.x - self.s.x + T::ONE) * (self.e.y - self.s.y + T::ONE)
    }

    pub fn overlaps(&self, rhs: &Self) -> Option<Self> {
        let x0 = self.s.x.max(rhs.s.x);
        let y0 = self.s.y.max(rhs.s.y);
        let x1 = self.e.x.min(rhs.e.x);
        let y1 = self.e.y.min(rhs.e.y);

        if x0 <= x1 && y0 <= y1 {
            Some(Self::new(Point::new(x0, y0), Point::new(x1, y1)))
        } else {
            None
        }
    }

    pub fn overlap_remaining_into(&self, rhs: &Self, remaining: &mut Vec<Self>) -> bool {
        if let Some(overlap) = self.overlaps(rhs) {
            // There are 8 possible remaining areas
            // 1  2  3
            // 4  X  5
            // 6  7  8
            if self.s.y < overlap.s.y {
                // Areas 1, 2, 3
                let x0 = overlap.s.x.min(self.s.x);
                let x1 = overlap.e.x.max(self.e.x);
                let t = Tile::new(
                    Point::new(x0, self.s.y),
                    Point::new(x1, overlap.s.y - T::ONE),
                );
                remaining.push(t);
            }

            if self.s.x < overlap.s.x {
                // Area 4
                let t = Tile::new(
                    Point::new(self.s.x, overlap.s.y),
                    Point::new(overlap.s.x - T::ONE, overlap.e.y),
                );
                remaining.push(t);
            }
            if self.e.x > overlap.e.x {
                // Area 5
                let t = Tile::new(
                    Point::new(overlap.e.x + T::ONE, overlap.s.y),
                    Point::new(self.e.x, overlap.e.y),
                );
                remaining.push(t);
            }

            if self.e.y > overlap.e.y {
                // Areas 6, 7, 8
                let x0 = overlap.s.x.min(self.s.x);
                let x1 = overlap.e.x.max(self.e.x);
                let t = Tile::new(
                    Point::new(x0, overlap.e.y + T::ONE),
                    Point::new(x1, self.e.y),
                );
                remaining.push(t);
            }

            true
        } else {
            false
        }
    }

    pub fn overlap_remaining(&self, rhs: &Self) -> Option<Vec<Self>> {
        let mut remaining = Vec::new();
        if self.overlap_remaining_into(rhs, &mut remaining) {
            Some(remaining)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T: Integer> {
    pub x: T,
    pub y: T,
}

impl<T: Integer> Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

impl<T: Integer> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn manhattan_dist(&self, rhs: &Self) -> T {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}
