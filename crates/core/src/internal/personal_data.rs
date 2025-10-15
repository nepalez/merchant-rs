use crate::internal::Masked;

/// A trait to mark types as personal data that aren't sensitive
/// but require additional attention to prevent leakage of PII in logs.
pub trait PersonalData: Masked {
    unsafe fn as_str(&self) -> &str;
}
