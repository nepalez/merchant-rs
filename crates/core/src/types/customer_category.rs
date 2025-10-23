use crate::AccountType;
use std::fmt::Display;
use zeroize_derive::ZeroizeOnDrop;

/// Categories of users
///
/// # Data Protection
/// This is a public value, neither secret nor even PII.
/// Customer categories are standardized classifiers requiring no security protection.
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
