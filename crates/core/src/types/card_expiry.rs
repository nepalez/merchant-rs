use std::cmp::Ordering;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Masked, validated::*};

const FIRST_CREDIT_CARD_YEAR: u16 = 1950;
const MAX_SUPPORTED_YEAR: u16 = 2050;

/// Card expiration date (month and year)
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

impl CardExpiry {
    /// Constructs a new CardExpiry instance, performing strict domain validation.
    ///
    /// The input must provide the month (1-12) and the full 4-digit year.
    /// This method enforces structural validity (range checks) but does not check
    /// for expiration against the current time.
    #[inline]
    pub fn new(month: &u8, year: &u16) -> Result<Self, Error> {
        Self {
            month: *month,
            year: *year,
        }
        .validated()
    }

    /// Returns the expiration month (1-12).
    #[inline]
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Returns the expiration year (1970-2050).
    #[inline]
    pub fn year(&self) -> u16 {
        self.year
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
        validate_year(&self.year, FIRST_CREDIT_CARD_YEAR, MAX_SUPPORTED_YEAR)?;
        validate_month(&self.month)
    }
}
