use crate::error::Result;
use std::fmt;

/// Sealed trait for newtype wrappers that can be constructed from validated strings.
///
/// # SAFETY
///
/// This trait is unsafe to implement because it assumes that the implementer
/// will uphold the invariants required for safely handling sensitive data and also takes
/// into account that the underlying data can be invalid.
///
/// Specifically, the implementer **MUST** ensure that exposing first and last characters:
/// 1. Neither causes out-of-bounds access to potentially INVALID data,
///    so that debug output can be used in error messages,
/// 2. Nor leaks the essential part of the sensitive VALID data
///    and violate the applicable standards (e.g., PCI DSS) or policies.
pub unsafe trait Masked: Sized {
    /// The name of the type to wrap the masked data
    const TYPE_WRAPPER: &'static str;
    /// Masking string used in the debug output
    /// (if the first and last chars are exposed, it is placed BETWEEN them).
    const MASKING_STR: &'static str = "***";

    /// Safely exposes the first chars of the stored value
    fn first_chars(&self) -> String {
        String::new()
    }

    /// Safely exposes the last chars of the stored value
    fn last_chars(&self) -> String {
        String::new()
    }

    #[inline]
    fn masked_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked = format!(
            "{}{}{}",
            self.first_chars(),
            Self::MASKING_STR,
            self.last_chars(),
        );

        f.debug_tuple(Self::TYPE_WRAPPER).field(&masked).finish()
    }
}
