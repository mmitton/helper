#[derive(Clone)]
pub struct BitArray {
    array: Vec<usize>,
    cap: usize,
}

impl BitArray {
    pub fn new(len: usize) -> Self {
        let cap = ((len + usize::BITS as usize - 1) / usize::BITS as usize) * usize::BITS as usize;
        Self {
            array: vec![0; cap / usize::BITS as usize],
            cap,
        }
    }

    pub fn copy_from(&mut self, other: &Self) {
        if self.array.len() == other.array.len() {
            self.array
                .iter_mut()
                .zip(other.array.iter())
                .for_each(|(s, o)| *s = *o);
        } else {
            self.array = other.array.clone();
            self.cap = other.cap;
        }
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn set(&mut self, bit_idx: usize, set: bool) -> bool {
        let idx = bit_idx / usize::BITS as usize;
        let bit = bit_idx % usize::BITS as usize;

        let entry = &mut self.array[idx];
        let prev = *entry & (1 << bit) != 0;
        if set {
            *entry |= 1 << bit;
        } else {
            *entry &= !(1 << bit);
        }

        prev
    }

    pub fn get(&self, bit_idx: usize) -> bool {
        let idx = bit_idx / usize::BITS as usize;
        let bit = bit_idx % usize::BITS as usize;

        self.array[idx] & (1 << bit) != 0
    }
}
