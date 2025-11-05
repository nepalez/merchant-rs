use strum_macros::{AsRefStr, Display};

/// Status of a recurring payment subscription
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum SubscriptionStatus {
    Active,
    Paused,
    Canceled,
    PastDue,
    Expired,
    Pending,
}
