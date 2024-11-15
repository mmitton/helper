use std::ops::{Add, Div, Mul, Rem, RemAssign, ShrAssign, Sub};

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + Clone + PartialEq + PartialOrd + Rem<Output = T> + From<usize>,
{
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0.into() {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy
        + Clone
        + PartialEq
        + PartialOrd
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + From<usize>,
{
    // LCM = a*b / gcd
    a * (b / gcd(a, b))
}

pub fn egcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Copy
        + Clone
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + Rem<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + From<u8>,
{
    if a == 0.into() {
        (b, 0.into(), 1.into())
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn modinverse<T>(a: T, m: T) -> Option<T>
where
    T: Copy
        + Clone
        + PartialEq
        + PartialOrd
        + Rem<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + From<u8>,
{
    let (g, x, _) = egcd(a, m);
    if g != 1.into() {
        None
    } else {
        Some((x % m + m) % m)
    }
}

pub fn modexp<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: Copy
        + Clone
        + PartialEq
        + PartialOrd
        + Rem<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Mul<Output = T>
        + RemAssign
        + Div<Output = T>
        + ShrAssign
        + From<u8>,
{
    let mut result = 1.into();
    base %= modulus;

    loop {
        if exp <= 0.into() {
            break;
        }

        if exp % 2.into() == 1.into() {
            result = (result * base) % modulus;
        }

        exp >>= 1.into();
        base = (base * base) % modulus;
    }

    result
}
