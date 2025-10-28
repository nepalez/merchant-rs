use codes_iso_3166::part_1;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// Country code in addresses (either a country- or a region-wide).
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
/// * converts dots and underscores to hyphens (`PT_11` -> `PT-11`, `pt.11` -> `pt-11`).
///
/// # Validation
/// * validates country against the [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) standard
///   via the crate [codes-iso-3166](https://crates.io/crates/codes-iso-3166)
/// * does not validate region codes (this should be done by the gateway adapter if necessary).
///
/// # Data Protection
/// Country codes are NOT considered PII in any reasonable context,
/// as they represent broad geographic areas that cannot identify individuals.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct CountryCode(String);

impl<'a> TryFrom<&'a str> for CountryCode {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl AsRef<str> for CountryCode {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for CountryCode {
    fn sanitize(input: &str) -> Self {
        let input = input.trim();
        let mut output = Self(String::with_capacity(input.len()));
        for c in input.chars() {
            if c.is_ascii_alphanumeric() {
                output.0.push(c.to_ascii_uppercase());
            } else if c == '.' || c == '_' {
                output.0.push('-');
            } else {
                output.0.push(c)
            }
        }
        output.0.shrink_to_fit();
        output
    }
}

impl Validated for CountryCode {
    // We don't care about zeroization of the temporary data, that is not PII.
    fn validate(self) -> Result<Self, Error> {
        let (country_part, _) = self.0.split_once('-').unwrap_or_default();
        part_1::CountryCode::from_str(country_part)
            .map_err(|_| Error::InvalidInput(format!("{self:?} is invalid")))
            .map(|_| self)
    }
}
