use codes_iso_3166::part_1;
use std::convert::{AsRef, TryFrom};
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
        let (country_part, _) = self.0.split_once('-').unwrap_or((self.0.as_ref(), ""));
        part_1::CountryCode::from_str(country_part)
            .map_err(|_| Error::InvalidInput(format!("{self:?} is invalid")))
            .map(|_| self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_COUNTRY: &str = "US";
    const VALID_REGION: &str = "US-CA";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_region_codes() {
            [VALID_COUNTRY, VALID_REGION].iter().for_each(|&input| {
                let result = CountryCode::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            });
        }

        #[test]
        fn constructed_from_dots_and_underscores_to_hyphens() {
            let input = "us.ca";
            let code = CountryCode::try_from(input).unwrap();
            let result = code.as_ref();
            assert_eq!(result, VALID_REGION);

            let input = "us_ca";
            let code = CountryCode::try_from(input).unwrap();
            let result = code.as_ref();
            assert_eq!(result, VALID_REGION);
        }

        #[test]
        fn rejects_invalid_country_code() {
            let input = "ZZ-00";
            let result = CountryCode::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_invalid_format() {
            let input = "USA-11";
            let result = CountryCode::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn exposes_debug() {
            let code = CountryCode::try_from(VALID_REGION).unwrap();
            let debug_output = format!("{:?}", code);
            // CountryCode is public data, so it's not masked
            assert!(debug_output.contains(VALID_REGION));
        }

        #[test]
        fn as_ref_is_safe() {
            let input = " us.ca \n\t";
            let code = CountryCode::try_from(input).unwrap();
            let exposed = code.as_ref();
            assert_eq!(exposed, VALID_REGION);
        }
    }
}
