mod as_unsafe_ref;
mod error;
mod internal;

pub mod flows;
pub mod inputs;
pub mod types;

pub use as_unsafe_ref::AsUnsafeRef;
pub use error::Error;

/// Root trait for payment gateway adapters.
///
/// # Associated Constants
///
/// * `MAX_ADDITIONAL_RECIPIENTS` - Maximum number of additional recipients for split payments.
///   The default is 0 (no split support). Override to enable split payments:
///   ```skip
///   impl Gateway for MyGateway {
///       const MAX_ADDITIONAL_RECIPIENTS: usize = 10;
///   }
///   ```
pub trait Gateway: Send + Sync {
    const MAX_ADDITIONAL_RECIPIENTS: usize = 0;
}
