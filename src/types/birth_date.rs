use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
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
    /// Exposes the reference to the zeroized year of birth
    ///
    /// # SAFETY
    /// This method is unsafe because it exposes sensitive PII data.
    ///
    /// Ensure that:
    /// (1) the data is not leaking into logs, error messages, etc.;
    /// (2) every clone or another object containing these data
    ///     is not leaked in logs and is zeroized upon a drop.
    pub unsafe fn year(&self) -> &u16 {
        &self.year
    }

    /// Exposes the reference to the zeroized month of birth
    ///
    /// # SAFETY
    /// This method is unsafe because it exposes sensitive PII data.
    ///
    /// Ensure that:
    /// (1) the data is not leaking into logs, error messages, etc.;
    /// (2) every clone or another object containing these data
    ///     is not leaked in logs and is zeroized upon a drop.
    pub unsafe fn month(&self) -> &u8 {
        &self.month
    }

    /// Exposes the reference to the zeroized day of birth
    ///
    /// # SAFETY
    /// This method is unsafe because it exposes sensitive PII data.
    ///
    /// Ensure that:
    /// (1) the data is not leaking into logs, error messages, etc.;
    /// (2) every clone or another object containing these data
    ///     is not leaked in logs and is zeroized upon a drop.
    pub unsafe fn day(&self) -> &u8 {
        &self.day
    }
}

impl TryFrom<crate::BirthDate> for BirthDate {
    type Error = Error;

    fn try_from(input: crate::BirthDate) -> Result<Self, Self::Error> {
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
        (self.year, self.month, self.day).cmp(&(other.year, other.month, other.day))
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

#[cfg(test)]
mod tests {
    use super::*;

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_dates() {
            let inputs = [
                (1, 1, 1909),
                (31, 12, 1909),
                (29, 2, 2000), // Leap year
                (15, 8, 1990),
                (31, 12, 2050),
            ];

            for (day, month, year) in inputs {
                let input = crate::BirthDate { day, month, year };
                let result = BirthDate::try_from(input);
                assert!(result.is_ok(), "({day}, {month}, {year}) failed validation");
            }
        }

        #[test]
        fn rejects_year_too_old() {
            let input = crate::BirthDate {
                day: 15,
                month: 8,
                year: 1908,
            };
            let result = BirthDate::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_year_too_far() {
            let input = crate::BirthDate {
                day: 15,
                month: 8,
                year: 2051,
            };
            let result = BirthDate::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_invalid_day_zero() {
            let input = crate::BirthDate {
                day: 0,
                month: 8,
                year: 1990,
            };
            let result = BirthDate::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_invalid_day_for_month() {
            let input = crate::BirthDate {
                day: 31,
                month: 4, // April has 30 days
                year: 1990,
            };
            let result = BirthDate::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_invalid_leap_day() {
            let input = crate::BirthDate {
                day: 29,
                month: 2,
                year: 1999, // Not a leap year
            };
            let result = BirthDate::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let input = crate::BirthDate {
                day: 15,
                month: 8,
                year: 1990,
            };
            let birth_date = BirthDate::try_from(input).unwrap();
            let debug_output = format!("{:?}", birth_date);
            assert!(
                debug_output.contains(r#"BirthDate("**/**/****")"#),
                "Expected masked debug output, got: {debug_output}"
            );
            assert!(!debug_output.contains("15"));
            assert!(!debug_output.contains("8"));
            assert!(!debug_output.contains("1990"));
        }

        #[test]
        fn getters_are_unsafe() {
            let input = crate::BirthDate {
                day: 15,
                month: 8,
                year: 1990,
            };
            let birth_date = BirthDate::try_from(input).unwrap();

            unsafe {
                assert_eq!(*birth_date.day(), 15);
                assert_eq!(*birth_date.month(), 8);
                assert_eq!(*birth_date.year(), 1990);
            }
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let day_ptr: *const u8;
            let month_ptr: *const u8;
            let year_ptr: *const u16;

            {
                let birth_date = BirthDate::try_from(crate::BirthDate {
                    day: 15,
                    month: 8,
                    year: 1990,
                })
                .unwrap();
                day_ptr = &birth_date.day as *const u8;
                month_ptr = &birth_date.month as *const u8;
                year_ptr = &birth_date.year as *const u16;
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                assert_ne!(*day_ptr, 15, "Day should be zeroed after drop");
                assert_ne!(*month_ptr, 8, "Month should be zeroed after drop");
                assert_ne!(*year_ptr, 1990, "Year should be zeroed after drop");
            }
        }
    }

    mod comparison {
        use super::*;

        #[test]
        fn compares_by_year_month_day() {
            let earlier = BirthDate::try_from(crate::BirthDate {
                day: 15,
                month: 8,
                year: 1990,
            })
            .unwrap();
            let later = BirthDate::try_from(crate::BirthDate {
                day: 20,
                month: 8,
                year: 1990,
            })
            .unwrap();

            assert!(earlier < later);
        }

        #[test]
        fn equal_dates_are_equal() {
            let first = BirthDate::try_from(crate::BirthDate {
                day: 15,
                month: 8,
                year: 1990,
            })
            .unwrap();
            let second = BirthDate::try_from(crate::BirthDate {
                day: 15,
                month: 8,
                year: 1990,
            })
            .unwrap();

            assert_eq!(first, second);
        }
    }
}
