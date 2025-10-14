use crate::internal::{ExposeChars, SafeWrapper, SecretString};

/// Trait to provide access to sensitive data wrapped in a secure container,
/// while enforcing safety and awareness of its sensitivity.
pub trait ExposeSensitive {
    /// Exposes the first characters of the sensitive data
    /// as permitted by the type definition.
    fn first_chars(&self) -> &str;

    /// Exposes the last characters of the sensitive data
    /// as permitted by the type definition.
    fn last_chars(&self) -> &str;

    /// Exposes the underlying sensitive data as a string slice.
    ///
    /// This method is designed for sending data via Gateway adapter.
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
    unsafe fn with_exposed_secret<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&str) -> T;
}

impl<W> ExposeSensitive for W
where
    W: SafeWrapper<Inner = SecretString>,
{
    fn first_chars(&self) -> &str {
        // SAFETY:
        // Exposing only the allowed number of characters as per the type definition.
        // The safety contract is upheld by the SafeWrapper implementation.
        unsafe {
            <SecretString as ExposeChars>::first_chars(
                self.inner(),
                <Self as SafeWrapper>::FIRST_CHARS,
            )
        }
    }

    fn last_chars(&self) -> &str {
        // SAFETY:
        // Exposing only the allowed number of characters as per the type definition.
        // The safety contract is upheld by the SafeWrapper implementation.
        unsafe {
            <SecretString as ExposeChars>::last_chars(
                self.inner(),
                <Self as SafeWrapper>::LAST_CHARS,
            )
        }
    }

    #[inline]
    unsafe fn with_exposed_secret<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&str) -> T,
    {
        // SAFETY: the safety contract is passed to the caller.
        unsafe { self.inner().with_exposed_secret(f) }
    }
}
