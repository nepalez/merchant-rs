use strum_macros::{AsRefStr, Display};

/// Status of a recurring payment subscription
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Paused,
    Canceled,
    PastDue,
    Expired,
    Pending,
}
