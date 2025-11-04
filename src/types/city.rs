use std::convert::{AsRef, TryFrom};
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// City name in addresses
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-100 characters
///
/// # Data Protection
/// City names are NOT considered PII in any reasonable context,
/// as they represent broad geographic areas that cannot identify individuals.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct City(String);

impl<'a> TryFrom<&'a str> for City {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl AsRef<str> for City {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for City {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for City {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 100)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CITY: &str = "New York";
    const VALID_CITY_LONG: &str = "Llanfairpwllgwyngyllgogerychwyrndrobwllllantysiliogogogoch";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_cities() {
            for input in [VALID_CITY, "London", VALID_CITY_LONG, "São Paulo", "Москва"] {
                let result = City::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " New York \n\t\r ";
            let city = City::try_from(input).unwrap();
            let result = city.as_ref();
            assert_eq!(result, VALID_CITY);
        }

        #[test]
        fn rejects_empty_city() {
            let input = "";
            let result = City::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_city() {
            let input = "a".repeat(101);
            let result = City::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn exposes_debug() {
            let city = City::try_from(VALID_CITY).unwrap();
            let debug_output = format!("{:?}", city);
            // City is public data, so it's not masked
            assert!(debug_output.contains(VALID_CITY));
        }

        #[test]
        fn as_ref_is_safe() {
            let input = " New York \n\t";
            let city = City::try_from(input).unwrap();
            let exposed = city.as_ref();
            assert_eq!(exposed, VALID_CITY);
        }
    }
}
