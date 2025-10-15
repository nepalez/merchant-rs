use crate::AccountType;
use std::fmt::Display;
use zeroize_derive::ZeroizeOnDrop;

/// Categories of users
///
/// # Data Protection
/// This is a commonly used classifier requiring no security protection.
///
/// Consequently, both `Debug` and `Display` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub enum CustomerCategory {
    Individual,
    Company,
}

impl Display for CustomerCategory {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
