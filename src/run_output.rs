use std::fmt::Display;

pub enum RunOutput {
    String(String),
    Usize(usize),
    Isize(isize),
    U128(u128),
    I128(i128),
}

impl Display for RunOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::Usize(v) => write!(f, "{v}"),
            Self::Isize(v) => write!(f, "{v}"),
            Self::U128(v) => write!(f, "{v}"),
            Self::I128(v) => write!(f, "{v}"),
        }
    }
}

macro_rules! impl_from {
    ($ty:ty, $cast:ty, $enum:ident) => {
        impl From<$ty> for RunOutput {
            fn from(value: $ty) -> Self {
                Self::$enum(value as $cast)
            }
        }
    };
}

impl_from!(String, String, String);
impl_from!(u8, usize, Usize);
impl_from!(u16, usize, Usize);
impl_from!(u32, usize, Usize);
impl_from!(u64, usize, Usize);
impl_from!(u128, u128, U128);
impl_from!(usize, usize, Usize);
impl_from!(i8, isize, Isize);
impl_from!(i16, isize, Isize);
impl_from!(i32, isize, Isize);
impl_from!(i64, isize, Isize);
impl_from!(isize, isize, Isize);
impl_from!(i128, i128, I128);
