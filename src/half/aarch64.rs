use std::{arch::asm, ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign}, cmp::Ordering};

use super::f16;

impl f16 {
    /// Computes the value's absolute
    pub fn abs (self) -> f16 {
        unsafe {
            let result : u16;
            asm!("fabs {0:h}, {1:h}", out(vreg) result, in(vreg) self.0);
            Self(result)
        }
    }

    /// Computes the value's square root
    pub fn sqrt (self) -> f16 {
        unsafe {
            let result : u16;
            asm!("fsqrt {0:h}, {1:h}", out(vreg) result, in(vreg) self.0);
            Self(result)
        }
    }

    /// Returns the largest integer less than or equal to a number
    pub fn floor (self) -> f16 {
        unsafe {
            let result: u16;
            asm!("frintm {0:h}, {1:h}", out(vreg) result, in(vreg) self.0);
            Self(result)
        }
    }

    /// Returns the smallest integer less than or equal to a number
    pub fn ceil (self) -> f16 {
        unsafe {
            let result: u16;
            asm!("frintp {0:h}, {1:h}", out(vreg) result, in(vreg) self.0);
            Self(result)
        }
    }

    /// Returns the nearest integer to a number. Round half-way cases away from 0.0
    pub fn round (self) -> f16 {
        unsafe {
            let result: u16;
            asm!("frintz {0:h}, {1:h}", out(vreg) result, in(vreg) self.0);
            Self(result)
        }
    }
    
    /// Returns the integer part of a number
    pub fn trunc (self) -> f16 {
        unsafe {
            let result: u16;
            asm!("frintx {0:h}, {1:h}", out(vreg) result, in(vreg) self.0);
            Self(result)
        }
    }

    /// First multiplies self x d1, then adds d2 to that result, returning the result
    pub fn mul_add (self, d1: f16, d2: f16) -> f16 {
        unsafe {
            let d0 : u16;
            asm!("fmadd {0:h}, {1:h}, {2:h}, {3:h}", out(vreg) d0, in(vreg) self.0, in(vreg) d1.0, in(vreg) d2.0);
            Self(d0)
        }
    }

    /// First multiplies self x d1, negates the product, then adds d2 to that result, returning the result
    pub fn mul_sub (self, d1: f16, d2: f16) -> f16 {
        unsafe {
            let d0 : u16;
            asm!("fmsub {0:h}, {1:h}, {2:h}, {3:h}", out(vreg) d0, in(vreg) self.0, in(vreg) d1.0, in(vreg) d2.0);
            Self(d0)
        }
    }
}

// ARITHMETIC
impl Add for f16 {
    type Output = Self;

    #[inline(always)]
    fn add (self, rhs: Self) -> Self {
        unsafe {
            let result : u16;
            asm!("fadd {0:h}, {1:h}, {2:h}", out(vreg) result, in(vreg) self.0, in(vreg) rhs.0);
            Self(result)
        }
    }
}

impl AddAssign for f16 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (*self + rhs).0
    }
}

impl Sub for f16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        unsafe {
            let result : u16;
            asm!("fsub {0:h}, {1:h}, {2:h}", out(vreg) result, in(vreg) self.0, in(vreg) rhs.0);
            Self(result)
        }
    }
}

impl SubAssign for f16 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = (*self - rhs).0
    }
}

impl Mul for f16 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unsafe {
            let result : u16;
            asm!("fmul {0:h}, {1:h}, {2:h}", out(vreg) result, in(vreg) self.0, in(vreg) rhs.0);
            Self(result)
        }
    }
}

impl MulAssign for f16 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (*self * rhs).0
    }
}

impl Div for f16 {
    type Output = Self;

    fn div (self, rhs: Self) -> Self {
        unsafe {
            let result : u16;
            asm!("fdiv {0:h}, {1:h}, {2:h}", out(vreg) result, in(vreg) self.0, in(vreg) rhs.0);
            Self(result)
        }
    }
}

impl DivAssign for f16 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 = (*self / rhs).0
    }
}

impl Neg for f16 {
    type Output = Self;

    fn neg(self) -> Self {
        unsafe {
            let result : u16;
            asm!("fneg {0:h}, {1:h}", out(vreg) result, in(vreg) self.0);
            Self(result)
        }
    }
}

// COMPARE
impl PartialOrd for f16 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for f16 {
    fn cmp (&self, other: &Self) -> std::cmp::Ordering {
        return if self == other { Ordering::Equal } else { if self.min(other) == self { Ordering::Less } else { Ordering::Greater } }
    }

    fn min (self, other: Self) -> Self where Self: Sized {
        unsafe {
            let result : u16;
            asm!("fmin {0:h}, {1:h}, {2:h}", out(vreg) result, in(vreg) self.0, in(vreg) other.0);
            Self(result)
        }
    }

    fn max (self, other: Self) -> Self where Self: Sized {
        unsafe {
            let result : u16;
            asm!("fmax {0:h}, {1:h}, {2:h}", out(vreg) result, in(vreg) self.0, in(vreg) other.0);
            Self(result)
        }
    }
}

// CASTING
impl From<u8> for f16 {
    fn from (x: u8) -> Self {
        unsafe {
            let result: u16;
            asm!("ucvtf {0:h}, {1}", out(vreg) result, in(reg) x);
            Self(result)
        }
    }
}

impl From<u16> for f16 {
    fn from (x: u16) -> Self {
        unsafe {
            let result: u16;
            asm!("ucvtf {0:h}, {1}", out(vreg) result, in(reg) x);
            Self(result)
        }
    }
}

impl From<u32> for f16 {
    fn from (x: u32) -> Self {
        unsafe {
            let result: u16;
            asm!("ucvtf {0:h}, {1}", out(vreg) result, in(reg) x);
            Self(result)
        }
    }
}

impl From<u64> for f16 {
    fn from (x: u64) -> Self {
        unsafe {
            let result: u16;
            asm!("ucvtf {0:h}, {1}", out(vreg) result, in(reg) x);
            Self(result)
        }
    }
}

impl From<i8> for f16 {
    fn from (x: i8) -> Self {
        unsafe {
            let result: u16;
            asm!("scvtf {0:h}, {1}", out(vreg) result, in(reg) x);
            Self(result)
        }
    }
}

impl From<i16> for f16 {
    fn from (x: i16) -> Self {
        unsafe {
            let result: u16;
            asm!("scvtf {0:h}, {1}", out(vreg) result, in(reg) x);
            Self(result)
        }
    }
}

impl From<i32> for f16 {
    fn from (x: i32) -> Self {
        unsafe {
            let result: u16;
            asm!("scvtf {0:h}, {1}", out(vreg) result, in(reg) x);
            Self(result)
        }
    }
}

impl From<i64> for f16 {
    fn from (x: i64) -> Self {
        unsafe {
            let result: u16;
            asm!("scvtf {0:h}, {1}", out(vreg) result, in(reg) x);
            Self(result)
        }
    }
}

impl From<f32> for f16 {
    fn from(x: f32) -> Self {
        unsafe {
            let result: u16;
            asm!("fcvt {0:h}, {1:s}", out(vreg) result, in(vreg) x);
            Self(result)
        }
    }
}

impl From<f64> for f16 {
    fn from(x: f64) -> Self {
        unsafe {
            let result: u16;
            asm!("fcvt {0:h}, {1:d}", out(vreg) result, in(vreg) x);
            Self(result)
        }
    }
}

impl Into<u16> for f16 {
    fn into(self) -> u16 {
        unsafe {
            let result: u16;
            asm!("fcvtzu {0}, {1:h}", out(reg) result, in(vreg) self.0);
            result
        }
    }
}

impl Into<i32> for f16 {
    fn into(self) -> i32 {
        unsafe {
            let result: i32;
            asm!("fcvtzs {0}, {1:h}", out(reg) result, in(vreg) self.0);
            result
        }
    }
}

impl Into<f32> for f16 {
    fn into(self) -> f32 {
        unsafe {
            let result: f32;
            asm!("fcvt {0:s}, {1:h}", out(vreg) result, in(vreg) self.0);
            result
        }
    }
}

impl Into<f64> for f16 {
    fn into(self) -> f64 {
        unsafe {
            let result: f64;
            asm!("fcvt {0:d}, {1:h}", out(vreg) result, in(vreg) self.0);
            result
        }
    }
}