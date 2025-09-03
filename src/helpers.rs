use core::num::FpCategory;

pub(crate) const C_ZERO: FpCategory = FpCategory::Zero;
pub(crate) const C_SUB: FpCategory = FpCategory::Subnormal;
pub(crate) const C_NORM: FpCategory = FpCategory::Normal;
pub(crate) const C_INF: FpCategory = FpCategory::Infinite;
pub(crate) const C_NAN: FpCategory = FpCategory::Nan;

pub(crate) const fn is_zero(category: FpCategory) -> bool {
    matches!(category, C_ZERO)
}

pub(crate) const fn is_subnormal(category: FpCategory) -> bool {
    matches!(category, C_SUB)
}

pub(crate) const fn is_normal(category: FpCategory) -> bool {
    matches!(category, C_NORM)
}

pub(crate) const fn is_infinite(category: FpCategory) -> bool {
    matches!(category, C_INF)
}

pub(crate) const fn is_nan(category: FpCategory) -> bool {
    matches!(category, C_NAN)
}

pub(crate) const fn is_finite(category: FpCategory) -> bool {
    matches!(category, C_ZERO | C_SUB | C_NORM)
}
