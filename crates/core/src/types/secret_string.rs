use secrecy::{ExposeSecret, SecretBox};
use std::convert::From;

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

// Methods to access the PAN in a controlled manner.
impl SecretString {
    /// Exposes the first six digits of the stored String.
    ///
    /// # SAFETY
    ///
    /// This method is marked `unsafe` to signal that exposing
    /// even a partial part of the sensitive data should be done
    /// with care and full awareness of the implications.
    ///
    /// Every wrapper **MUST** ensure that the secret string stored in the container
    /// is long enough to avoid leaking the essential part of the data,
    /// and the exposition is compliant with the relevant regulations.
    pub unsafe fn first_chars(&self, number: usize) -> &str {
        let full_token_slice = self.0.expose_secret();
        &full_token_slice[..number]
    }

    /// Exposes the last four digits of the PAN as a String.
    ///
    /// # SAFETY
    ///
    /// This method is marked `unsafe` to signal that exposing
    /// even a partial part of the sensitive data should be done
    /// with care and full awareness of the implications.
    ///
    /// The caller **MUST** ensure that the secret string stored in the container
    /// is long enough (> 13 characters) to avoid leaking the essential part of the data.
    /// The limit of 13 characters is chosen as it is the minimum length of a valid PAN,
    /// for which the PCI DSS explicitly allows exposing the first six digits.
    pub unsafe fn last_chars(&self, number: usize) -> &str {
        let full_token_slice = self.0.expose_secret();
        &full_token_slice[full_token_slice.len() - number..]
    }

    /// Exposes the underlying Primary Account Number (PAN) as a string slice.
    ///
    /// This method is designed for use by external payment adapter crates ONLY.
    ///
    /// # SAFETY
    ///
    /// This method is marked `unsafe` because it exposes highly sensitive data to the closure.
    ///
    /// The caller **MUST** ensure:
    /// 1. The processing within the closure does not copy
    ///    or store the exposed data in unsecured memory.
    /// 2. The data is consumed immediately and its exposure lifetime
    ///    is strictly minimal (e.g., for transmission).
    /// 3. **Any structure or variable containing the exposed `&str` reference
    ///    MUST NOT escape the closure, and any intermediate structure
    ///    containing a copy of the raw data (for example, the request)
    ///    MUST itself guarantee zeroization upon drop.**
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
