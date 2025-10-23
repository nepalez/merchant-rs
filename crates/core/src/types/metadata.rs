use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::Exposed;

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
pub struct Metadata(HashMap<&'static str, Value>);

impl Metadata {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, key: &'static str, value: &str) -> Result<(), Error> {
        self.0.insert(key, Value::from_str(value)?);
        Ok(())
    }
}

// SAFETY: The trait is safely implemented as:
// 1. it exposes a hash with non-secret keys and of references to the internal
//    Strings which will be zeroized on a drop as its values;
//    No owned copies of those values are created;
// 2. its `Debug` implementation reuses the `masked_debug` implementation
//    for every value in the hash.
unsafe impl Exposed for Metadata {
    type Output<'a> = HashMap<&'static str, &'a str>;
    const TYPE_WRAPPER: &'static str = "Metadata";

    fn expose(&self) -> Self::Output<'_> {
        let mut output = Self::Output::with_capacity(self.0.len());
        for (key, value) in self.0.iter() {
            output.insert(key, value.expose());
        }
        output
    }
}

#[derive(Clone, ZeroizeOnDrop)]
pub struct Value(String);

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// SAFETY: The trait is safely implemented as:
// 1. it exposes a reference to the internal String which will be zeroized on a drop;
//    No copies are created, neither new memory is allocated;
// 2. it masks a value in logs without exposing any its part.
unsafe impl Exposed for Value {
    type Output<'a> = &'a str;
    const TYPE_WRAPPER: &'static str = "Value";

    fn expose(&self) -> Self::Output<'_> {
        self.0.as_str()
    }

    fn masked_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "***")
    }
}
