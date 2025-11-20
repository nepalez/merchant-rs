use std::cmp::Ordering;
use std::convert::TryFrom;

use crate::Error;
use crate::internal::Validated;

/// Billing interval for recurring subscriptions
///
/// Represents the frequency at which a customer is charged.
/// Supports both duration-based and calendar-based semantics.
///
/// # Duration-based vs Calendar-based
///
/// * `Day(n)`: Duration-based, counted from subscription start date
///   - `Day(7)` = exactly 7 * 24 hours from start (weekly)
///   - `Day(30)` = exactly 30 * 24 hours from start (not the same as monthly!)
///
/// * `Month(n)`: Calendar-based, anchored to calendar dates
///   - `Month(1)` = monthly billing
///   - `Month(3)` = quarterly billing
///   - `Month(12)` = yearly billing
///   - Starting Jan 31 → next billing Feb 28/29 (last day of shorter month)
///
/// # Ordering
///
/// `Day` and `Month` intervals cannot be compared as they represent
/// fundamentally different concepts (duration vs calendar).
///
/// # Examples
///
/// ```skip
/// use merchant_rs::types::SubscriptionInterval;
///
/// // Every day
/// let daily = SubscriptionInterval::Day(1);
///
/// // Weekly (every 7 days)
/// let weekly = SubscriptionInterval::Day(7);
///
/// // Every 30 days (different from monthly!)
/// let thirty_days = SubscriptionInterval::Day(30);
///
/// // Monthly (calendar-based)
/// let monthly = SubscriptionInterval::Month(1);
///
/// // Quarterly
/// let quarterly = SubscriptionInterval::Month(3);
///
/// // Yearly
/// let yearly = SubscriptionInterval::Month(12);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionInterval {
    /// Duration-based: exactly 24 hours * count from start_date
    Day(u32),

    /// Calendar-based: calendar month * count
    /// Anchored to specific day of month, handles variable month lengths (28-31 days)
    Month(u32),
}

impl PartialOrd for SubscriptionInterval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Day(a), Self::Day(b)) => Some(a.cmp(b)),
            (Self::Month(a), Self::Month(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

impl TryFrom<crate::SubscriptionInterval> for SubscriptionInterval {
    type Error = Error;

    fn try_from(input: crate::SubscriptionInterval) -> Result<Self, Self::Error> {
        match input {
            crate::SubscriptionInterval::Day(n) => Self::Day(n),
            crate::SubscriptionInterval::Month(n) => Self::Month(n),
        }
        .validate()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for SubscriptionInterval {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::Day(0) | Self::Month(0) => Err(Error::InvalidInput(
                "Subscription interval must be positive".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_days() {
        assert!(SubscriptionInterval::Day(1) < SubscriptionInterval::Day(2));
        assert!(SubscriptionInterval::Day(7) < SubscriptionInterval::Day(30));
    }

    #[test]
    fn compares_months() {
        assert!(SubscriptionInterval::Month(1) < SubscriptionInterval::Month(3));
        assert!(SubscriptionInterval::Month(3) < SubscriptionInterval::Month(12));
    }

    #[test]
    fn cannot_compare_days_and_months() {
        assert_eq!(
            SubscriptionInterval::Day(30).partial_cmp(&SubscriptionInterval::Month(1)),
            None
        );
        assert_eq!(
            SubscriptionInterval::Day(365).partial_cmp(&SubscriptionInterval::Month(12)),
            None
        );
    }

    #[test]
    fn equality() {
        assert_eq!(SubscriptionInterval::Day(7), SubscriptionInterval::Day(7));
        assert_eq!(
            SubscriptionInterval::Month(12),
            SubscriptionInterval::Month(12)
        );
    }

    #[test]
    fn validates_positive_day() {
        let result = SubscriptionInterval::Day(1).validate();
        assert!(result.is_ok());
    }

    #[test]
    fn validates_positive_month() {
        let result = SubscriptionInterval::Month(1).validate();
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_zero_day() {
        let result = SubscriptionInterval::Day(0).validate();
        assert!(matches!(result, Err(crate::Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_zero_month() {
        let result = SubscriptionInterval::Month(0).validate();
        assert!(matches!(result, Err(crate::Error::InvalidInput(_))));
    }
}
