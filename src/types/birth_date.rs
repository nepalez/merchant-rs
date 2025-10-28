use std::cmp::Ordering;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::inputs::BirthDate as Input;
use crate::internal::{Masked, Validated};

/// Birthdate of a payer
///
/// # Validation
/// * year: 1909-2050,
/// * month: valid 1-12,
/// * day: valid for the given month and year
///
/// # Data Protection
/// Birth dates can be used to identify individuals
/// and enable identity theft, making them sensitive PII
/// (Personal Identifiable Information).
///
/// As such, they are fully masked via `Debug` implementation
/// to prevent data leaking in logs.
#[derive(Clone, Eq, PartialEq, ZeroizeOnDrop)]
pub struct BirthDate {
    day: u8,
    month: u8,
    year: u16,
}

impl BirthDate {
    /// Safely exposes the reference to the zeroized year of birth
    pub fn year(&self) -> &u16 {
        &self.year
    }

    /// Safely exposes the reference to the zeroized month of birth
    pub fn month(&self) -> &u8 {
        &self.month
    }

    /// Safely exposes the reference to the zeroized day of birth
    pub fn day(&self) -> &u8 {
        &self.day
    }
}

impl TryFrom<Input> for BirthDate {
    type Error = Error;

    fn try_from(input: Input) -> Result<Self, Self::Error> {
        Self {
            day: input.day,
            month: input.month,
            year: input.year,
        }
        .validate()
    }
}

impl fmt::Debug for BirthDate {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl Ord for BirthDate {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, self.day))
    }
}

impl PartialOrd for BirthDate {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for BirthDate {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        const OLDEST_HUMAN_BIRTHDAY: u16 = 1909;
        const MAX_SUPPORTED_YEAR: u16 = 2050;

        self._validate_year(&self.year, OLDEST_HUMAN_BIRTHDAY, MAX_SUPPORTED_YEAR)?;
        self._validate_day(&self.day, &self.month, &self.year)?;
        Ok(self)
    }
}

// SAFETY: The trait is safe to implement because
// it does not expose any part of its data via `first_chars` or `last_chars`.
unsafe impl Masked for BirthDate {
    const TYPE_WRAPPER: &'static str = "BirthDate";
    const MASKING_STR: &'static str = "**/**/****";
}
