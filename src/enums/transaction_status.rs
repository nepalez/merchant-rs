use strum_macros::{AsRefStr, Display};

/// Status of a payment transaction
///
/// # Data Protection
/// This is a commonly used classifier requiring no security protection.
///
/// Consequently, both `Debug` and `Display` are implemented without masking.
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum TransactionStatus {
    /// Funds reserved but not yet captured
    Authorized,
    /// Funds captured and transferred to merchant
    Captured,
    /// Transaction awaiting processing or confirmation
    Pending,
    /// Transaction rejected by issuer or gateway
    Declined,
    /// Transaction failed due to technical error
    Failed,
    /// Previously authorized transaction canceled before capture
    Voided,
    /// Funds returned to customer after successful capture
    Refunded,
    /// Transaction currently being processed by gateway
    Processing,
}
