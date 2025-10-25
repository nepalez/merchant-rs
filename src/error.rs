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

use std::{error::Error as StdError, fmt};
use strum_macros::{AsRefStr, Display};

/// Canonical result type used across the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Canonical error type for the core.
/// Used for both Err return values and as an optional field in Response structs.
#[derive(Debug, Clone)]
pub struct Error {
    /// The strictly typed canonical error code.
    pub code: ErrorCode,
    /// Human-readable message from the gateway/adapter.
    pub message: String,
    /// The original, specific error code returned by the gateway for debugging/logging.
    pub gateway_code: Option<String>,
    /// Flag indicating if the operation should be safely retried.
    pub is_retriable: bool,
    /// Additional context or details (e.g., JSON representation of a webhook error).
    pub detail: Option<String>,
}

/// Canonical error codes that can be unambiguously handled by the application.
/// Adapters must translate gateway-specific errors into this set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// General data validation error (invalid CVV, expired card, missing field).
    ValidationFailed,
    /// Transaction was declined by the card issuer or gateway for a soft reason (e.g., insufficient funds).
    CardDeclined,
    /// Communication failure with the gateway (timeout, 5xx server error).
    GatewayTimeout,
    /// Transaction blocked by the fraud detection system.
    FraudDetected,
    /// Problem with merchant account setup (invalid API key, missing configuration).
    Configuration,
    /// An error that does not fit into any other canonical category.
    Other,
}

impl Error {
    /// Creates an error with `ErrorCode::ValidationFailed` for internal domain validation issues.
    /// This is the preferred method for errors raised by types like PrimaryAccountNumber.
    pub fn validation_failed(message: String) -> Self {
        Error {
            code: ErrorCode::ValidationFailed,
            message,
            gateway_code: None,
            is_retriable: false,
            detail: None,
        }
    }

    /// Creates an error with `ErrorCode::Other` for general internal errors
    /// that do not have a specific canonical code.
    pub fn other(message: String) -> Self {
        Error {
            code: ErrorCode::Other,
            message,
            gateway_code: None,
            is_retriable: false,
            detail: None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Start with the canonical code and the human-readable message.
        write!(f, "{}: {}", self.code.as_ref(), self.message)?;

        // Append gateway-specific code if available.
        if let Some(ref gw_code) = self.gateway_code {
            write!(f, " (Gateway Code: {})", gw_code)?;
        }

        // Append additional details if available.
        if let Some(ref detail) = self.detail {
            write!(f, " - Details: {}", detail)?;
        }

        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        // The canonical error is an abstraction layer; it doesn't wrap a concrete
        // Rust error object, so the source is typically None.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_failed_constructor() {
        let error = Error::validation_failed("Invalid format".to_string());
        assert_eq!(error.code, ErrorCode::ValidationFailed);
        assert_eq!(error.message, "Invalid format");
        assert_eq!(error.gateway_code, None);
        assert_eq!(error.is_retriable, false);
    }

    #[test]
    fn test_other_constructor() {
        let error = Error::other("Internal logic failed".to_string());
        assert_eq!(error.code, ErrorCode::Other);
        assert_eq!(error.message, "Internal logic failed");
    }

    #[test]
    fn test_error_display_minimal() {
        let error = Error {
            code: ErrorCode::CardDeclined,
            message: "Insufficient funds".to_string(),
            gateway_code: None,
            is_retriable: false,
            detail: None,
        };

        let display = format!("{}", error);
        assert_eq!(display, "CARD_DECLINED: Insufficient funds");
    }

    #[test]
    fn test_error_display_full() {
        let error = Error {
            code: ErrorCode::Other,
            message: "Unknown error".to_string(),
            gateway_code: Some("ERR_999".to_string()),
            is_retriable: true,
            detail: Some("Contact support".to_string()),
        };

        let display = format!("{}", error);
        assert_eq!(
            display,
            "OTHER: Unknown error (Gateway Code: ERR_999) - Details: Contact support"
        );
    }

    #[test]
    fn test_error_code_as_ref() {
        assert_eq!(ErrorCode::ValidationFailed.as_ref(), "VALIDATION_FAILED");
        assert_eq!(ErrorCode::CardDeclined.as_ref(), "CARD_DECLINED");
        assert_eq!(ErrorCode::GatewayTimeout.as_ref(), "GATEWAY_TIMEOUT");
        assert_eq!(ErrorCode::FraudDetected.as_ref(), "FRAUD_DETECTED");
        assert_eq!(ErrorCode::Configuration.as_ref(), "CONFIGURATION");
        assert_eq!(ErrorCode::Other.as_ref(), "OTHER");
    }

    #[test]
    fn test_all_error_codes() {
        let codes = vec![
            ErrorCode::ValidationFailed,
            ErrorCode::CardDeclined,
            ErrorCode::GatewayTimeout,
            ErrorCode::FraudDetected,
            ErrorCode::Configuration,
            ErrorCode::Other,
        ];

        for code in codes {
            let error = Error {
                code,
                message: format!("Test for {:?}", code),
                gateway_code: None,
                is_retriable: false,
                detail: None,
            };

            // Verify that Display works for all error codes
            let display = format!("{}", error);
            assert!(display.contains(&format!("{}", code.as_ref()).to_uppercase()));
        }
    }
}
