//! Floats stored as raw bits, making them hashable and totally ordered.
//!
//! The types in this crate represent IEEE 754 binary floating point numbers, using unsigned
//! integers to store the raw bits of the floats.  Some of these types represent float formats not
//! supported by Rust, or only if specific Rust features are available and enabled.  A limited
//! suite of operations are available that operate directly on the raw bit representation,
//! bypassing the need for Rust support.
//!
//! # Example
//!
//! ```rust
//! # use float_bits::F64;
//!
//! let x: f64 = 0.1;
//! let y: F64 = x.into();
//! let z: f64 = y.into();
//! assert_eq!(x, z);
//! assert_eq!(0x3fb999999999999a, y.to_bits());
//! ```

#![no_std]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]
#![allow(missing_docs)]

#[macro_use]
mod macros;

mod helpers;

define! {
    #[doc = "A newtype containing the raw bits of a Google BFloat16 floating point number."]
    #[doc = ""]
    #[doc = "Values of this type are hashable and have a well-defined total order: the one given "]
    #[doc = "by [`Self::total_cmp`].  As a consequence, `+0.0` is not equal to `-0.0`, and NaN "]
    #[doc = "compares equal to NaN if both NaN values have exactly the same bit pattern."]
    pub struct BF16;
    size 16 bits;
    exp 8 bits;
    repr u16 / i16;
}

define! {
    #[doc = "A newtype containing the raw bits of an IEEE 754 binary16 floating point number."]
    #[doc = ""]
    #[doc = "Values of this type are hashable and have a well-defined total order: the one given "]
    #[doc = "by [`Self::total_cmp`].  As a consequence, `+0.0` is not equal to `-0.0`, and NaN "]
    #[doc = "compares equal to NaN if both NaN values have exactly the same bit pattern."]
    #[doc = ""]
    #[doc = "# Features"]
    #[doc = ""]
    #[doc = "Crate feature `f16` enables use of the Rust [`f16`] primitive type, which is a"]
    #[doc = "nightly-only language feature that's only functional on certain architectures."]
    pub struct F16;
    size 16 bits;
    exp 5 bits;
    repr u16 / i16;
    float f16 with feature "f16";
}

define! {
    #[doc = "A newtype containing the raw bits of an IEEE 754 binary32 floating point number."]
    #[doc = ""]
    #[doc = "Values of this type are hashable and have a well-defined total order: the one given "]
    #[doc = "by [`Self::total_cmp`].  As a consequence, `+0.0` is not equal to `-0.0`, and NaN "]
    #[doc = "compares equal to NaN if both NaN values have exactly the same bit pattern."]
    pub struct F32;
    size 32 bits;
    exp 8 bits;
    repr u32 / i32;
    float f32;
}

define! {
    #[doc = "A newtype containing the raw bits of an IEEE 754 binary64 floating point number."]
    #[doc = ""]
    #[doc = "Values of this type are hashable and have a well-defined total order: the one given "]
    #[doc = "by [`Self::total_cmp`].  As a consequence, `+0.0` is not equal to `-0.0`, and NaN "]
    #[doc = "compares equal to NaN if both NaN values have exactly the same bit pattern."]
    pub struct F64;
    size 64 bits;
    exp 11 bits;
    repr u64 / i64;
    float f64;
}

define! {
    #[doc = "A newtype containing the raw bits of an IEEE 754 binary128 floating point number."]
    #[doc = ""]
    #[doc = "Values of this type are hashable and have a well-defined total order: the one given "]
    #[doc = "by [`Self::total_cmp`].  As a consequence, `+0.0` is not equal to `-0.0`, and NaN "]
    #[doc = "compares equal to NaN if both NaN values have exactly the same bit pattern."]
    #[doc = ""]
    #[doc = "# Features"]
    #[doc = ""]
    #[doc = "Crate feature `f128` enables use of the Rust [`f128`] primitive type, which is a"]
    #[doc = "nightly-only language feature that's only functional on certain architectures."]
    pub struct F128;
    size 128 bits;
    exp 15 bits;
    repr u128 / i128;
    float f128 with feature "f128";
}

impl core::fmt::Display for F64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val = self.to_float();
        core::fmt::Display::fmt(&val, f)
    }
}

impl core::str::FromStr for F64 {
    type Err = core::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_float(s.parse()?))
    }
}

impl core::fmt::Display for F32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val = self.to_float();
        core::fmt::Display::fmt(&val, f)
    }
}

impl core::str::FromStr for F32 {
    type Err = core::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_float(s.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use core::num::FpCategory;

    use super::*;
    use crate::helpers::{C_INF, C_NAN, C_NORM, C_ZERO};

    #[test]
    fn f32_smoke_test() {
        const P_ZERO: F32 = F32::ZERO;
        const N_ZERO: F32 = F32::NEG_ZERO;
        const P_TINY: F32 = F32::MIN_POSITIVE;
        const N_TINY: F32 = F32::MAX_NEGATIVE;
        const P_ONE: F32 = F32::ONE;
        const N_ONE: F32 = F32::NEG_ONE;
        const P_MAX: F32 = F32::MAX;
        const N_MAX: F32 = F32::MIN;
        const P_INF: F32 = F32::INFINITY;
        const N_INF: F32 = F32::NEG_INFINITY;
        const P_QNAN: F32 = F32::QNAN;
        const P_SNAN: F32 = F32::SNAN;
        const N_QNAN: F32 = F32::NEG_QNAN;
        const N_SNAN: F32 = F32::NEG_SNAN;

        type Row = (u32, FpCategory, bool, F32, f32);
        const ROWS: [Row; 14] = [
            (0x00000000, C_ZERO, false, P_ZERO, 0.0),
            (0x80000000, C_ZERO, true, N_ZERO, -0.0),
            (0x00800000, C_NORM, false, P_TINY, f32::MIN_POSITIVE),
            (0x80800000, C_NORM, true, N_TINY, -f32::MIN_POSITIVE),
            (0x3f800000, C_NORM, false, P_ONE, 1.0),
            (0xbf800000, C_NORM, true, N_ONE, -1.0),
            (0x7f7fffff, C_NORM, false, P_MAX, f32::MAX),
            (0xff7fffff, C_NORM, true, N_MAX, -f32::MAX),
            (0x7f800000, C_INF, false, P_INF, f32::INFINITY),
            (0xff800000, C_INF, true, N_INF, f32::NEG_INFINITY),
            (0x7f800001, C_NAN, false, P_SNAN, f32::NAN),
            (0x7fc00001, C_NAN, false, P_QNAN, f32::NAN),
            (0xff800001, C_NAN, true, N_SNAN, f32::NAN),
            (0xffc00001, C_NAN, true, N_QNAN, f32::NAN),
        ];
        for (bits, class, neg, val, float) in ROWS {
            assert_eq!(neg, val.is_sign_negative());
            assert_eq!(class, val.classify());
            assert_eq!(float.is_nan(), val.is_nan());
            if !val.is_nan() {
                assert_eq!(float, val.to_float());
            }
            assert_eq!(bits, val.to_bits());
        }
    }

    #[test]
    fn f64_smoke_test() {
        const P_ZERO: F64 = F64::ZERO;
        const N_ZERO: F64 = F64::NEG_ZERO;
        const P_TINY: F64 = F64::MIN_POSITIVE;
        const N_TINY: F64 = F64::MAX_NEGATIVE;
        const P_ONE: F64 = F64::ONE;
        const N_ONE: F64 = F64::NEG_ONE;
        const P_MAX: F64 = F64::MAX;
        const N_MAX: F64 = F64::MIN;
        const P_INF: F64 = F64::INFINITY;
        const N_INF: F64 = F64::NEG_INFINITY;
        const P_QNAN: F64 = F64::QNAN;
        const P_SNAN: F64 = F64::SNAN;
        const N_QNAN: F64 = F64::NEG_QNAN;
        const N_SNAN: F64 = F64::NEG_SNAN;

        type Row = (u64, FpCategory, bool, F64, f64);
        const ROWS: [Row; 14] = [
            (0x0000000000000000, C_ZERO, false, P_ZERO, 0.0),
            (0x8000000000000000, C_ZERO, true, N_ZERO, -0.0),
            (0x0010000000000000, C_NORM, false, P_TINY, f64::MIN_POSITIVE),
            (0x8010000000000000, C_NORM, true, N_TINY, -f64::MIN_POSITIVE),
            (0x3ff0000000000000, C_NORM, false, P_ONE, 1.0),
            (0xbff0000000000000, C_NORM, true, N_ONE, -1.0),
            (0x7fefffffffffffff, C_NORM, false, P_MAX, f64::MAX),
            (0xffefffffffffffff, C_NORM, true, N_MAX, -f64::MAX),
            (0x7ff0000000000000, C_INF, false, P_INF, f64::INFINITY),
            (0xfff0000000000000, C_INF, true, N_INF, f64::NEG_INFINITY),
            (0x7ff0000000000001, C_NAN, false, P_SNAN, f64::NAN),
            (0x7ff8000000000001, C_NAN, false, P_QNAN, f64::NAN),
            (0xfff0000000000001, C_NAN, true, N_SNAN, f64::NAN),
            (0xfff8000000000001, C_NAN, true, N_QNAN, f64::NAN),
        ];
        for (bits, class, neg, val, float) in ROWS {
            assert_eq!(neg, val.is_sign_negative());
            assert_eq!(class, val.classify());
            assert_eq!(float.is_nan(), val.is_nan());
            if !val.is_nan() {
                assert_eq!(float, val.to_float());
            }
            assert_eq!(bits, val.to_bits());
        }
    }
}
