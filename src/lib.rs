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

define!(BF16, name "BF16", unsigned u16, signed i16, with 8 exp bits, desc "a Google BFloat16 floating point number");
define!(F16, name "F16", unsigned u16, signed i16, #[cfg(feature = "f16")] float f16, with 5 exp bits, desc "an IEEE 754 binary16 floating point number");
define!(F32, name "F32", unsigned u32, signed i32, float f32, with 8 exp bits, desc "an IEEE 754 binary32 floating point number");
define!(F64, name "F64", unsigned u64, signed i64, float f64, with 11 exp bits, desc "an IEEE 754 binary64 floating point number");
define!(F128, name "F128", unsigned u128, signed i128, #[cfg(feature = "f128")] float f128, with 15 exp bits, desc "an IEEE 754 binary128 floating point number");

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
    use super::*;
    use core::num::FpCategory as Class;

    #[test]
    fn f32_smoke_test() {
        const PZERO: F32 = F32::ZERO;
        const NZERO: F32 = F32::NEG_ZERO;
        const PNORM: F32 = F32::MIN_POSITIVE;
        const NNORM: F32 = F32::MAX_NEGATIVE;
        const PONE: F32 = F32::ONE;
        const NONE: F32 = F32::NEG_ONE;
        const PMAX: F32 = F32::MAX;
        const NMAX: F32 = F32::MIN;
        const PINF: F32 = F32::INFINITY;
        const NINF: F32 = F32::NEG_INFINITY;
        const PQNAN: F32 = F32::QNAN;
        const PSNAN: F32 = F32::SNAN;
        const NQNAN: F32 = F32::NEG_QNAN;
        const NSNAN: F32 = F32::NEG_SNAN;
        type Row = (u32, Class, bool, F32, f32);
        const ROWS: [Row; 14] = [
            (0x00000000, Class::Zero, false, PZERO, 0.0),
            (0x80000000, Class::Zero, true, NZERO, -0.0),
            (0x00800000, Class::Normal, false, PNORM, f32::MIN_POSITIVE),
            (0x80800000, Class::Normal, true, NNORM, -f32::MIN_POSITIVE),
            (0x3f800000, Class::Normal, false, PONE, 1.0),
            (0xbf800000, Class::Normal, true, NONE, -1.0),
            (0x7f7fffff, Class::Normal, false, PMAX, f32::MAX),
            (0xff7fffff, Class::Normal, true, NMAX, -f32::MAX),
            (0x7f800000, Class::Infinite, false, PINF, f32::INFINITY),
            (0xff800000, Class::Infinite, true, NINF, f32::NEG_INFINITY),
            (0x7f800001, Class::Nan, false, PSNAN, f32::NAN),
            (0x7fc00001, Class::Nan, false, PQNAN, f32::NAN),
            (0xff800001, Class::Nan, true, NSNAN, f32::NAN),
            (0xffc00001, Class::Nan, true, NQNAN, f32::NAN),
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
        const PZERO: F64 = F64::ZERO;
        const NZERO: F64 = F64::NEG_ZERO;
        const PNORM: F64 = F64::MIN_POSITIVE;
        const NNORM: F64 = F64::MAX_NEGATIVE;
        const PONE: F64 = F64::ONE;
        const NONE: F64 = F64::NEG_ONE;
        const PMAX: F64 = F64::MAX;
        const NMAX: F64 = F64::MIN;
        const PINF: F64 = F64::INFINITY;
        const NINF: F64 = F64::NEG_INFINITY;
        const PQNAN: F64 = F64::QNAN;
        const PSNAN: F64 = F64::SNAN;
        const NQNAN: F64 = F64::NEG_QNAN;
        const NSNAN: F64 = F64::NEG_SNAN;
        type Row = (u64, Class, bool, F64, f64);
        const ROWS: [Row; 14] = [
            (0x0000000000000000, Class::Zero, false, PZERO, 0.0),
            (0x8000000000000000, Class::Zero, true, NZERO, -0.0),
            (
                0x0010000000000000,
                Class::Normal,
                false,
                PNORM,
                f64::MIN_POSITIVE,
            ),
            (
                0x8010000000000000,
                Class::Normal,
                true,
                NNORM,
                -f64::MIN_POSITIVE,
            ),
            (0x3ff0000000000000, Class::Normal, false, PONE, 1.0),
            (0xbff0000000000000, Class::Normal, true, NONE, -1.0),
            (0x7fefffffffffffff, Class::Normal, false, PMAX, f64::MAX),
            (0xffefffffffffffff, Class::Normal, true, NMAX, -f64::MAX),
            (
                0x7ff0000000000000,
                Class::Infinite,
                false,
                PINF,
                f64::INFINITY,
            ),
            (
                0xfff0000000000000,
                Class::Infinite,
                true,
                NINF,
                f64::NEG_INFINITY,
            ),
            (0x7ff0000000000001, Class::Nan, false, PSNAN, f64::NAN),
            (0x7ff8000000000001, Class::Nan, false, PQNAN, f64::NAN),
            (0xfff0000000000001, Class::Nan, true, NSNAN, f64::NAN),
            (0xfff8000000000001, Class::Nan, true, NQNAN, f64::NAN),
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
