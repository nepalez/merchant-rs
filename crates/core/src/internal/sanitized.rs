/// Sealed trait for types that require input sanitization before validation.
///
/// Sanitization typically involves:
/// - Removing allowed separators (spaces, hyphens, underscores)
/// - Normalizing whitespace
/// - Filtering invalid characters
///
/// # Default Implementation
///
/// The default implementation performs no sanitization (identity function).
/// This is appropriate for types that accept input as-is.
pub(crate) trait Sanitized {
    /// Sanitizes the input string, returning the cleaned version.
    #[inline]
    fn sanitize(input: String) -> crate::Result<String> {
        Ok(input)
    }
}
