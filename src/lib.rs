#![feature(asm_const, div_duration)]

macro_rules! flat_mod {
    ($($i:ident),*) => {
        $(
            mod $i;
            pub use $i::*;
        )*
    };
}

mod utils;

#[doc = include_str!("../docs/half.md")]
pub mod half;

#[doc = include_str!("../docs/extended.md")]
pub mod extended;