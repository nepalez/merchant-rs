use std::cmp::Ordering;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, validated::*};

/// Birthdate of a payer
///
/// Use a builder pattern to safely create the BirthDate structure.
/// Notice that all fields take references to values to prevent
/// unsafe
///
/// ```skip
/// let birth_date = BirthDate::builder()
///     .year(1980)?
///     .month(1)?
///     .day(13)?
///     .build()?;
/// ```
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
/// As such, they are:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * exposed via the **unsafe** `as_str` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, Eq, PartialEq, ZeroizeOnDrop)]
pub struct BirthDate {
    day: u8,
    month: u8,
    year: u16,
}

impl BirthDate {
    #[inline]
    pub fn builder() -> Builder {
        Builder::default()
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
    fn validate(&self) -> Result<(), String> {
        const OLDEST_HUMAN_BIRTHDAY: u16 = 1909;
        const MAX_SUPPORTED_YEAR: u16 = 2050;

        validate_year(&self.year, OLDEST_HUMAN_BIRTHDAY, MAX_SUPPORTED_YEAR)?;
        validate_day(&self.day, &self.month, &self.year)
    }
}

// SAFETY: The trait is safely implemented because it totally masks sensitive data.
unsafe impl Exposed for BirthDate {
    type Output<'a> = ExposedBirthDate<'a>;

    const TYPE_WRAPPER: &'static str = "BirthDate";
    const MASKING_STR: &'static str = "**/**/****";

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        Self::Output {
            day: &self.day,
            month: &self.month,
            year: &self.year,
        }
    }
}

// --- Additional types ---

#[derive(Clone, Eq, PartialEq)]
pub struct ExposedBirthDate<'a> {
    pub day: &'a u8,
    pub month: &'a u8,
    pub year: &'a u16,
}

impl Ord for ExposedBirthDate<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, self.day))
    }
}

impl PartialOrd for ExposedBirthDate<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default, ZeroizeOnDrop)]
pub struct Builder {
    day: Option<u8>,
    month: Option<u8>,
    year: Option<u16>,
}

impl Builder {
    #[inline]
    pub fn day(mut self, input: u8) -> Result<Self, Error> {
        self.day = Some(input);
        Ok(self)
    }

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

    pub fn build(self) -> Result<BirthDate, Error> {
        let Some(day) = self.day else {
            Err(Error::validation_failed("day is missed".to_string()))?
        };
        let Some(month) = self.month else {
            Err(Error::validation_failed("month is missed".to_string()))?
        };
        let Some(year) = self.year else {
            Err(Error::validation_failed("year is missed".to_string()))?
        };
        BirthDate { day, month, year }.validated()
    }
}
