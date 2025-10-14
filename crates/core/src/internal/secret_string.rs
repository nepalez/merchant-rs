use secrecy::{ExposeSecret, SecretBox};
use std::convert::From;

use crate::internal::{ExposeChars, SafeWrapper};

/// The Zero-cost container to control access to a PII-sensitive data.
///
/// # SAFETY
///
/// * Uses secure storage (`SecretBox`) to ensure memory zeroization after use.
/// * The container can be constructed via `From<String>` only.
///   Notice, however, that it is the responsibility of the caller to ensure
///   that the input string were sourced securely and had neither been cloned
///   nor logged before.
/// * `Clone` is implemented for request resilience, but the cloned value
///   is immediately re-wrapped in a new `SecretBox`.
/// * Neither `Debug` nor `Display` are implemented to prevent accidental leakage.
/// * Full access is only possible via the **unsafe** `with_exposed_secret` method,
///   which forces developers to acknowledge the handling of sensitive data.
/// * Partial access to the first six and last four digits
///   is provided via **unsafe** methods as permitted by PCI DSS for some types.
///   NEVERTHELESS, consider with care whether implement such methods for wrappers
///   to prevent leaking the length or the essential part of the data.
pub(crate) struct SecretString(SecretBox<String>);

impl SecretString {
    #[inline]
    pub unsafe fn with_exposed_secret<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&str) -> T,
    {
        f(self.0.expose_secret())
    }
}

impl From<String> for SecretString {
    fn from(input: String) -> Self {
        SecretString(SecretBox::new(Box::new(input)))
    }
}

impl Clone for SecretString {
    fn clone(&self) -> Self {
        let cloned_string = self.0.expose_secret().clone();
        SecretString(SecretBox::new(Box::new(cloned_string)))
    }
}

impl ExposeChars for SecretString {
    #[inline]
    unsafe fn first_chars(&self, number: usize) -> &str {
        let full_token_slice = self.0.expose_secret();
        &full_token_slice[..number]
    }

    #[inline]
    unsafe fn last_chars(&self, number: usize) -> &str {
        let full_token_slice = self.0.expose_secret();
        &full_token_slice[full_token_slice.len() - number..]
    }
}
