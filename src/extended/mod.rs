use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        flat_mod!(x86);
    } else {
        pub type f80 = f64;
    }
}