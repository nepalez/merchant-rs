/// Insecure representation of a subscription billing interval.
///
/// ```skip
/// let daily = SubscriptionInterval::Day(1);
/// let monthly = SubscriptionInterval::Month(1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionInterval {
    /// Duration-based: exactly 24 hours * count from start_date
    Day(u32),
    /// Calendar-based: calendar month * count
    Month(u32),
}
