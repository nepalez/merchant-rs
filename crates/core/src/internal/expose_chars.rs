use crate::internal::SecretString;

/// # SAFETY
/// `n` must be <= string length. For SafeWrapper: guaranteed by FIRST_CHARS/LAST_CHARS <= MIN_LENGTH.
pub trait ExposeChars {
    /// # SAFETY
    /// Caller must ensure `n <= self.len()`
    unsafe fn first_chars(&self, n: usize) -> &str;

    /// # SAFETY
    /// Caller must ensure `n <= self.len()`
    unsafe fn last_chars(&self, n: usize) -> &str;
}

impl ExposeChars for String {
    #[inline]
    unsafe fn first_chars(&self, n: usize) -> &str {
        &self[..n]
    }

    #[inline]
    unsafe fn last_chars(&self, n: usize) -> &str {
        let start = self.len() - n;
        &self[start..]
    }
}
