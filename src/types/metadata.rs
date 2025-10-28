use std::collections::HashMap;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::inputs::Metadata as Input;
use crate::internal::Masked;

/// Secure container for additional adapter-specific parameters
///
/// The data is neither sanitized nor validated.
///
/// # Data Protection
/// As we don't know in advance what data will be passed to the gateway,
/// we should treat it as PII/SAD (Sensitive Account Data)
/// requiring the strictest level of protection.
///
/// As such, it is:
/// * fully mask values in logs (via `Debug` implementation) to prevent any leaks,
/// * do NOT provide any public getter methods,
/// * expose their values only as a part of a request or response
///   via **unsafe** method `with_exposed_secret`,
/// * zeroize values on a drop.
#[derive(Clone, Debug, Default)]
pub struct Metadata(HashMap<&'static str, MetadataValue>);

impl TryFrom<Input<'_>> for Metadata {
    type Error = Error;

    #[inline]
    fn try_from(input: Input<'_>) -> Result<Self, Self::Error> {
        let mut output = Self::default();
        for (key, value) in input.into_iter() {
            output.0.insert(key, value.try_into()?);
        }
        Ok(output)
    }
}

#[derive(Clone, ZeroizeOnDrop)]
pub struct MetadataValue(String);

impl<'a> TryFrom<&'a str> for MetadataValue {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(input.to_string()))
    }
}

impl fmt::Debug for MetadataValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// SAFETY: The trait is safely implemented as:
// 1. it exposes a reference to the internal String which will be zeroized on a drop;
//    No copies are created, neither new memory is allocated;
// 2. it masks a value in logs without exposing any part.
unsafe impl Masked for MetadataValue {
    const TYPE_WRAPPER: &'static str = "Value";

    #[inline]
    fn masked_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "***")
    }
}
