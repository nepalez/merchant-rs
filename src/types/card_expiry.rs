use std::cmp::Ordering;
use std::convert::TryFrom;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::inputs::CardExpiry as Input;
use crate::internal::Validated;

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
#[derive(Clone, Debug, Eq, PartialEq, ZeroizeOnDrop)]
pub struct CardExpiry {
    month: u8,
    year: u16,
}

impl CardExpiry {
    /// Exposes the reference to zeroized month
    ///
    /// # SAFETY
    /// This method is unsafe because it exposes sensitive PII data.
    ///
    /// Ensure that:
    /// (1) the data is not leaking into logs, error messages, etc.;
    /// (2) every clone or another object containing these data
    ///     is not leaked in logs and is zeroized upon a drop.
    pub unsafe fn month(&self) -> u8 {
        self.month
    }

    /// Exposes the reference to zeroized year
    ///
    /// # SAFETY
    /// This method is unsafe because it exposes sensitive PII data.
    ///
    /// Ensure that:
    /// (1) the data is not leaking into logs, error messages, etc.;
    /// (2) every clone or another object containing these data
    ///     is not leaked in logs and is zeroized upon a drop.
    pub unsafe fn year(&self) -> u16 {
        self.year
    }
}

impl TryFrom<Input> for CardExpiry {
    type Error = Error;

    fn try_from(input: Input) -> Result<Self, Self::Error> {
        Self {
            month: input.month,
            year: input.year,
        }
        .validate()
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

// --- Sealed traits (not parts of the public API) ---

impl Validated for CardExpiry {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        const FIRST_CREDIT_CARD_YEAR: u16 = 1950;
        const MAX_SUPPORTED_YEAR: u16 = 2050;

        self._validate_year(&self.year, FIRST_CREDIT_CARD_YEAR, MAX_SUPPORTED_YEAR)?;
        self._validate_month(&self.month)?;
        Ok(self)
    }
}
