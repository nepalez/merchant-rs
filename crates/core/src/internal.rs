//! The module keeps sealed internal traits and types used exclusively
//! by types in this crate to DRY the code and prevent errors.

mod highly_secret;
mod masked;
mod personal_data;
pub(crate) mod sanitized;
pub(crate) mod validated;

// Internal traits
pub(crate) use highly_secret::HighlySecret;
pub(crate) use masked::Masked;
pub(crate) use personal_data::PersonalData;
pub(crate) use sanitized::Sanitized;
pub(crate) use validated::Validated;
