macro_rules! define_head {
    ( $( #[$meta:meta] )* $vis:vis struct $ty:ident; $u_ty:ty) => {
        $( #[$meta] )*
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $ty {
            /// The raw bits representing this float value.
            pub bits: $u_ty,
        }

        impl $ty {
            /// Constructs a wrapped float from the raw float bits.
            pub const fn from_bits(bits: $u_ty) -> Self {
                Self { bits }
            }

            /// Returns the raw float bits.
            pub const fn to_bits(&self) -> $u_ty {
                self.bits
            }
        }
    };
}

macro_rules! define_mid {
    ($ty:ident;) => {};
    ($ty:ident; float $f_ty:ident;) => {
        impl $ty {
            /// Constructs a wrapped float from a Rust float.
            pub const fn from_float(float: $f_ty) -> Self {
                Self::from_bits(float.to_bits())
            }

            /// Returns the Rust float which this wrapped float represents.
            pub const fn to_float(&self) -> $f_ty {
                <$f_ty>::from_bits(self.bits)
            }
        }

        impl From<$f_ty> for $ty {
            fn from(float: $f_ty) -> Self {
                Self::from_float(float)
            }
        }

        impl From<$ty> for $f_ty {
            fn from(val: $ty) -> $f_ty {
                val.to_float()
            }
        }
    };
    ($ty:ident; float $f_ty:ident with feature $feature:literal;) => {
        #[cfg(feature = $feature)]
        impl $ty {
            /// Constructs a wrapped float from a Rust float.
            ///
            /// # Features
            ///
            /// Available only with feature: `
            #[doc = $feature]
            /// `.
            pub const fn from_float(float: $f_ty) -> Self {
                Self::from_bits(float.to_bits())
            }

            /// Returns the Rust float which this wrapped float represents.
            ///
            /// # Features
            ///
            /// Available only with feature: `
            #[doc = $feature]
            /// `.
            pub const fn to_float(&self) -> $f_ty {
                <$f_ty>::from_bits(self.bits)
            }
        }

        #[cfg(feature = $feature)]
        impl From<$f_ty> for $ty {
            fn from(float: $f_ty) -> Self {
                Self::from_float(float)
            }
        }

        #[cfg(feature = $feature)]
        impl From<$ty> for $f_ty {
            fn from(val: $ty) -> $f_ty {
                val.to_float()
            }
        }
    };
}

macro_rules! define_tail {
    ($ty:ident; $u_ty:ty; $s_ty:ty; $size_bits:literal; $exp_bits:literal) => {
        impl $ty {
            /// Number of total bits in the representation.
            pub const BITS: usize = $size_bits;

            /// Number of bits in the exponent representation.
            pub const EXP_BITS: usize = $exp_bits;

            /// Number of bits in the mantissa representation.
            pub const MANTISSA_BITS: usize = Self::BITS - Self::EXP_BITS - 1;

            /// Number of significant digits in base 2.
            ///
            /// Note that the size of the mantissa in the bitwise representation is one smaller
            /// than this, since the leading 1 is not stored explicitly.
            pub const MANTISSA_DIGITS: usize = Self::BITS - Self::EXP_BITS;

            // All bits except sign bit.
            const ABS_MASK: $u_ty = <$u_ty>::MAX >> 1;

            // Sign bit.
            const SIGN_MASK: $u_ty = !Self::ABS_MASK;

            // Exponent bits.
            const EXP_MASK: $u_ty = Self::ABS_MASK & !Self::MANT_MASK;

            // Exponent bit pattern representing 2^0.
            //
            // For 8 exponent bits, this is 0x7f shifted into the exponent slot.
            const EXP_ZERO: $u_ty = Self::EXP_MASK & (Self::EXP_MASK >> 1);

            // Exponent bit pattern representing 2^MAX.
            //
            // For 8 exponent bits, this is 0xfe shifted into the exponent slot.
            const EXP_MAX: $u_ty = Self::EXP_MASK & (Self::EXP_MASK << 1);

            // Exponent bit pattern representing 2^MIN.
            //
            // For 8 exponent bits, this is 0x01 shifted into the exponent slot.
            const EXP_MIN: $u_ty = Self::EXP_MASK & !Self::EXP_MAX;

            // Mantissa bits.
            const MANT_MASK: $u_ty = Self::ABS_MASK >> Self::EXP_BITS;

            // Most significant mantissa bit, representing `is_quiet` for NaN values.
            const QUIET_MASK: $u_ty = Self::MANT_MASK & !(Self::MANT_MASK >> 1);

            /// Positive zero (`+0.0`).
            pub const ZERO: Self = Self::from_bits(0);

            /// Positive one (`+1.0`).
            pub const ONE: Self = Self::from_bits(Self::EXP_ZERO);

            /// Positive infinity (`+∞`).
            pub const INFINITY: Self = Self::from_bits(Self::EXP_MASK);

            /// Not a Number (NaN) with sign bit 0, `is_quiet` bit 0, and arbitrary payload.
            pub const SNAN: Self = Self::from_bits(Self::EXP_MASK | 1);

            /// Not a Number (NaN) with sign bit 0, `is_quiet` bit 1, and arbitrary payload.
            pub const QNAN: Self = Self::from_bits(Self::EXP_MASK | Self::QUIET_MASK | 1);

            /// Negative zero (`−0.0`).
            pub const NEG_ZERO: Self = Self::ZERO.neg();

            /// Negative one (`−1.0`).
            pub const NEG_ONE: Self = Self::ONE.neg();

            /// Negative infinity (`−∞`).
            pub const NEG_INFINITY: Self = Self::INFINITY.neg();

            /// Not a Number (NaN) with sign bit 1, `is_quiet` bit 0, and arbitrary payload.
            pub const NEG_SNAN: Self = Self::SNAN.neg();

            /// Not a Number (NaN) with sign bit 1, `is_quiet` bit 1, and arbitrary payload.
            pub const NEG_QNAN: Self = Self::QNAN.neg();

            /// The positive normal value with the greatest possible absolute magnitude.
            pub const MAX: Self = Self::from_bits(Self::EXP_MAX | Self::MANT_MASK);

            /// The negative normal value with the greatest possible absolute magnitude.
            pub const MIN: Self = Self::MAX.neg();

            /// The positive normal value with the least possible absolute magnitude.
            pub const MIN_POSITIVE: Self = Self::from_bits(Self::EXP_MIN);

            /// The negative normal value with the least possible absolute magnitude.
            pub const MAX_NEGATIVE: Self = Self::MIN_POSITIVE.neg();

            #[doc(hidden)]
            pub const NAN: Self = Self::QNAN;

            /// Returns `true` if self has a positive sign, including `+0.0`, `+∞`, and [NaN] with positive sign bit.
            ///
            /// [NaN]: https://en.wikipedia.org/wiki/NaN
            pub const fn is_sign_positive(&self) -> bool {
                (self.bits & Self::SIGN_MASK) == 0
            }

            /// Returns `true` if self has a negative sign, including `-0.0`, `-∞`, and [NaN] with negative sign bit.
            ///
            /// [NaN]: https://en.wikipedia.org/wiki/NaN
            pub const fn is_sign_negative(&self) -> bool {
                !self.is_sign_positive()
            }

            /// Returns the floating point category of the number.
            pub const fn classify(&self) -> core::num::FpCategory {
                let exp = self.bits & Self::EXP_MASK;
                let mant = self.bits & Self::MANT_MASK;
                use core::num::FpCategory;
                match (exp, mant) {
                    (0, 0) => FpCategory::Zero,
                    (0, _) => FpCategory::Subnormal,
                    (Self::EXP_MASK, 0) => FpCategory::Infinite,
                    (Self::EXP_MASK, _) => FpCategory::Nan,
                    _ => FpCategory::Normal,
                }
            }

            /// Returns `true` if the number is `+0.0` or `-0.0`.
            pub const fn is_zero(&self) -> bool {
                crate::helpers::is_zero(self.classify())
            }

            /// Returns `true` if the number is [subnormal].
            ///
            /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
            pub const fn is_subnormal(&self) -> bool {
                crate::helpers::is_subnormal(self.classify())
            }

            /// Returns `true` if the number is neither zero, infinite, [subnormal], or [NaN].
            ///
            /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
            /// [NaN]: https://en.wikipedia.org/wiki/NaN
            pub const fn is_normal(&self) -> bool {
                crate::helpers::is_normal(self.classify())
            }

            /// Returns `true` if the number is [subnormal].
            ///
            /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
            pub const fn is_infinite(&self) -> bool {
                crate::helpers::is_infinite(self.classify())
            }

            /// Returns `true` if this value is [NaN].
            ///
            /// [NaN]: https://en.wikipedia.org/wiki/NaN
            pub const fn is_nan(&self) -> bool {
                crate::helpers::is_nan(self.classify())
            }

            /// Returns `true` if this number is neither infinite nor [NaN].
            ///
            /// [NaN]: https://en.wikipedia.org/wiki/NaN
            pub const fn is_finite(&self) -> bool {
                crate::helpers::is_finite(self.classify())
            }

            /// Computes the absolute value of `self`.
            ///
            /// The result is always exact.  The result will always test `true` with [`Self::is_sign_positive`].
            pub const fn abs(&self) -> Self {
                let bits = self.bits & Self::ABS_MASK;
                Self { bits }
            }

            /// Computes the negation of `self`.
            ///
            /// The result is always exact.
            pub const fn neg(&self) -> Self {
                let bits = self.bits ^ Self::SIGN_MASK;
                Self { bits }
            }

            /// Returns a number that represents the sign of `self`.
            ///
            /// * [`Self::ONE`] if the number is positive, including `+0.0` or `+∞`
            /// * [`Self::NEG_ONE`] if the number is negative, including `-0.0` or `-∞`
            /// * `self` if the number is [NaN]
            ///
            /// [NaN]: https://en.wikipedia.org/wiki/NaN
            pub const fn signum(&self) -> Self {
                if self.is_nan() {
                    *self
                } else if self.is_sign_negative() {
                    Self::NEG_ONE
                } else {
                    Self::ONE
                }
            }

            /// Returns a number composed of the magnitude of `self` and the sign of `sign`.
            pub const fn copysign(&self, sign: Self) -> Self {
                let self_bits = self.bits & Self::ABS_MASK;
                let sign_bit = sign.bits & Self::SIGN_MASK;
                let bits = self_bits | sign_bit;
                Self { bits }
            }

            const fn sort_bits(&self) -> $s_ty {
                let mask = if self.is_sign_negative() { Self::ABS_MASK } else { 0 };
                let bits = self.bits ^ mask;
                bits as $s_ty
            }

            /// Returns the ordering between `self` and `rhs`.
            ///
            /// Unlike the standard partial comparison between floating point numbers, this
            /// comparison always produces an ordering in accordance to the `totalOrder` predicate
            /// as defined in the IEEE 754 (2008 revision) floating point standard. The values are
            /// ordered in the following sequence:
            ///
            /// * negative quiet NaN
            /// * negative signaling NaN
            /// * negative infinity
            /// * negative numbers
            /// * negative subnormal numbers
            /// * negative zero
            /// * positive zero
            /// * positive subnormal numbers
            /// * positive numbers
            /// * positive infinity
            /// * positive signaling NaN
            /// * positive quiet NaN
            pub const fn total_cmp(&self, rhs: Self) -> core::cmp::Ordering {
                use core::cmp::Ordering;
                let lhs = self.sort_bits();
                let rhs = rhs.sort_bits();
                if lhs == rhs {
                    Ordering::Equal
                } else if lhs < rhs {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }

            /// Restrict a value to a certain interval unless it is NaN.
            ///
            /// Returns `max` if `self` is greater than `max`, and `min` if `self` is less than
            /// `min`. Otherwise this returns `self`.
            ///
            /// Note that this function returns NaN if the initial value was NaN as well.
            ///
            /// # Panics
            ///
            /// Panics if `min` > `max`, `min` is NaN, or `max` is NaN.
            pub const fn clamp(&self, min: Self, max: Self) -> Self {
                use core::cmp::Ordering;
                if min.is_nan() {
                    panic!("min is NaN")
                } else if max.is_nan() {
                    panic!("max is NaN")
                } else if matches!(min.total_cmp(max), Ordering::Greater) {
                    panic!("min > max")
                } else if self.is_nan() {
                    *self
                } else if matches!(self.total_cmp(min), Ordering::Less) {
                    min
                } else if matches!(self.total_cmp(max), Ordering::Greater) {
                    max
                } else {
                    *self
                }
            }
        }

        impl Default for $ty {
            /// Returns [`Self::ZERO`].
            fn default() -> Self {
                Self::ZERO
            }
        }

        impl PartialOrd for $ty {
            fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
                Some(Ord::cmp(self, rhs))
            }
        }

        impl Ord for $ty {
            fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
                self.total_cmp(*rhs)
            }
        }
    };
}

macro_rules! define {
    {
        $( #[$meta:meta] )* $vis:vis struct $ty:ident;
        size $size_bits:literal bits;
        exp $exp_bits:literal bits;
        repr $u_ty:ident / $s_ty:ident;
        $( $rest:tt )*
    } => {
        define_head!($( #[$meta] )* $vis struct $ty; $u_ty);
        define_mid!($ty; $( $rest )*);
        define_tail!($ty; $u_ty; $s_ty; $size_bits; $exp_bits);
    };
}
