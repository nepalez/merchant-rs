use codes_iso_639::part_1::LanguageCode;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// Language tag in IETF BCP 47 format (e.g., "en-US", "pt-BR").
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
/// * converts underscores to hyphens (`en_US` -> `en-US`).
///
/// # Validation
/// * validates the language part against the ISO 639-1 standard
///   via the crate [codes-iso-639](https://crates.io/crates/codes-iso-639)
/// * does not validate region codes (this should be done by the gateway adapter if necessary).
///
/// # Data Protection
/// Language tags are NOT considered PII in any reasonable context,
/// as they represent broad linguistic preferences that cannot identify individuals.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct Language(String);

impl TryFrom<&str> for Language {
    type Error = Error;

    #[inline]
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl AsRef<str> for Language {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for Language {
    fn sanitize(input: &str) -> Self {
        let input = input.trim();
        let mut output = Self(String::with_capacity(input.len()));
        for c in input.chars() {
            if c == '_' {
                output.0.push('-');
            } else if c.is_ascii_control() {
                continue;
            } else {
                output.0.push(c)
            }
        }
        output.0.shrink_to_fit();
        output
    }
}

impl Validated for Language {
    fn validate(self) -> Result<Self, Error> {
        let (language_part, _) = self.0.split_once('-').unwrap_or((self.0.as_ref(), ""));
        language_part
            .to_ascii_lowercase()
            .parse::<LanguageCode>()
            .map_err(|_| Error::InvalidInput(format!("{self:?} is invalid")))
            .map(|_| self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_LANGUAGE: &str = "en";
    const VALID_TAG: &str = "en-US";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_tags() {
            [VALID_LANGUAGE, VALID_TAG, "pt-BR", "zh-CN"]
                .iter()
                .for_each(|&input| {
                    let result = Language::try_from(input);
                    assert!(result.is_ok(), "{input:?} failed validation");
                });
        }

        #[test]
        fn constructed_from_underscores_to_hyphens() {
            let input = "en_US";
            let tag = Language::try_from(input).unwrap();
            let result = tag.as_ref();
            assert_eq!(result, VALID_TAG);
        }

        #[test]
        fn rejects_invalid_language_code() {
            let input = "zz-ZZ";
            let result = Language::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_invalid_format() {
            let input = "english";
            let result = Language::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn exposes_debug() {
            let tag = Language::try_from(VALID_TAG).unwrap();
            let debug_output = format!("{:?}", tag);
            assert!(debug_output.contains(VALID_TAG));
        }

        #[test]
        fn as_ref_is_safe() {
            let input = " en_US \n\t";
            let tag = Language::try_from(input).unwrap();
            let exposed = tag.as_ref();
            assert_eq!(exposed, VALID_TAG);
        }
    }
}
