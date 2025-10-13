use std::cmp::Ordering;

use crate::error::{Error, ErrorCode};

/// Represents a validated card expiration date (Month and 4-digit Year).
/// This type guarantees that month and year values are structurally valid (within defined range).
#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub fn new(month: u8, year: u16) -> Result<Self, Error> {
        CardExpiry::validate(month, year)?;
        Ok(CardExpiry { month, year })
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
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month).cmp(&(other.year, other.month))
    }
}

impl PartialOrd for CardExpiry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Custom validation logic
impl CardExpiry {
    // Constants derived from historical and practical standards
    /// Year of first widespread modern payment cards
    const MIN_YEAR: u16 = 1970;
    /// Practical upper limit for card expiration year to avoid unrealistic dates
    const MAX_YEAR: u16 = 2050;

    #[inline]
    fn validate(month: u8, year: u16) -> Result<(), Error> {
        if month == 0 || month > 12 {
            return Err(Error::validation_failed(format!(
                "Expiration month must be between 1 and 12, received {}",
                month
            )));
        }

        if year < Self::MIN_YEAR {
            return Err(Error::validation_failed(format!(
                "Expiration year ({}) is below the minimum required year ({})",
                year,
                Self::MIN_YEAR
            )));
        }

        if year > Self::MAX_YEAR {
            return Err(Error::validation_failed(format!(
                "Expiration year ({}) exceeds the maximum allowed year ({})",
                year,
                Self::MAX_YEAR
            )));
        }

        Ok(())
    }
}
