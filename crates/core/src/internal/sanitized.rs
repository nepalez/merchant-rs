/// Sealed trait for types that require input sanitization before validation.
pub(crate) trait Sanitized {
    /// Trim leading and trailing whitespace.
    const TRIM: bool = false;

    /// Characters to remove from input.
    /// When set to `Some(_)`, ALL whitespace characters are removed automatically,
    /// plus any additional characters specified in the string.
    const CHARS_TO_REMOVE: Option<&'static str> = None;

    #[inline]
    fn sanitize(input: String) -> String {
        let mut result = input;

        if Self::TRIM {
            result = Self::apply_trim(result);
        }

        if Self::CHARS_TO_REMOVE.is_some() {
            result = Self::apply_char_removal(result);
        }

        result
    }

    #[inline]
    fn apply_trim(input: String) -> String {
        let trimmed = input.trim();
        if trimmed.len() != input.len() {
            trimmed.to_string()
        } else {
            input
        }
    }

    #[inline]
    fn apply_char_removal(input: String) -> String {
        let chars_to_remove = Self::CHARS_TO_REMOVE.unwrap();
        input
            .chars()
            .filter(|c| !c.is_whitespace() && !chars_to_remove.contains(*c))
            .collect()
    }
}
