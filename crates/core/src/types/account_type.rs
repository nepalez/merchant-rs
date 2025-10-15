use std::fmt::Display;
use zeroize_derive::ZeroizeOnDrop;

/// Type of the bank account
///
/// # Data Protection
/// This is a commonly used classifier requiring no security protection.
///
/// Consequently, both `Debug` and `Display` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub enum AccountType {
    Checking,
    Savings,
}

impl Display for AccountType {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
