use strum_macros::{AsRefStr, Display};

/// Status of a payment transaction
///
/// # Data Protection
/// This is a commonly used classifier requiring no security protection.
///
/// Consequently, both `Debug` and `Display` are implemented without masking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
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
