use std::fmt;

/// The sealed trait is used to protect sensitive data (PII or SAD)
/// by masking them in debug output.
///
/// # Safety
///
/// This trait is unsafe to implement because it assumes that the implementer
/// will uphold the invariants required for safely handling sensitive data
/// and also takes into account that the underlying data can be invalid.
///
/// Implementors MUST ensure that both `first_chars` and `last_chars`:
/// (1) neither cause **out-of-bounds access** to data that can be INVALID
///     (so the masking debug can be used in error messages);
/// (2) nor violate the applicable standards (e.g., PCI DSS) or policies
///     by **leaking the essential part** of the sensitive VALID data.
#[deny(private_bounds)]
pub(crate) unsafe trait Masked: Sized {
    /// The name of the type to wrap the masked data
    const TYPE_WRAPPER: &'static str;
    /// Masking string used in the debug output
    /// (if the first and last chars are exposed, it is placed BETWEEN them).
    const MASKING_STR: &'static str = "***";

    /// Safely exposes the first chars of the stored value
    #[inline]
    fn first_chars(&self) -> String {
        String::new()
    }

    /// Safely exposes the last chars of the stored value
    #[inline]
    fn last_chars(&self) -> String {
        String::new()
    }

    /// Returns the masked value
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
