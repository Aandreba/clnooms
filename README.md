# Clnooms
Assembly-level implementation of various interesting data types on Rust

# Half precision floating point (f16)

A 16-bit floating point type; specifically, the ARM alternative half-precision.
It is only available for aarch64 targets & it differs from it's IEEE variant by doing away with the special case for an exponent value of 31, disallawing infinities & NaNs.
For non-compatible targets, the "f16" type is an alias of "f32"<br>

See the <a href="https://en.wikipedia.org/wiki/Half-precision_floating-point_format">Wikipedia article</a>

# Extended floating point (f80)

A 80-bit floating point type that provides greater precision than the basic floating-point formats.
It is an x86 exclusive and it's mainly used to minimize roundoff and overflow errors in smaller floating point types.
For non-compatible targets, the "f80" type is an alias of "f64"<br>

See the <a href="https://en.wikipedia.org/wiki/Extended_precision">Wikipedia article</a>

# Instalation
This library can be installed through cargo with this command<br>
```cargo install clnooms```