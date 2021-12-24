use std::{arch::asm, alloc::{Layout, alloc}, ptr::addr_of, fmt::{Display, Debug}, ops::{Add, Sub, Mul, Div, Neg}};

// For FPU, use reg + pointers
// For SSE, use xmm_reg
// For AVX, use ymm_reg
// For AVX512, use zmm_reg

macro_rules! priv_single_call {
    ($f:ident, $n:literal) => {
        fn $f (self) -> f80 {
            unsafe {
                let ptr = addr_of!(self.0);
                let mut out = Self::allocate();

                asm!(
                    "fld tbyte ptr [{0}]",
                    $n,
                    "fstp tbyte ptr [{1}]",
                    in(reg) ptr,
                    out(reg) out
                );

                f80(out.read())
            }
        }
    };
}

macro_rules! single_call {
    ($f:ident, $n:literal) => {
        pub fn $f (self) -> f80 {
            unsafe {
                let ptr = addr_of!(self.0);
                let mut out = Self::allocate();

                asm!(
                    "fld tbyte ptr [{0}]",
                    $n,
                    "fstp tbyte ptr [{1}]",
                    in(reg) ptr,
                    out(reg) out
                );

                f80(out.read())
            }
        }
    };
}

#[derive(Clone, Copy)]
pub struct f80 ([u8;10]);

impl f80 {
    pub fn of_bits (bits: [u8;10]) -> f80 {
        f80(bits)
    }

    const LAYPUT : Layout = Layout::new::<[u8;10]>();
    unsafe fn allocate () -> *mut [u8;10] {
        alloc(Self::LAYPUT) as *mut [u8;10]
    }
}

// ARITHMETIC
impl Add for f80 {
    type Output = f80;

    fn add (self, rhs: Self) -> Self::Output {
        unsafe {
            let ptr1 = addr_of!(self.0);
            let ptr2 = addr_of!(rhs.0);
            let mut out = Self::allocate();

            asm!(
                "fld tbyte ptr [{0}]",
                "fld tbyte ptr [{1}]",
                "faddp st(1), st",
                "fstp tbyte ptr [{2}]",
                in(reg) ptr1,
                in(reg) ptr2,
                out(reg) out
            );

            f80(out.read())
        }
    }
}

impl Sub for f80 {
    type Output = f80;

    fn sub (self, rhs: Self) -> Self::Output {
        unsafe {
            let ptr1 = addr_of!(self.0);
            let ptr2 = addr_of!(rhs.0);
            let mut out = Self::allocate();

            asm!(
                "fld tbyte ptr [{0}]",
                "fld tbyte ptr [{1}]",
                "fsubp st(1), st",
                "fstp tbyte ptr [{2}]",
                in(reg) ptr1,
                in(reg) ptr2,
                out(reg) out
            );

            f80(out.read())
        }
    }
}

impl Mul for f80 {
    type Output = f80;

    fn mul (self, rhs: Self) -> Self::Output {
        unsafe {
            let ptr1 = addr_of!(self.0);
            let ptr2 = addr_of!(rhs.0);
            let mut out = Self::allocate();

            asm!(
                "fld tbyte ptr [{0}]",
                "fld tbyte ptr [{1}]",
                "fmulp st(1), st",
                "fstp tbyte ptr [{2}]",
                in(reg) ptr1,
                in(reg) ptr2,
                out(reg) out
            );

            f80(out.read())
        }
    }
}

impl Div for f80 {
    type Output = f80;

    fn div (self, rhs: Self) -> Self::Output {
        unsafe {
            let ptr1 = addr_of!(self.0);
            let ptr2 = addr_of!(rhs.0);
            let mut out = Self::allocate();

            asm!(
                "fld tbyte ptr [{0}]",
                "fld tbyte ptr [{1}]",
                "fdivp st(1), st",
                "fstp tbyte ptr [{2}]",
                in(reg) ptr1,
                in(reg) ptr2,
                out(reg) out
            );

            f80(out.read())
        }
    }
}

impl Neg for f80 {
    type Output = f80;
    priv_single_call!(neg, "fchs");
}

// OTHERS
impl f80 {
    /// Computes the absolute value of self. Returns NAN if the number is NAN
    single_call!(abs, "fabs");

    /// Computes the value's square root
    single_call!(sqrt, "fsqrt");

    /// Computes the value's sine
    single_call!(sin, "fsin");

    /// Computes the value's cosine
    single_call!(cos, "fcos");

    /// Returns the nearest integer to a number. Round half-way cases away from 0.0
    single_call!(round, "frndint");

    /// Computes the value's sine & cosine simultaneously
    pub fn sin_cos (self) -> (f80, f80) {
        unsafe {
            let ptr = addr_of!(self.0);
            let mut sin = Self::allocate();
            let mut cos = Self::allocate();

            asm!(
                "fld tbyte ptr [{0}]",
                "fsincos",
                "fstp tbyte ptr [{2}]",
                "fstp tbyte ptr [{1}]",
                in(reg) ptr,
                lateout(reg) sin,
                out(reg) cos
            );

            (f80(sin.read()), f80(cos.read()))
        }
    }

    /// Computes the value's tangent
    pub fn tan (self) -> f80 {
        unsafe {
            let ptr = addr_of!(self.0);
            let mut out = Self::allocate();

            asm!(
                "fld tbyte ptr [{0}]",
                "fptan",
                "FINCSTP",
                "fstp tbyte ptr [{1}]",
                in(reg) ptr,
                out(reg) out,
            );

            f80(out.read())
        }
    }

    /// Computes the value's arc tangent
    pub fn atan (self) -> f80 { 
        unsafe {
            let ptr = addr_of!(self.0);
            let mut out = Self::allocate();

            asm!(
                "fld tbyte ptr [{0}]",
                "fld1",
                "fpatan",
                "fstp tbyte ptr [{1}]",
                in(reg) ptr,
                out(reg) out
            );

            f80(out.read())
        }
    }

    /// Computes the tangent of `self / rhs`
    pub fn atan2 (self, rhs: f80) -> f80 {
        unsafe {
            let ptr1 = addr_of!(self.0);
            let ptr2 = addr_of!(rhs.0);
            let mut out = Self::allocate();

            asm!(
                "fld tbyte ptr [{0}]",
                "fld tbyte ptr [{1}]",
                "fpatan",
                "fstp tbyte ptr [{2}]",
                in(reg) ptr1,
                in(reg) ptr2,
                out(reg) out
            );

            f80(out.read())
        }
    }
}

// CASTING
impl From<f64> for f80 {
    fn from(x: f64) -> Self {
        unsafe {
            let ptr = addr_of!(x);
            let mut out = Self::allocate();

            asm!(
                "fld qword ptr [{0}]",
                "fstp tbyte ptr [{1}]",
                in(reg) ptr,
                out(reg) out
            );

            f80(out.read())
        }
    }
}

impl From<f32> for f80 {
    fn from(x: f32) -> Self {
        unsafe {
            let ptr = addr_of!(x);
            let mut out = Self::allocate();

            asm!(
                "fld dword ptr [{0}]",
                "fstp tbyte ptr [{1}]",
                in(reg) ptr,
                out(reg) out
            );

            f80(out.read())
        }
    }
}

const F64_LAYOUT : Layout = Layout::new::<f64>();
impl Into<f64> for f80 {
    fn into(self) -> f64 {
        unsafe {
            let ptr = addr_of!(self);
            let mut out = alloc(F64_LAYOUT) as *mut f64;

            asm!(
                "fld tbyte ptr [{0}]",
                "fstp qword ptr [{1}]",
                in(reg) ptr,
                out(reg) out
            );

            out.read()
        }
    }
}

const F32_LAYOUT : Layout = Layout::new::<f32>();
impl Into<f32> for f80 {
    fn into(self) -> f32 {
        unsafe {
            let ptr = addr_of!(self);
            let mut out = alloc(F32_LAYOUT) as *mut f32;

            asm!(
                "fld tbyte ptr [{0}]",
                "fstp dword ptr [{1}]",
                in(reg) ptr,
                out(reg) out
            );

            out.read()
        }
    }
}

// DISPLAY
impl Display for f80 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let into : f64 = (*self).into();
        std::fmt::Display::fmt(&into, f)
    }
}

impl Debug for f80 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}