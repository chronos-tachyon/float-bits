use core::num::FpCategory;

pub const fn is_zero(category: FpCategory) -> bool {
    matches!(category, FpCategory::Zero)
}

pub const fn is_subnormal(category: FpCategory) -> bool {
    matches!(category, FpCategory::Subnormal)
}

pub const fn is_normal(category: FpCategory) -> bool {
    matches!(category, FpCategory::Normal)
}

pub const fn is_infinite(category: FpCategory) -> bool {
    matches!(category, FpCategory::Infinite)
}

pub const fn is_nan(category: FpCategory) -> bool {
    matches!(category, FpCategory::Nan)
}

pub const fn is_finite(category: FpCategory) -> bool {
    matches!(
        category,
        FpCategory::Zero | FpCategory::Subnormal | FpCategory::Normal
    )
}
