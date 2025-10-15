use crate::internal::Masked;
use std::ops::Deref;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Trait to provide access to sensitive data wrapped in a secure container,
/// while enforcing safety and awareness of its sensitivity.
///
/// Sensitive data MUST be masked with all safety guarantees provided.
pub trait HighlySecret<'a>
where
    Self: Masked + ZeroizeOnDrop,
    Self: 'a,
{
    type Exposed;

    /// Exposes the underlying sensitive data as a string slice.
    ///
    /// This method is designed for sending data via Gateway adapter ONLY.
    ///
    /// # SAFETY
    ///
    /// This method is marked `unsafe` because it exposes highly sensitive data to the closure.
    ///
    /// The caller **MUST** ensure:
    /// 1. The processing within the closure does not copy
    ///    or store the exposed data in unsecured memory.
    /// 2. The data is consumed immediately, and its exposure lifetime
    ///    is strictly minimal (e.g., for transmission).
    /// 3. **Any structure or variable containing the exposed `&str` reference
    ///    MUST NOT escape the closure, and any intermediate structure
    ///    containing a copy of the raw data (for example, the request)
    ///    MUST itself guarantee zeroization upon a drop.**
    unsafe fn with_exposed_secret<T, F>(&'a self, f: F) -> T
    where
        F: FnOnce(Self::Exposed) -> T;

    #[inline]
    fn first_chars(&self) -> String {
        <Self as Masked>::first_chars(self)
    }

    #[inline]
    fn last_chars(&self) -> String {
        <Self as Masked>::last_chars(self)
    }
}
