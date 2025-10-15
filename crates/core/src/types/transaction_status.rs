use strum_macros::{AsRefStr, Display};

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
