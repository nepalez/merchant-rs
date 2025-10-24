use std::cmp::Ordering;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, validated::*};

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

impl CardExpiry {
    /// Start the builder chain to safely create the CardExpiry structure.
    #[inline]
    pub fn builder() -> Builder {
        Builder::default()
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
        const FIRST_CREDIT_CARD_YEAR: u16 = 1950;
        const MAX_SUPPORTED_YEAR: u16 = 2050;

        validate_year(&self.year, FIRST_CREDIT_CARD_YEAR, MAX_SUPPORTED_YEAR)?;
        validate_month(&self.month)
    }
}

unsafe impl Exposed for CardExpiry {
    type Output<'a> = ExposedCardExpiry<'a>;
    const TYPE_WRAPPER: &'static str = "CardExpiry";

    fn expose(&self) -> Self::Output<'_> {
        Self::Output {
            month: &self.month,
            year: &self.year,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ExposedCardExpiry<'a> {
    pub month: &'a u8,
    pub year: &'a u16,
}

impl Ord for ExposedCardExpiry<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month).cmp(&(other.year, other.month))
    }
}

impl PartialOrd for ExposedCardExpiry<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// --- Additional types ---

#[derive(Default, ZeroizeOnDrop)]
pub struct Builder {
    month: Option<u8>,
    year: Option<u16>,
}

impl Builder {
    #[inline]
    pub fn month(mut self, input: u8) -> Result<Self, Error> {
        self.month = Some(input);
        Ok(self)
    }

    #[inline]
    pub fn year(mut self, input: u16) -> Result<Self, Error> {
        self.year = Some(input);
        Ok(self)
    }

    #[inline]
    pub fn build(self) -> Result<CardExpiry, Error> {
        let Some(month) = self.month else {
            Err(Error::validation_failed("month is missed".to_string()))?
        };
        let Some(year) = self.year else {
            Err(Error::validation_failed("year is missed".to_string()))?
        };
        CardExpiry { month, year }.validated()
    }
}
