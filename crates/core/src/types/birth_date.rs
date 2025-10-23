use std::cmp::Ordering;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, validated::*};

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
    pub fn new(year: &u16, month: &u8, day: &u8) -> Result<Self, Error> {
        Self {
            year: *year,
            month: *month,
            day: *day,
        }
        .validated()
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

// SAFETY: The trait is safely implemented as it does NOT expose any part of the birthdate,
// fully protecting this sensitive PII in all contexts.
unsafe impl Exposed for BirthDate {
    type Output<'a> = ExposedBirthDate<'a>;

    const TYPE_WRAPPER: &'static str = "BirthDate";
    const MASKING_STR: &'static str = "**/**/****";

    fn expose(&self) -> Self::Output<'_> {
        Self::Output {
            day: &self.day,
            month: &self.month,
            year: &self.year,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ExposedBirthDate<'a> {
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
