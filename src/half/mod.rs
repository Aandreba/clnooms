use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        flat_mod!(defaults, aarch64);
    } else {
        pub type f16 = f32;
    }
}