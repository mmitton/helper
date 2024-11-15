pub struct BitGridConst<
    const MIN_X: isize,
    const MIN_Y: isize,
    const WIDTH: usize,
    const HEIGHT: usize,
> {
    grid: Vec<usize>,
}

impl<const MIN_X: isize, const MIN_Y: isize, const WIDTH: usize, const HEIGHT: usize> Default
    for BitGridConst<MIN_X, MIN_Y, WIDTH, HEIGHT>
{
    fn default() -> Self {
        assert_eq!(WIDTH % Self::BITS, 0);
        Self {
            grid: vec![0; (WIDTH / Self::BITS) * HEIGHT],
        }
    }
}

impl<const MIN_X: isize, const MIN_Y: isize, const WIDTH: usize, const HEIGHT: usize>
    BitGridConst<MIN_X, MIN_Y, WIDTH, HEIGHT>
{
    const BITS: usize = usize::BITS as usize;

    pub fn new() -> Self {
        Self::default()
    }

    fn index_bit(&self, x: isize, y: isize) -> (usize, usize) {
        let nx = (x - MIN_X) as usize;
        let ny = (y - MIN_Y) as usize;
        let pos = (ny * WIDTH) + nx;
        (pos / Self::BITS, 1 << (pos % Self::BITS))
    }

    pub fn get_surround(&self, x: isize, y: isize) -> u16 {
        let (index, bit) = self.index_bit(x - 1, y - 1);
        let mut top = self.grid[index] >> bit.trailing_zeros();
        let mut middle = self.grid[index + (WIDTH / Self::BITS)] >> bit.trailing_zeros();
        let mut bottom = self.grid[index + (2 * WIDTH / Self::BITS)] >> bit.trailing_zeros();

        let extra_bits = Self::BITS - bit.trailing_zeros() as usize;
        if extra_bits < 3 {
            let extra_top = self.grid[index + 1] << extra_bits;
            let extra_middle = self.grid[index + 1 + (WIDTH / Self::BITS)] << extra_bits;

            let extra_bottom = self.grid[index + 1 + (2 * WIDTH / Self::BITS)] << extra_bits;
            top |= extra_top;
            middle |= extra_middle;
            bottom |= extra_bottom;
        }

        (((top & 0b111) << 6) | ((middle & 0b111) << 3) | (bottom & 0b111)) as u16
    }

    pub fn set_bit(&mut self, x: isize, y: isize) {
        let (index, bit) = self.index_bit(x, y);
        self.grid[index] |= bit;
    }

    pub fn clear_bit(&mut self, x: isize, y: isize) {
        let (index, bit) = self.index_bit(x, y);
        self.grid[index] &= !bit;
    }

    pub fn bit_is_set(&self, x: isize, y: isize) -> bool {
        let (index, bit) = self.index_bit(x, y);
        self.grid[index] & bit != 0
    }

    pub fn clear(&mut self) {
        self.grid.iter_mut().for_each(|v| *v = 0);
    }
}

pub struct BitGrid<const BX: usize = 10, const BY: usize = 10> {
    grid: Vec<usize>,
    min: (isize, isize),
    max: (isize, isize),
    width: usize,
    min_set: (isize, isize),
    max_set: (isize, isize),
}

impl<const BX: usize, const BY: usize> Default for BitGrid<BX, BY> {
    fn default() -> Self {
        Self {
            grid: Vec::new(),
            min: (isize::MAX, isize::MAX),
            max: (isize::MIN, isize::MIN),
            width: 0,
            min_set: (isize::MAX, isize::MAX),
            max_set: (isize::MIN, isize::MIN),
        }
    }
}

impl<const BX: usize, const BY: usize> BitGrid<BX, BY> {
    const BITS: usize = usize::BITS as usize;

    pub fn new() -> Self {
        Self::default()
    }

    fn index_bit(&self, x: isize, y: isize) -> (usize, usize) {
        self.index_bit_full(x, y, &self.min, self.width)
    }

    fn index_bit_full(
        &self,
        x: isize,
        y: isize,
        min: &(isize, isize),
        width: usize,
    ) -> (usize, usize) {
        let nx = (x - min.0) as usize;
        let ny = (y - min.1) as usize;
        let pos = (ny * width) + nx;
        (pos / Self::BITS, 1 << (pos % Self::BITS))
    }

    fn resize(&mut self, min: (isize, isize), max: (isize, isize)) {
        let new_min = ((min.0 >> 6) << 6, min.1);
        let new_max = (((max.0 >> 6) << 6) + usize::BITS as isize - 1, max.1);

        let new_width = (new_max.0 - new_min.0 + 1) as usize;
        let new_height = (new_max.1 - new_min.1 + 1) as usize;
        let mut new_grid = vec![0; (new_width / Self::BITS) * new_height];
        debug_assert!(
            (new_min != self.min || new_max != self.max)
                && new_min.0 <= self.min.0
                && new_min.1 <= self.min.1
                && new_max.0 >= self.max.0
                && new_max.1 >= self.max.1
        );

        if !self.grid.is_empty() {
            let (mut index, bit) = self.index_bit_full(self.min.0, self.min.1, &new_min, new_width);
            debug_assert!(bit == 1);

            if new_min.0 == self.min.0 && new_max.0 == self.max.0 {
                // Quicker, only increasing in y direction
                new_grid[index..index + self.grid.len()].copy_from_slice(&self.grid);
            } else {
                // Need to copy each row individuallyA
                let mut old_index = 0;
                let old_stride = self.width / usize::BITS as usize;
                let new_stride = new_width / usize::BITS as usize;
                for _ in self.min.1..=self.max.1 {
                    new_grid[index..index + old_stride]
                        .copy_from_slice(&self.grid[old_index..old_index + old_stride]);
                    index += new_stride;
                    old_index += old_stride;
                }
            }
        }

        self.grid = new_grid;
        self.min = new_min;
        self.max = new_max;
        self.width = new_width;
    }

    pub fn get_surround(&mut self, x: isize, y: isize) -> u16 {
        self.min_set.0 = self.min_set.0.min(x - 1);
        self.min_set.1 = self.min_set.1.min(y - 1);
        self.max_set.0 = self.max_set.0.max(x + 1);
        self.max_set.1 = self.max_set.1.max(y + 1);

        if self.min_set.0 < self.min.0
            || self.max_set.0 > self.max.0
            || self.min_set.1 < self.min.1
            || self.max_set.1 > self.max.1
        {
            self.resize(
                (self.min_set.0 - BX as isize, self.min_set.1 - BY as isize),
                (self.max_set.0 + BX as isize, self.max_set.1 + BY as isize),
            );
        }

        let (index, bit) = self.index_bit(x - 1, y - 1);
        let mut top = self.grid[index] >> bit.trailing_zeros();
        let mut middle = self.grid[index + (self.width / Self::BITS)] >> bit.trailing_zeros();
        let mut bottom = self.grid[index + (2 * self.width / Self::BITS)] >> bit.trailing_zeros();

        let extra_bits = Self::BITS - bit.trailing_zeros() as usize;
        if extra_bits < 3 {
            let extra_top = self.grid[index + 1] << extra_bits;
            let extra_middle = self.grid[index + 1 + (self.width / Self::BITS)] << extra_bits;

            let extra_bottom = self.grid[index + 1 + (2 * self.width / Self::BITS)] << extra_bits;
            top |= extra_top;
            middle |= extra_middle;
            bottom |= extra_bottom;
        }

        (((top & 0b111) << 6) | ((middle & 0b111) << 3) | (bottom & 0b111)) as u16
    }

    pub fn set_bit(&mut self, x: isize, y: isize) {
        self.min_set.0 = self.min_set.0.min(x);
        self.min_set.1 = self.min_set.1.min(y);
        self.max_set.0 = self.max_set.0.max(x);
        self.max_set.1 = self.max_set.1.max(y);

        if self.min_set.0 < self.min.0
            || self.max_set.0 > self.max.0
            || self.min_set.1 < self.min.1
            || self.max_set.1 > self.max.1
        {
            self.resize(
                (self.min_set.0 - BX as isize, self.min_set.1 - BY as isize),
                (self.max_set.0 + BX as isize, self.max_set.1 + BY as isize),
            );
        }

        let (index, bit) = self.index_bit(x, y);
        self.grid[index] |= bit;
    }

    pub fn clear_bit(&mut self, x: isize, y: isize) {
        if x < self.min.0 || x > self.max.0 || y < self.min.1 || y > self.max.1 {
            return;
        }
        let (index, bit) = self.index_bit(x, y);
        self.grid[index] &= !bit;
    }

    pub fn bit_is_set(&self, x: isize, y: isize) -> bool {
        if x < self.min.0 || x > self.max.0 || y < self.min.1 || y > self.max.1 {
            return false;
        }
        let (index, bit) = self.index_bit(x, y);
        self.grid[index] & bit != 0
    }

    pub fn clear(&mut self) {
        self.grid.iter_mut().for_each(|v| *v = 0);
    }

    pub fn count_set(&self) -> usize {
        self.grid.iter().fold(0, |c, v| c + v.count_ones() as usize)
    }
}

mod test {
    #[test]
    fn bit_grid() {
        let mut grid: super::BitGrid = super::BitGrid::new();
        let mut set_bits = std::collections::BTreeSet::new();

        println!("Checking to see if a new grid doesn't have any bits set");
        for y in 0..10 {
            for x in -10..10 {
                assert!(!grid.bit_is_set(x, y));
            }
        }

        println!("Checking to see if a clearing bits in a new grid doesn't resize the grid");
        for y in 0..10 {
            for x in -10..10 {
                grid.clear_bit(x, y)
            }
        }
        assert!(grid.grid.is_empty(), "Grid should still be empty");

        grid.set_bit(100, 100);
        set_bits.insert((100, 100));
        println!(
            "grid.len():{}  min:{:?}  max:{:?}  min_set:{:?}  max_set:{:?}  width:{}",
            grid.grid.len(),
            grid.min,
            grid.max,
            grid.min_set,
            grid.max_set,
            grid.width
        );
        assert_eq!(grid.grid.len(), 21);
        assert_eq!(grid.width, 64);
        assert_eq!(grid.min, (64, 90));
        assert_eq!(grid.max, (127, 110));
        assert_eq!(grid.min_set, (100, 100));
        assert_eq!(grid.max_set, (100, 100));

        println!("Check for resize in the -y dir");
        for i in 1..=10 {
            grid.set_bit(100, 100 - i);
            set_bits.insert((100, 100 - i));
        }
        assert_eq!(grid.grid.len(), 21);
        assert_eq!(grid.width, 64);
        assert_eq!(grid.min, (64, 90));
        assert_eq!(grid.max, (127, 110));
        assert_eq!(grid.min_set, (100, 90));
        assert_eq!(grid.max_set, (100, 100));

        // One more will resize
        grid.set_bit(100, 89);
        set_bits.insert((100, 89));
        assert_eq!(grid.grid.len(), 32);
        assert_eq!(grid.width, 64);
        assert_eq!(grid.min, (64, 79));
        assert_eq!(grid.max, (127, 110));
        assert_eq!(grid.min_set, (100, 89));
        assert_eq!(grid.max_set, (100, 100));

        println!("Check for resize in the +y dir");
        for i in 1..=10 {
            grid.set_bit(100, 100 + i);
            set_bits.insert((100, 100 + i));
        }
        assert_eq!(grid.grid.len(), 32);
        assert_eq!(grid.width, 64);
        assert_eq!(grid.min, (64, 79));
        assert_eq!(grid.max, (127, 110));
        assert_eq!(grid.min_set, (100, 89));
        assert_eq!(grid.max_set, (100, 110));

        // One more will resize
        grid.set_bit(100, 111);
        set_bits.insert((100, 111));

        println!(
            "grid.len():{}  min:{:?}  max:{:?}  min_set:{:?}  max_set:{:?}  width:{}",
            grid.grid.len(),
            grid.min,
            grid.max,
            grid.min_set,
            grid.max_set,
            grid.width
        );
        assert_eq!(grid.grid.len(), 43);
        assert_eq!(grid.width, 64);
        assert_eq!(grid.min, (64, 79));
        assert_eq!(grid.max, (127, 121));
        assert_eq!(grid.min_set, (100, 89));
        assert_eq!(grid.max_set, (100, 111));

        // Should still not resize
        grid.clear_bit(0, 100);
        assert_eq!(grid.grid.len(), 43);
        assert_eq!(grid.width, 64);
        assert_eq!(grid.min, (64, 79));
        assert_eq!(grid.max, (127, 121));
        assert_eq!(grid.min_set, (100, 89));
        assert_eq!(grid.max_set, (100, 111));

        // This will resize in the -x
        grid.set_bit(0, 100);
        set_bits.insert((0, 100));
        println!(
            "grid.len():{}  min:{:?}  max:{:?}  min_set:{:?}  max_set:{:?}  width:{}",
            grid.grid.len(),
            grid.min,
            grid.max,
            grid.min_set,
            grid.max_set,
            grid.width
        );
        assert_eq!(grid.grid.len(), 129);
        assert_eq!(grid.width, 192);
        assert_eq!(grid.min, (-64, 79));
        assert_eq!(grid.max, (127, 121));
        assert_eq!(grid.min_set, (0, 89));
        assert_eq!(grid.max_set, (100, 111));

        // This will resize in the +x
        grid.set_bit(255, 100);
        set_bits.insert((255, 100));
        println!(
            "grid.len():{}  min:{:?}  max:{:?}  min_set:{:?}  max_set:{:?}  width:{}",
            grid.grid.len(),
            grid.min,
            grid.max,
            grid.min_set,
            grid.max_set,
            grid.width
        );
        assert_eq!(grid.grid.len(), 258);
        assert_eq!(grid.width, 384);
        assert_eq!(grid.min, (-64, 79));
        assert_eq!(grid.max, (319, 121));
        assert_eq!(grid.min_set, (0, 89));
        assert_eq!(grid.max_set, (255, 111));

        // Scan to see if the set_bits and grid agree
        for y in grid.min.1..=grid.max.1 {
            for x in grid.min.0..=grid.max.0 {
                assert_eq!(grid.bit_is_set(x, y), set_bits.contains(&(x, y)));
            }
        }

        // Clear a bit and recheck set_bits and grid
        grid.clear_bit(100, 100);
        set_bits.remove(&(100, 100));
        for y in grid.min.1..=grid.max.1 {
            for x in grid.min.0..=grid.max.0 {
                assert_eq!(grid.bit_is_set(x, y), set_bits.contains(&(x, y)));
            }
        }
        assert_eq!(grid.grid.len(), 258);
        assert_eq!(grid.width, 384);
        assert_eq!(grid.min, (-64, 79));
        assert_eq!(grid.max, (319, 121));
        assert_eq!(grid.min_set, (0, 89));
        assert_eq!(grid.max_set, (255, 111));
    }

    #[test]
    fn test_min_max() {
        fn min_max(i: i8) -> (i8, i8) {
            let min = (i >> 6) << 6;
            let max = ((i >> 6) << 6) + usize::BITS as i8 - 1;
            println!("{i} {min} {max}  {i:0b} {min:0b} {max:0b}");
            (min, max)
        }

        assert_eq!(min_max(0), (0, 63));
        assert_eq!(min_max(1), (0, 63));
        assert_eq!(min_max(-1), (-64, -1));
    }
}
