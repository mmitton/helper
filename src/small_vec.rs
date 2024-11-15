use std::{
    fmt::{Binary, Debug},
    marker::PhantomData,
    ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl, Shr},
};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct SmallVec<T, K, const LEN: u32>
where
    T: Default,
{
    buffer: T,
    _phantom: PhantomData<K>,
}

impl<T, K, const LEN: u32> Default for SmallVec<T, K, LEN>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            buffer: T::default(),
            _phantom: PhantomData,
        }
    }
}

impl<T, K, const LEN: u32> Debug for SmallVec<T, K, LEN>
where
    T: SmallVecBuffer + From<K>,
    K: From<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T, K, const LEN: u32> SmallVec<T, K, LEN>
where
    T: SmallVecBuffer + From<K>,
    K: From<T>,
{
    const CAP: u8 = (T::BITS - Self::LEN_WIDTH) as u8 / LEN as u8;
    const LEN_WIDTH: u32 = T::BITS - (T::BITS - ((T::BITS / LEN) as u8).leading_zeros());

    pub fn new() -> Self {
        #[cfg(test)]
        println!("CAP:{} LEN_WIDTH:{}", Self::CAP, Self::LEN_WIDTH);
        Self::default()
    }

    pub fn pop(&mut self) -> Option<K> {
        if self.is_empty() {
            None
        } else {
            let len = self.set_len(|len| len - 1) as u32;
            let mut v: T = self.buffer >> ((LEN * len) + Self::LEN_WIDTH);
            v &= !(T::ONES << LEN);
            let mask = !(!(T::ONES << LEN) << ((LEN * len) + Self::LEN_WIDTH));
            self.buffer &= mask;
            Some(v.into())
        }
    }

    pub fn push(&mut self, v: K) {
        assert!(
            self.len() < self.capacity(),
            "SmallVec exceeded capacity of {}",
            Self::CAP
        );
        let v: T = v.into();
        assert!(
            v.leading_zeros() >= T::BITS - LEN,
            "0b{:b} is too big to fit in {LEN} bits",
            v
        );
        let v = v << ((LEN * self.len() as u32) + Self::LEN_WIDTH);
        self.buffer |= v;
        self.set_len(|len| len + 1);
    }

    pub fn len(&self) -> u8 {
        (self.buffer & Self::len_mask()).into_u8()
    }

    fn set_len<F>(&mut self, f: F) -> u8
    where
        F: FnOnce(u8) -> u8,
    {
        let new_len = f((self.buffer & Self::len_mask()).into_u8());
        self.buffer &= !Self::len_mask();
        let len = T::from_u8(new_len);
        self.buffer |= len;
        new_len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> SmallVecIter<T, K, LEN> {
        self.into_iter()
    }

    pub fn capacity(&self) -> u8 {
        Self::CAP
    }

    fn len_mask() -> T {
        !(T::ONES << Self::LEN_WIDTH)
    }

    fn buffer(&self) -> T {
        (self.buffer & !Self::len_mask()) >> Self::LEN_WIDTH
    }
}

impl<T, K, const LEN: u32> SmallVec<T, K, LEN>
where
    T: SmallVecBuffer + From<K>,
    K: From<T>,
{
}

impl<'a, T, K, const LEN: u32> IntoIterator for &'a SmallVec<T, K, LEN>
where
    T: SmallVecBuffer + From<K>,
    K: From<T>,
{
    type Item = K;
    type IntoIter = SmallVecIter<'a, T, K, LEN>;

    fn into_iter(self) -> Self::IntoIter {
        SmallVecIter {
            idx: 0,
            len: self.len(),
            small_vec: self,
        }
    }
}

pub struct SmallVecIter<'a, T, K, const LEN: u32>
where
    T: Default,
{
    idx: u8,
    len: u8,
    small_vec: &'a SmallVec<T, K, LEN>,
}

impl<'a, T, K, const LEN: u32> Iterator for SmallVecIter<'a, T, K, LEN>
where
    T: SmallVecBuffer + From<K>,
    K: From<T>,
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            let mut v: T = self.small_vec.buffer() >> (LEN * self.idx as u32);
            v &= !(T::ONES << LEN);
            self.idx += 1;
            Some(v.into())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.len - self.idx) as usize;
        (remaining, Some(remaining))
    }
}

impl<'a, T, K, const LEN: u32> ExactSizeIterator for SmallVecIter<'a, T, K, LEN>
where
    T: SmallVecBuffer + From<K>,
    K: From<T>,
{
    fn len(&self) -> usize {
        (self.len - self.idx) as usize
    }
}

impl<'a, T, K, const LEN: u32> DoubleEndedIterator for SmallVecIter<'a, T, K, LEN>
where
    T: SmallVecBuffer + From<K>,
    K: From<T>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            let mut v: T = self.small_vec.buffer() >> (LEN * (self.len - 1) as u32);
            v &= !(T::ONES << LEN);
            self.len -= 1;
            Some(v.into())
        }
    }
}

pub trait SmallVecBuffer:
    Default
    + Binary
    + Clone
    + Copy
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOrAssign
    + Not<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
{
    const ONES: Self;
    const ZERO: Self;
    const BITS: u32;

    fn leading_zeros(self) -> u32;
    fn into_u8(self) -> u8;
    fn from_u8(v: u8) -> Self;
}

macro_rules! impl_bits {
    ($ty:ty) => {
        impl SmallVecBuffer for $ty {
            const ONES: Self = !0;
            const ZERO: Self = 0;
            const BITS: u32 = <$ty>::BITS;

            fn leading_zeros(self) -> u32 {
                <$ty>::leading_zeros(self)
            }

            fn into_u8(self) -> u8 {
                self as u8
            }

            fn from_u8(v: u8) -> $ty {
                v as $ty
            }
        }
    };
}

impl_bits!(usize);
impl_bits!(u8);
impl_bits!(u16);
impl_bits!(u32);
impl_bits!(u64);
impl_bits!(u128);

#[cfg(test)]
mod test {
    use super::SmallVec;

    #[derive(Debug, PartialEq)]
    enum TestEnum {
        A,
        B,
        C,
        D,
    }

    macro_rules! impl_from {
        ($ty:ty) => {
            impl From<TestEnum> for $ty {
                fn from(value: TestEnum) -> Self {
                    match value {
                        TestEnum::A => 0,
                        TestEnum::B => 1,
                        TestEnum::C => 2,
                        TestEnum::D => 3,
                    }
                }
            }

            impl From<$ty> for TestEnum {
                fn from(value: $ty) -> Self {
                    match value {
                        0 => TestEnum::A,
                        1 => TestEnum::B,
                        2 => TestEnum::C,
                        3 => TestEnum::D,
                        _ => panic!("{value:?} is an invalid TestEnum"),
                    }
                }
            }
        };
    }

    impl_from!(u8);
    impl_from!(u16);
    impl_from!(u32);

    #[test]
    fn small_vec() {
        let mut array: SmallVec<u16, TestEnum, 2> = SmallVec::new();

        macro_rules! expect_buffer {
            ($pop:expr, $buffer:literal, $len:literal) => {{
                let popped = array.pop();
                assert_eq!(
                    popped, $pop,
                    "Expected array.pop() to be {:?}, got {:?}",
                    $pop, popped
                );
                expect_buffer!($buffer, $len);
            }};

            ($buffer:literal, $len:literal) => {{
                assert_eq!(
                    array.buffer(),
                    $buffer,
                    "Expected array.buffer() to be {:08b}, got {:08b}",
                    $buffer,
                    array.buffer()
                );
                assert_eq!(
                    array.len(),
                    $len,
                    "Expected array.len() to be {}, got {}",
                    $len,
                    array.len()
                );
            }};
        }

        expect_buffer!(0b00000000, 0);

        array.push(TestEnum::A);
        expect_buffer!(0b00000000, 1);

        array.push(TestEnum::B);
        expect_buffer!(0b00000100, 2);

        array.push(TestEnum::C);
        expect_buffer!(0b00100100, 3);

        array.push(TestEnum::D);
        expect_buffer!(0b11100100, 4);

        expect_buffer!(Some(TestEnum::D), 0b00100100, 3);
        expect_buffer!(Some(TestEnum::C), 0b00000100, 2);
        expect_buffer!(Some(TestEnum::B), 0b00000000, 1);
        expect_buffer!(Some(TestEnum::A), 0b00000000, 0);
        expect_buffer!(None::<TestEnum>, 0b00000000, 0);
    }

    #[test]
    #[should_panic(expected = "SmallVec exceeded capacity of 6")]
    fn small_vec_full_push() {
        let mut array: SmallVec<u16, TestEnum, 2> = SmallVec::new();
        for _ in 0..=array.capacity() {
            array.push(TestEnum::D);
        }
    }

    #[test]
    fn small_vec_iter() {
        let mut array: SmallVec<u16, TestEnum, 2> = SmallVec::new();
        array.push(TestEnum::A);
        array.push(TestEnum::B);
        array.push(TestEnum::C);
        array.push(TestEnum::D);

        for _ in 0..2 {
            let mut iter = array.into_iter();
            assert_eq!(iter.next(), Some(TestEnum::A));
            assert_eq!(iter.next(), Some(TestEnum::B));
            assert_eq!(iter.next(), Some(TestEnum::C));
            assert_eq!(iter.next(), Some(TestEnum::D));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        for _ in 0..2 {
            let mut iter = array.iter().rev();
            assert_eq!(iter.next(), Some(TestEnum::D));
            assert_eq!(iter.next(), Some(TestEnum::C));
            assert_eq!(iter.next(), Some(TestEnum::B));
            assert_eq!(iter.next(), Some(TestEnum::A));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        let mut iter = array.iter();
        assert_eq!(iter.next(), Some(TestEnum::A));
        assert_eq!(iter.next_back(), Some(TestEnum::D));
        assert_eq!(iter.next(), Some(TestEnum::B));
        assert_eq!(iter.next_back(), Some(TestEnum::C));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic(expected = "0b10 is too big to fit in 1 bits")]
    fn small_vec_too_big_value() {
        let mut array: SmallVec<u16, TestEnum, 1> = SmallVec::new();
        array.push(TestEnum::A);
        array.push(TestEnum::B);
        array.push(TestEnum::C);
    }

    #[test]
    fn small_vec_capacity() {
        let array: SmallVec<u16, TestEnum, 1> = SmallVec::new();
        assert_eq!(array.capacity(), 13);

        let array: SmallVec<u16, TestEnum, 2> = SmallVec::new();
        assert_eq!(array.capacity(), 6);

        let array: SmallVec<u32, TestEnum, 3> = SmallVec::new();
        assert_eq!(array.capacity(), 9);
    }
}
