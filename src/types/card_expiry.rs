use std::cmp::Ordering;
use std::convert::TryFrom;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
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

impl TryFrom<crate::CardExpiry> for CardExpiry {
    type Error = Error;

    fn try_from(input: crate::CardExpiry) -> Result<Self, Self::Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_dates() {
            let inputs = [(1, 1950), (12, 1950), (6, 2025), (12, 2050)];

            for (month, year) in inputs {
                let input = crate::CardExpiry { month, year };
                let result = CardExpiry::try_from(input);
                assert!(result.is_ok(), "({month}, {year}) failed validation");
            }
        }

        #[test]
        fn rejects_invalid_month_zero() {
            let input = crate::CardExpiry {
                month: 0,
                year: 2025,
            };
            let result = CardExpiry::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_invalid_month_thirteen() {
            let input = crate::CardExpiry {
                month: 13,
                year: 2025,
            };
            let result = CardExpiry::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_year_too_old() {
            let input = crate::CardExpiry {
                month: 6,
                year: 1949,
            };
            let result = CardExpiry::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_year_too_far() {
            let input = crate::CardExpiry {
                month: 6,
                year: 2051,
            };
            let result = CardExpiry::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn exposes_debug() {
            let input = crate::CardExpiry {
                month: 12,
                year: 2030,
            };
            let expiry = CardExpiry::try_from(input).unwrap();
            let debug_output = format!("{:?}", expiry);
            assert!(debug_output.contains("CardExpiry"));
            assert!(debug_output.contains("month: 12"));
            assert!(debug_output.contains("year: 2030"));
        }

        #[test]
        fn getters_are_unsafe() {
            let input = crate::CardExpiry {
                month: 12,
                year: 2030,
            };
            let expiry = CardExpiry::try_from(input).unwrap();

            unsafe {
                assert_eq!(expiry.month(), 12);
                assert_eq!(expiry.year(), 2030);
            }
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let month_ptr: *const u8;
            let year_ptr: *const u16;

            {
                let expiry = CardExpiry::try_from(crate::CardExpiry {
                    month: 12,
                    year: 2030,
                })
                .unwrap();
                month_ptr = &expiry.month as *const u8;
                year_ptr = &expiry.year as *const u16;
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                assert_ne!(*month_ptr, 12, "Month should be zeroed after drop");
                assert_ne!(*year_ptr, 2030, "Year should be zeroed after drop");
            }
        }
    }

    mod comparison {
        use super::*;

        #[test]
        fn compares_by_year_then_month() {
            let earlier = CardExpiry::try_from(crate::CardExpiry {
                month: 6,
                year: 2025,
            })
            .unwrap();
            let later = CardExpiry::try_from(crate::CardExpiry {
                month: 12,
                year: 2025,
            })
            .unwrap();

            assert!(earlier < later);
        }

        #[test]
        fn equal_dates_are_equal() {
            let first = CardExpiry::try_from(crate::CardExpiry {
                month: 6,
                year: 2025,
            })
            .unwrap();
            let second = CardExpiry::try_from(crate::CardExpiry {
                month: 6,
                year: 2025,
            })
            .unwrap();

            assert_eq!(first, second);
        }
    }
}
