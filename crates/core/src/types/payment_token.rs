use std::convert::TryFrom;
use std::fmt;

use crate::error::*;
use crate::internal::*;

/// A Tokenized Payment Credential (e.g., from a payment processor or vault).
/// Wraps secrecy::SecretBox<String> to ensure memory is zeroed on drop and value is masked in Debug/Display.
#[derive(Clone)]
pub struct PaymentToken(SecretString);

// SAFETY:
//
// The trait is safely implemented because:
// 1. The type is wrapped in SecretString, which ensures memory is zeroed on drop,
// 2. The Debug implementation masks all but the last four characters of the token,
//    which is explicitly allowed by PCI DSS for Primary Account Numbers (PANs),
//    and tokens has more symbols than PANs.
// 3. The validation ensures that the token has at least 16 characters,
//    so it is guaranteed to have at least 4 characters to show.
unsafe impl SafeWrapper for PaymentToken {
    type Inner = SecretString;

    const LAST_CHARS: usize = 4;

    #[inline]
    fn wrap(inner: Self::Inner) -> Self {
        Self(inner)
    }

    #[inline]
    unsafe fn inner(&self) -> &Self::Inner {
        &self.0
    }
}

// The token should never be modified. Any extra chars are treated as an error.
impl Sanitized for PaymentToken {}

impl Validated for PaymentToken {
    const TYPE_NAME: &'static str = "Payment token";
    const MIN_LENGTH: usize = 16;
    const MAX_LENGTH: usize = 4096;
    const EXTRA_CHARS: Option<&'static str> = None;

    #[inline]
    fn validate(input: &str) -> Result<()> {
        if input.trim() != input {
            return Err(Error::validation_failed(format!(
                "{} contains invalid leading or trailing whitespace",
                Self::TYPE_NAME
            )));
        }

        Self::validate_length(input)?;

        Ok(())
    }
}

impl TryFrom<String> for PaymentToken {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

impl fmt::Debug for PaymentToken {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    impl FromStr for PaymentToken {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            Self::try_from(s.to_owned())
        }
    }
}
