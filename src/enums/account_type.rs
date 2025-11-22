use strum_macros::{AsRefStr, Display};

/// Type of the bank account
///
/// # Data Protection
/// This is a public value, neither secret nor even PII.
/// Bank account types are standardized classifiers requiring no security protection.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum AccountType {
    /// Current account for everyday transactions and payments
    Checking,
    /// Deposit account for accumulating funds with interest
    Savings,
}
