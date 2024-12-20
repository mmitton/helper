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
    const MIN: Self;
    const MAX: Self;

    fn abs(self) -> Self {
        self
    }

    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
    fn dist(self, rhs: Self) -> Self;
}

macro_rules! impl_integer {
    (SIGNED => $ty:ty) => {
        impl Integer for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN: Self = <$ty>::MIN;
            const MAX: Self = <$ty>::MAX;

            fn abs(self) -> Self {
                self.abs()
            }

            fn wrapping_add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }

            fn wrapping_sub(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }

            fn wrapping_mul(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }

            fn wrapping_div(self, rhs: Self) -> Self {
                self.wrapping_div(rhs)
            }

            fn dist(self, rhs: Self) -> Self {
                (self - rhs).abs()
            }
        }
    };
    (UNSIGNED => $ty:ty, $sty:ty) => {
        impl Integer for $ty {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN: Self = <$ty>::MIN;
            const MAX: Self = <$ty>::MAX;

            fn wrapping_add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }

            fn wrapping_sub(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }

            fn wrapping_mul(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }

            fn wrapping_div(self, rhs: Self) -> Self {
                self.wrapping_div(rhs)
            }

            fn dist(self, rhs: Self) -> Self {
                (self as $sty - rhs as $sty).unsigned_abs()
            }
        }
    };
}

impl_integer!(UNSIGNED => u8, i8);
impl_integer!(UNSIGNED => u16, i16);
impl_integer!(UNSIGNED => u32, i32);
impl_integer!(UNSIGNED => u64, i64);
impl_integer!(UNSIGNED => u128, i128);
impl_integer!(UNSIGNED => usize, isize);
impl_integer!(SIGNED => i8);
impl_integer!(SIGNED => i16);
impl_integer!(SIGNED => i32);
impl_integer!(SIGNED => i64);
impl_integer!(SIGNED => i128);
impl_integer!(SIGNED => isize);
