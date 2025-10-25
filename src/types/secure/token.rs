use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, validated::*};
use crate::types::insecure;

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
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
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

// SAFETY: The trait is safely implemented as it does NOT expose any part of the token,
// fully protecting this sensitive authentication data from exposure in debug output.
unsafe impl Exposed for Token {
    type Output<'a> = insecure::Token<'a>;
    const TYPE_WRAPPER: &'static str = "Token";

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        self.0.as_str()
    }
}
