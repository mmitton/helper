use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub trait Integer:
    Sized
    + Copy
    + Clone
    + Display
    + Debug
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
    + Ord
    + Eq
{
    const ZERO: Self;
    const ONE: Self;

    fn abs(self) -> Self {
        self
    }
}

macro_rules! impl_integer {
    (SIGNED => $ty:ty) => {
        impl Integer for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn abs(self) -> Self {
                self.abs()
            }
        }
    };
    (UNSIGNED => $ty:ty) => {
        impl Integer for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
    };
}

impl_integer!(UNSIGNED => u8);
impl_integer!(UNSIGNED => u16);
impl_integer!(UNSIGNED => u32);
impl_integer!(UNSIGNED => u64);
impl_integer!(UNSIGNED => u128);
impl_integer!(UNSIGNED => usize);
impl_integer!(SIGNED => i8);
impl_integer!(SIGNED => i16);
impl_integer!(SIGNED => i32);
impl_integer!(SIGNED => i64);
impl_integer!(SIGNED => i128);
impl_integer!(SIGNED => isize);
