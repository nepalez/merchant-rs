use crate::error::Result;
use crate::internal::{ExposeChars, ExposeSensitive, Sanitized, SecretString, Validated};
use std::fmt;

/// Sealed trait for newtype wrappers that can be constructed from validated strings.
///
/// # SAFETY
///
/// This trait is unsafe to implement because it assumes that the implementer
/// will uphold the invariants required for safely handling sensitive data.
///
/// Specifically, the implementer **MUST** ensure that:
/// 1. The `Inner` type is a **secure container** when this is required
///    by the applicable standards (e.g., PCI DSS) or policies
///    for the represented PII data.
/// 2. Exposition of the selected numbers of characters do not leak
///    the essential part of the sensitive data and do not violate
///    the applicable standards (e.g., PCI DSS) or policies.
/// 3. The validation rules guarantee that the inner data
///    has at least the number of characters that are exposed via
///    `EXPOSED_FIRST_CHARS` and `EXPOSED_LAST_CHARS`
///    so that no out-of-bounds access can occur.
///
/// Requires `Validated` to access `TYPE_NAME` and ensure `MIN_LENGTH` bounds.
pub(crate) unsafe trait SafeWrapper: Validated + Sized {
    type Inner;

    /// First chars exposed in debug. Must be <= MIN_LENGTH. PCI DSS: max 6 for PAN.
    const FIRST_CHARS: usize = 0;
    /// Last chars exposed in debug. Must be <= MIN_LENGTH. PCI DSS: max 4 for PAN.
    const LAST_CHARS: usize = 0;
    /// Masking string used in the debug output
    /// (if the first and last chars are exposed, it is placed BETWEEN them).
    const MASKING_STR: &'static str = "***";

    fn wrap(inner: Self::Inner) -> Self;

    /// # SAFETY
    ///
    /// Direct access to unmasked potentially sensitive data.
    /// This method is NOT supposed to be called outside from
    /// the `masked_debug` default implementation.
    unsafe fn inner(&self) -> &Self::Inner;

    // Default implementations (not to be overridden)

    #[inline]
    fn try_from_string(input: String) -> Result<Self>
    where
        Self: Sanitized,
        Self::Inner: From<String>,
    {
        let sanitized = Self::sanitize(input);
        Self::validate(&sanitized)?;
        Ok(Self::wrap(sanitized.into()))
    }

    #[inline]
    fn masked_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    where
        Self::Inner: ExposeChars,
    {
        let masked = if Self::FIRST_CHARS == 0 && Self::LAST_CHARS == 0 {
            Self::MASKING_STR.to_string()
        } else {
            // SAFETY:
            // The nested methods below is safe to call as they rely
            // on the safety checks that the implementer of the trait must provide.
            format!(
                "{}{}{}",
                unsafe { self.inner().first_chars(Self::FIRST_CHARS) },
                Self::MASKING_STR,
                unsafe { self.inner().last_chars(Self::LAST_CHARS) },
            )
        }
        .to_uppercase();

        f.debug_tuple(<Self as Validated>::TYPE_NAME)
            .field(&masked)
            .finish()
    }
}
