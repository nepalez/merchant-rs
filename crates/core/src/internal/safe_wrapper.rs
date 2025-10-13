use crate::internal::{Sanitized, Validated};

/// Sealed trait for newtype wrappers that can be constructed from validated strings.
///
/// This trait combines `Sanitized` and `Validated` to provide a complete
/// construction pipeline for secure wrapper types. It handles the three-step
/// process: sanitize → validate → wrap.
pub(crate) trait SafeWrapper: Sanitized + Validated + Sized {
    /// The inner type that can be constructed from a string.
    type Inner: From<String>;

    /// Wraps the inner value (obtained from a sanitized and validated string) into `Self`.
    /// This is typically a trivial newtype constructor: `Self(inner)`.
    fn wrap(inner: Self::Inner) -> Self;

    /// Constructs an instance of Self from a raw input string.
    #[inline]
    fn try_from_string(input: String) -> crate::Result<Self> {
        let sanitized = Self::sanitize(input);
        Self::validate(&sanitized)?;
        Ok(Self::wrap(sanitized.into()))
    }
}
