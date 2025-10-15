use std::fmt;
use std::str::FromStr;
use zeroize_derive::{Zeroize, ZeroizeOnDrop};

use crate::error::Error;
use crate::internal::{HighlySecret, Masked, validated::*};

/// Tokenized credential from a payment processor or vault
///
/// # Validation
/// * length: 16-4096 characters,
/// * no leading or trailing spaces are allowed
///
/// # Data Protection
/// Tokens provide full access to payment credentials and enable unauthorized transactions,
/// making them sensitive authentication data (SAD).
///
/// As such, they are:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * exposed via the **unsafe** `with_exposed_secret` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, ZeroizeOnDrop)]
pub struct Token(String);

impl FromStr for Token {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self(input.to_string()).validated()
    }
}

impl fmt::Debug for Token {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl<'a> HighlySecret<'a> for Token {
    type Exposed = &'a str;

    unsafe fn with_exposed_secret<T, F>(&'a self, f: F) -> T
    where
        F: FnOnce(Self::Exposed) -> T,
    {
        f(self.0.as_str())
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for Token {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 16, 4096)?;

        if self.0.trim() == self.0 {
            Ok(())
        } else {
            Err("contains trailing whitespaces".to_string())
        }
    }
}

// SAFETY: The trait is safely implemented as it does NOT expose any part of the internal value.
unsafe impl Masked for Token {
    const TYPE_WRAPPER: &'static str = "Token";
}
