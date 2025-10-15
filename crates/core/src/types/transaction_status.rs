use strum_macros::{AsRefStr, Display};
use zeroize_derive::ZeroizeOnDrop;

/// Status of a payment transaction
///
/// # Data Protection
/// This is a commonly used classifier requiring no security protection.
///
/// Consequently, both `Debug` and `Display` are implemented without masking.
#[derive(Debug, Clone, PartialEq, Eq, Display, AsRefStr, ZeroizeOnDrop)]
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
