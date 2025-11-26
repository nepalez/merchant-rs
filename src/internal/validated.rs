use crate::Error;
use std::fmt::Debug;

/// Sealed trait for types that require domain validation of their input.
///
/// Validation occurs on the *sanitized* input and typically includes:
/// - Length checks (min/max)
/// - Character set validation (alphanumeric and optional extra characters)
/// - Domain-specific rules (e.g., Luhn check for PAN)
///
/// # Default Implementation
///
/// The default `validate()` implementation covers most cases:
/// 1. Validates length against `MIN_LENGTH` and `MAX_LENGTH`
/// 2. If `EXTRA_CHARS` is `Some`, validates charset (alphanumeric and extras)
///
/// Override `validate()` for domain-specific rules.
pub(crate) trait Validated: Sized + Debug {
    fn validate(self) -> Result<Self, Error>;

    fn _validate_length(&self, input: &str, min: usize, max: usize) -> Result<(), Error> {
        // validate chars, not bytes!
        let len = input.chars().count();
        if len < min || len > max {
            Err(Error::InvalidInput(format!(
                "{self:?} length is out of range ({}-{})",
                min, max
            )))
        } else {
            Ok(())
        }
    }

    fn _validate_alphanumeric(&self, input: &str, extra: &str) -> Result<(), Error> {
        for c in input.chars() {
            if !c.is_ascii_alphanumeric() && !extra.contains(c) {
                return Err(Error::InvalidInput(format!(
                    "{self:?} contains invalid character `{c}`"
                )));
            }
        }
        Ok(())
    }

    fn _validate_alphabetic(&self, input: &str, extra: &str) -> Result<(), Error> {
        for c in input.chars() {
            if !c.is_ascii_alphabetic() && !extra.contains(c) {
                return Err(Error::InvalidInput(format!(
                    "{self:?} contains invalid character `{c}`"
                )));
            }
        }
        Ok(())
    }

    fn _validate_digits(&self, input: &str, extra: &str) -> Result<(), Error> {
        for c in input.chars() {
            if !c.is_ascii_digit() && !extra.contains(c) {
                return Err(Error::InvalidInput(format!(
                    "{self:?} contains invalid character `{c}`"
                )));
            }
        }
        Ok(())
    }

    fn _validate_month(&self, input: &u8) -> Result<(), Error> {
        if !(1..=12).contains(input) {
            Err(Error::InvalidInput(
                "{self:?} month is out of range (1-12)".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn _validate_year(&self, input: &u16, min: u16, max: u16) -> Result<(), Error> {
        if !(min..=max).contains(input) {
            Err(Error::InvalidInput(format!(
                "{self:?} year is out of range ({min}-{max})",
            )))
        } else {
            Ok(())
        }
    }

    fn _validate_day(&self, day: &u8, month: &u8, year: &u16) -> Result<(), Error> {
        self._validate_month(month)?;
        let max = days_in_month(year, month);

        if !(1..=max).contains(day) {
            Err(Error::InvalidInput(format!(
                "{self:?} is out of range (1-{max})"
            )))
        } else {
            Ok(())
        }
    }

    fn _validate_no_trailing_spaces(&self, input: &str) -> Result<(), Error> {
        if input.trim() == input {
            Ok(())
        } else {
            Err(Error::InvalidInput(format!(
                "{self:?} contains trailing whitespaces"
            )))
        }
    }
}

#[inline]
fn is_leap_year(year: &u16) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

#[inline]
fn days_in_month(year: &u16, month: &u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}
