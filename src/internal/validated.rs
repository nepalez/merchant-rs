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
    fn validate(&self) -> Result<(), String>;

    fn validated(self) -> Result<Self, Error> {
        self.validate()
            .map_err(|msg| Error::InvalidInput(format!("{self:?} {msg}")))?;
        Ok(self)
    }
}

#[inline]
pub(crate) fn validate_length(input: &str, min: usize, max: usize) -> Result<(), String> {
    // validate chars, not bytes!
    let len = input.chars().count();
    if len < min || len > max {
        Err(format!("length is out of range ({}-{})", min, max))
    } else {
        Ok(())
    }
}

#[inline]
pub(crate) fn validate_alphanumeric(input: &str, extra: &str) -> Result<(), String> {
    for c in input.chars() {
        if !c.is_ascii_alphanumeric() && !extra.contains(c) {
            return Err(format!("contains invalid character `{c}`"));
        }
    }
    Ok(())
}

#[inline]
pub(crate) fn validate_alphabetic(input: &str, extra: &str) -> Result<(), String> {
    for c in input.chars() {
        if !c.is_ascii_alphabetic() && !extra.contains(c) {
            return Err(format!("contains invalid character `{c}`"));
        }
    }
    Ok(())
}

#[inline]
pub(crate) fn validate_digits(input: &str, extra: &str) -> Result<(), String> {
    for c in input.chars() {
        if !c.is_ascii_digit() && !extra.contains(c) {
            return Err(format!("contains invalid character `{c}`"));
        }
    }
    Ok(())
}

#[inline]
pub(crate) fn validate_month(input: &u8) -> Result<(), String> {
    if !(1..=12).contains(input) {
        Err("month is out of range (1-12)".to_string())
    } else {
        Ok(())
    }
}

#[inline]
pub(crate) fn validate_year(input: &u16, min: u16, max: u16) -> Result<(), String> {
    if *input < min || *input > max {
        Err(format!("year is out of range ({}-{})", min, max))
    } else {
        Ok(())
    }
}

#[inline]
pub(crate) fn validate_day(day: &u8, month: &u8, year: &u16) -> Result<(), String> {
    validate_month(month)?;

    if *day == 0 {
        Err("day cannot be zero".to_string())
    } else if *day > days_in_month(year, month) {
        Err("day is out of range for given month and year".to_string())
    } else {
        Ok(())
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
