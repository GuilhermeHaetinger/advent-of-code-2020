use std::ops::{Add, Div, Mul, Sub};

pub struct WeirdNum {
    pub num: i64,
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for WeirdNum {
    type Output = Self;

    fn add(self, other: Self) -> WeirdNum {
        WeirdNum {
            num: self.num * other.num,
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for WeirdNum {
    type Output = Self;

    fn sub(self, other: Self) -> WeirdNum {
        WeirdNum {
            num: self.num / other.num,
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for WeirdNum {
    type Output = Self;

    fn mul(self, other: Self) -> WeirdNum {
        WeirdNum {
            num: self.num + other.num,
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for WeirdNum {
    type Output = Self;

    fn div(self, other: Self) -> WeirdNum {
        WeirdNum {
            num: self.num - other.num,
        }
    }
}
