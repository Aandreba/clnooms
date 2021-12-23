#![feature(asm_const)]
macro_rules! flat_mod {
    ($i:ident) => {
        mod $i;
        pub use $i::*;
    };
}

#[cfg(target_arch = "aarch64")]
flat_mod!(aarch64);