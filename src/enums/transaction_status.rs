use strum_macros::{AsRefStr, Display};

/// Status of a payment transaction
///
/// # Data Protection
/// This is a commonly used classifier requiring no security protection.
///
/// Consequently, both `Debug` and `Display` are implemented without masking.
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum TransactionStatus {
    Authorized,
    Captured,
    Pending,
    Declined,
    Failed,
    Voided,
    Refunded,
    Processing,
}
