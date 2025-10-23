/// Type of the bank account
///
/// # Data Protection
/// This is a public value, neither secret nor even PII.
/// Bank account types are standardized classifiers requiring no security protection.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(Clone, Copy, Debug)]
pub enum AccountType {
    Checking,
    Savings,
}

impl AsRef<str> for AccountType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}
