use std::cmp::Ordering;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, validated::*};
use crate::types::insecure;

/// Card expiration (month and year).
///
/// Use a builder pattern to safely create the CardExpiry structure.
/// ```skip
/// let card_expiry = CardExpiry::builder()
///     .month(10)?
///     .year(2028)?
///     .build()?;
/// ```
///
/// # Validation
/// * month: 1-12,
/// * year: 1950-2050
///
/// # Data Protection
/// PCI DSS does not consider card expiration dates sensitive data,
/// as they cannot be used alone to authorize transactions.
///
/// As such, they are:
/// * not masked in logs (via `Debug` implementation),
/// * exposed via safe public methods `month()` and `year()`.
#[derive(Clone, Eq, PartialEq, ZeroizeOnDrop)]
pub struct CardExpiry {
    month: u8,
    year: u16,
}

impl TryFrom<insecure::CardExpiry> for CardExpiry {
    type Error = Error;

    fn try_from(value: insecure::CardExpiry) -> Result<Self, Self::Error> {
        Self {
            month: value.month,
            year: value.year,
        }
        .validated()
    }
}

impl Ord for CardExpiry {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month).cmp(&(other.year, other.month))
    }
}

impl PartialOrd for CardExpiry {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for CardExpiry {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}/{:04}", self.month, self.year)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for CardExpiry {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        const FIRST_CREDIT_CARD_YEAR: u16 = 1950;
        const MAX_SUPPORTED_YEAR: u16 = 2050;

        validate_year(&self.year, FIRST_CREDIT_CARD_YEAR, MAX_SUPPORTED_YEAR)?;
        validate_month(&self.month)
    }
}

// SAFETY: The trait is safe to implement because:
// 1. Its output is a zeroized structure;
// 2. It does not expose any part of its data via `first_chars` or `last_chars`.
unsafe impl Exposed for CardExpiry {
    type Output<'a> = insecure::CardExpiry;
    const TYPE_WRAPPER: &'static str = "CardExpiry";

    fn expose(&self) -> Self::Output<'_> {
        Self::Output {
            month: self.month,
            year: self.year,
        }
    }
}
