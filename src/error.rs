//! Defines the **canonical error type** (`Error`) and **result alias** (`Result`)
//! for the entire `merchant-rs` ecosystem.
//!
//! This module acts as the **opaque boundary** of the core, ensuring that all
//! internal, gateway-specific errors are converted into a stable, unified type.
//! The canonical error allows client code (business logic) to handle faults
//! consistently, regardless of which payment gateway caused the failure.
//!
//! It includes metadata like a canonical error code, the gateway's original
//! error code, and a flag indicating if the operation is safely retriable.

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// General data validation error (invalid CVV, expired card, missing field).
    #[error("Validation failed: {0}")]
    InvalidInput(String),
}
