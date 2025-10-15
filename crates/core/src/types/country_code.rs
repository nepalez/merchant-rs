use codes_iso_3166::part_1;
use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{sanitized::*, validated::*};

/// Country code in addresses
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * validates against the [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) standard
///   via the crate [codes-iso-3166](https://crates.io/crates/codes-iso-3166)
///
/// # Data Protection
/// Country codes are NOT considered PII in any reasonable context,
/// as they represent broad geographic areas that cannot identify individuals.
///
/// Consequently, both `Debug` and `Display` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct CountryCode(String);

impl FromStr for CountryCode {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl fmt::Display for CountryCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl<'a> Sanitized<'a> for CountryCode {
    type Input = &'a str;

    #[inline]
    fn sanitize(input: Self::Input) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for CountryCode {
    // We don't care about zeroization of the temporary data, that is not PII.
    fn validate(&self) -> Result<(), String> {
        part_1::CountryCode::from_str(&self.0)
            .map_err(|_| "is not valid".to_string())
            .map(|_| ())
    }
}
