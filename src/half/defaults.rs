use std::fmt::{Write, Display, Debug};
use crate::utils::FlatMapResult;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct f16 (pub(crate) u16);

impl f16 {
    pub const ZERO : f16 = f16(0);
    pub const ONE : f16 = f16(0x3c00);
    pub const TWO : f16 = f16(0x4000);

    pub const ONE_HALF : f16 = f16(0x4000);

    pub const DIGITS : u32 = 3;

    pub fn of_bits (bits: u16) -> f16 {
        f16(bits)
    }

    pub fn fract (self) -> f16 {
        self - self.trunc()
    }

    pub fn signum (self) -> f16 {
        if self == Self::ZERO {
            return Self::ZERO
        }
        
        if (self.0 >> 15) == 1 { -Self::ONE } else { Self::ONE }
    }

    pub fn copysign (self, sign: f16) -> f16 {
        let same = (sign.0 >> 15) == (self.0 >> 15);
        if same { self } else { -self }
    }
}

// DISPLAY
impl Display for f16 {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let clone = *self;
        let mut int : i32 = clone.into();

        let ten = 10u8.into();
        let mut fract = clone - int.into();

        let mut chars = [u16::default(); 3];
        for i in 0..3 {
            fract *= ten;
            let digit : u16 = fract.into();

            fract -= digit.into();
            chars[i] = digit;
        }

        let next_digit : u16 = (fract * ten).into();
        if next_digit >= 5 {
            chars[2] += 1;
        }

        let mut i = 2;
        while chars[i] == 10 {
            let next = i - 1;

            chars[i] = 0;
            chars[next] += 1;
            
            i = next;
        }

        if chars[0] == 10 {
            chars[0] = 0;
            int += 1;
        }

        std::fmt::Display::fmt(&int, f)
            .flat_map(|_| f.write_char('.'))
            .flat_map(|_| std::fmt::Display::fmt(&chars[0], f))
            .flat_map(|_| std::fmt::Display::fmt(&chars[1], f))
            .flat_map(|_| std::fmt::Display::fmt(&chars[2], f))
    }
}

impl Debug for f16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}