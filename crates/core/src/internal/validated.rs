use crate::Error;

/// Sealed trait for types that require domain validation of their input.
///
/// Validation occurs on the *sanitized* input and typically includes:
/// - Length checks (min/max)
/// - Character set validation (alphanumeric + optional extra characters)
/// - Domain-specific rules (e.g., Luhn check for PAN)
///
/// # Default Implementation
///
/// The default `validate()` implementation covers most cases:
/// 1. Validates length against `MIN_LENGTH` and `MAX_LENGTH`
/// 2. If `EXTRA_CHARS` is `Some`, validates charset (alphanumeric + extras)
///
/// Override `validate()` for domain-specific rules.
pub(crate) trait Validated {
    const TYPE_NAME: &'static str;
    const MIN_LENGTH: usize = 1;
    const MAX_LENGTH: usize;

    /// Additional allowed characters beyond alphanumeric.
    /// - `None`: No charset validation
    /// - `Some("")`: Strict alphanumeric only
    /// - `Some("-_")`: Alphanumeric + specified characters
    const EXTRA_CHARS: Option<&'static str> = None;

    #[inline]
    fn validate(input: &str) -> crate::Result<()> {
        Self::validate_length(input)?;

        if Self::EXTRA_CHARS.is_some() {
            Self::validate_charset(input)?;
        }

        Ok(())
    }

    #[inline]
    fn validate_length(input: &str) -> crate::Result<()> {
        let len = input.len();

        if len < Self::MIN_LENGTH {
            let message = if Self::MIN_LENGTH == 1 {
                format!("{} cannot be empty", Self::TYPE_NAME)
            } else {
                format!(
                    "{} length ({}) is below minimum ({})",
                    Self::TYPE_NAME,
                    len,
                    Self::MIN_LENGTH
                )
            };
            return Err(Error::validation_failed(message));
        }

        if len > Self::MAX_LENGTH {
            return Err(Error::validation_failed(format!(
                "{} length ({}) exceeds maximum ({})",
                Self::TYPE_NAME,
                len,
                Self::MAX_LENGTH
            )));
        }

        Ok(())
    }

    #[inline]
    fn validate_charset(input: &str) -> crate::Result<()> {
        let extra = Self::EXTRA_CHARS.unwrap(); // Safe: checked in validate()

        for c in input.chars() {
            if !c.is_ascii_alphanumeric() && !extra.contains(c) {
                let description = if extra.is_empty() {
                    "alphanumeric characters".to_string()
                } else {
                    format!("alphanumeric characters and '{}'", extra)
                };

                return Err(Error::validation_failed(format!(
                    "{} must contain only {}",
                    Self::TYPE_NAME,
                    description
                )));
            }
        }

        Ok(())
    }
}
