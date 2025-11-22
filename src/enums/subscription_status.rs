use strum_macros::{AsRefStr, Display};

/// Status of a recurring payment subscription
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum SubscriptionStatus {
    /// Subscription is currently active and processing payments
    Active,
    /// Subscription temporarily suspended but can be resumed
    Paused,
    /// Subscription permanently terminated by user or merchant
    Canceled,
    /// Payment failed, subscription at risk of cancellation
    PastDue,
    /// Subscription reached its end date or maximum cycle count
    Expired,
    /// Subscription created but first payment not yet processed
    Pending,
}
