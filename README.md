# Clnooms
Assembly-level implementation of various interesting data types on Rust

# Table of Contents
- [Data Types](#dt)
    - [Half-precision floating point (f16)](#dt-f16)
    - [Extended floating point (f80)](#dt-f80)
- [Progress](#prg)
    - [Half-precision floating point (f16)](#prg-f16)
    - [Extended floating point (f80)](#prg-f80)
- [Installation](#inst)

# Data types # {#dt}
## Half-precision floating point (f16) ## {#dt-f16}

A 16-bit floating point type; specifically, the ARM alternative half-precision.
It is only available for aarch64 targets & it differs from it's IEEE variant by doing away with the special case for an exponent value of 31, disallawing infinities & NaNs.
For non-compatible targets, the "f16" type is an alias of "f32"<br>

See the <a href="https://en.wikipedia.org/wiki/Half-precision_floating-point_format">Wikipedia article</a>

## Extended floating point (f80) ## {#dt-f80}

A 80-bit floating point type that provides greater precision than the basic floating-point formats.
It is an x86 exclusive and it's mainly used to minimize roundoff and overflow errors in smaller floating point types.
For non-compatible targets, the "f80" type is an alias of "f64"<br>

See the <a href="https://en.wikipedia.org/wiki/Extended_precision">Wikipedia article</a>

# Progress # {#prg}

## Half-precision floating point (f16) ## {#prg-f16}
### Arithmetic
- [x] Addition
- [x] Subtraction
- [x] Multiplication
- [x] Division
- [x] Negation
- [x] Square root

### Exponential
- [ ] Exponential
- [ ] Natural Logarithm

### Trigonometry
- [ ] Sine
- [ ] Cosine
- [ ] Tangent
- [ ] Arc Sine
- [ ] Arc Cosine
- [ ] Arc Tangent
- [ ] Arc Tangent 2

### Comparing
- [x] By Equal
- [x] By Order

### From Casting
- [x] 8-bit unsigned integer
- [x] 8-bit signed integer
- [x] 16-bit unsigned integer
- [x] 16-bit signed integer
- [x] 32-bit unsigned integer
- [x] 32-bit signed integer
- [x] 64-bit unsigned integer
- [x] 64-bit signed integer
- [ ] 128-bit unsigned integer
- [ ] 128-bit signed integer
- [x] Single precision floating point
- [x] Double precision floating point

### Into Casting
- [ ] 8-bit unsigned integer
- [ ] 8-bit signed integer
- [x] 16-bit unsigned integer
- [ ] 16-bit signed integer
- [ ] 32-bit unsigned integer
- [x] 32-bit signed integer
- [ ] 64-bit unsigned integer
- [ ] 64-bit signed integer
- [ ] 128-bit unsigned integer
- [ ] 128-bit signed integer
- [x] Single precision floating point
- [x] Double precision floating point

### Miscelaneous
- [x] Absolute
- [x] Round
- [x] Truncate
- [x] Floor
- [x] Ceil

## Extended floating point (f80) ## {#prg-f80}
### Arithmetic
- [x] Addition
- [x] Subtraction
- [x] Multiplication
- [x] Division
- [x] Negation
- [x] Square root

### Exponential
- [ ] Exponential
- [ ] Natural Logarithm

### Trigonometry
- [x] Sine
- [x] Cosine
- [x] Tangent
- [ ] Arc Sine
- [ ] Arc Cosine
- [x] Arc Tangent
- [x] Arc Tangent 2

### Comparing
- [ ] By Equal
- [ ] By Order

### From Casting
- [ ] 8-bit unsigned integer
- [ ] 8-bit signed integer
- [ ] 16-bit unsigned integer
- [ ] 16-bit signed integer
- [ ] 32-bit unsigned integer
- [ ] 32-bit signed integer
- [ ] 64-bit unsigned integer
- [ ] 64-bit signed integer
- [ ] 128-bit unsigned integer
- [ ] 128-bit signed integer
- [x] Single precision floating point
- [x] Double precision floating point

### Into Casting
- [ ] 8-bit unsigned integer
- [ ] 8-bit signed integer
- [ ] 16-bit unsigned integer
- [ ] 16-bit signed integer
- [ ] 32-bit unsigned integer
- [] 32-bit signed integer
- [ ] 64-bit unsigned integer
- [ ] 64-bit signed integer
- [ ] 128-bit unsigned integer
- [ ] 128-bit signed integer
- [x] Single precision floating point
- [x] Double precision floating point

### Miscelaneous
- [x] Absolute
- [x] Round
- [ ] Truncate
- [ ] Floor
- [ ] Ceil

# Instalation # {#inst}
This library can be installed through cargo with the command<br>
```cargo install clnooms```