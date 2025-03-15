# Notes

## *Integers and Floating point numbers*
- Operations on number follow infix notation
- **Infix Notation**: Operator is placed between operands. -> 3 + 4
- **Prefix Notation**: -> + 3 4
- **Postfix Notation**: -> 3 4 +

## *Rust types for representing scalar(single) numbers*
- **i8, i16, i32, i64**: Signed integers ranging from 8 bit to 64 bit.
- **u8, u16, u32, u64**: Unsigned integers ranging from 8 bit to 64 bit.
- **f32, f64**: Floating-point numbers in 32 bit and 64 bit variants.
- **isize, usize**: Integers that assume the CPU's "native" width. For example, in 64-bit CPU's usize and isize will be 64-bits wide.

## *Bit Pattern in Memory*
## Integer Bit Patterns in Memory (`u32`, `i8`, and `f32`)

| **Number** | **Type** | **Bit Pattern in Memory** |
|-----------|---------|-------------------------|
| 20        | `u32`   | `00000000 00000000 00000000 00010100` |
| 20        | `i8`    | `00010100` |
| 20.0      | `f32`   | `01000001 10100000 00000000 00000000` |
| 18        | `u32`   | `00000000 00000000 00000000 00010010` |
| 18        | `i8`    | `00010010` |
| 18.0      | `f32`   | `01000001 10010000 00000000 00000000` |
| -18       | `i8`    | `11101110` (Two's Complement) |
| -20       | `i8`    | `11101100` (Two's Complement) |

### **Explanation**
1. **`u32` (Unsigned 32-bit Integer)**
   - Stores the number **directly in binary**.
   - Example: `20` → `00000000 00000000 00000000 00010100` (binary for `20`).
  
2. **`i8` (Signed 8-bit Integer, Two's Complement Representation)**
   - Stores positive numbers normally.
   - Stores negative numbers using **Two’s Complement** (invert bits and add `1`).
   - Example: `-18` in `i8`:
     - `18` in binary: `00010010`
     - Invert bits: `11101101`
     - Add `1`: `11101110` (Stored as `-18`)
  
3. **`f32` (Floating-Point, 32-bit, IEEE 754)**
   - Stores the number in **sign, exponent, and mantissa**.
   - Example: `20.0`
     - Sign bit: `0` (positive)
     - Exponent: `10000011` (`4 + 127 = 131`)
     - Mantissa: `01000000000000000000000`
     - Full bit pattern: `01000001 10100000 00000000 00000000`

