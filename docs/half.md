# Half precision floating point (f16)

A 16-bit floating point type; specifically, the ARM alternative half-precision.
It is only available for aarch64 targets & it differs from it's IEEE variant by doing away with the special case for an exponent value of 31, disallawing infinities & NaNs.
For non-compatible targets, the "f16" type is an alias of "f32"<br>

See the <a href="https://en.wikipedia.org/wiki/Half-precision_floating-point_format">Wikipedia article</a>