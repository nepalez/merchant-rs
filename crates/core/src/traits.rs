//! Defines the **asynchronous, modular trait contracts** for payment gateway adapters.
//!
//! These traits serve as the core's **Gateway Abstraction Layer**, allowing different
//! payment providers (e.g., Stripe, Adyen) to plug into the system without
//! modifying client-side business logic.
//!
//! The interface is split into granular, capability-based traits
//! (`Authorizable`, `Capturable`, `Refundable`, `Tokenizable`) to strictly adhere
//! to the **Interface Segregation Principle (ISP)**. This prevents adapters from
//! being forced to implement methods they do not support (e.g., a one-step payment
//! adapter does not need to implement the optional `Capturable` trait).

use async_trait::async_trait;
use crate::error::Result;
use crate::types::*;

/// Trait that allows implementing types to be initialized with configuration.
/// This is a minimal requirement for any Adapter.
pub trait Gateway {
    /// Returns the unique, canonical identifier for the payment gateway.
    /// Example: "Stripe", "Adyen", "PayPal".
    fn id(&self) -> &str;
}

/// TODO: to be guarded by feature flag 'standard-transactions'.
/// Core trait for initiating a payment transaction (Authorize or Sale) and subsequent void.
/// Every standard payment gateway adapter MUST implement this trait.
#[async_trait]
pub trait Authorizable: Gateway {
    /// Reserves funds (Auth) or immediately debits funds (Sale/Purchase).
    async fn authorize(
        &self,
        request: AuthorizationRequest,
    ) -> Result<AuthorizationResponse>;

    /// Cancels a pending authorization, releasing the reserved funds, or reverses a
    /// recently processed one-step transaction (Sale/Purchase) before settlement.
    ///
    /// The 'void' operation is mandatory here because it represents the immediate
    /// ability to retract the action initiated by 'authorize' before the funds
    /// are permanently settled by the payment network (which is actual
    /// for 1-step flows as well).
    async fn void(&self, request: VoidRequest) -> Result<VoidResponse>;
}

/// TODO: to be guarded by feature flag 'standard-transactions'.
/// Optional trait for payment gateways that support completing a two-step flow.
///
/// An adapter must implement this ONLY if it supports the two-step Auth -> Capture model.
/// Gateways that only support Sale/Purchase should NOT implement this trait.
#[async_trait]
pub trait Capturable: Authorizable {
    /// Confirms and debits the previously authorized funds.
    async fn capture(&self, request: CaptureRequest) -> Result<CaptureResponse>;
}

/// TODO: to be guarded by feature flag 'standard-transactions'.
/// Trait for payment gateways that support the return of funds to a customer.
#[async_trait]
pub trait Refundable: Gateway {
    /// Initiates the return of funds to the customer.
    async fn refund(&self, request: RefundRequest) -> Result<RefundResponse>;
}

/// TODO: to be guarded by feature flag 'tokenization'.
/// Trait for services that can convert raw payment details into a secure token.
#[async_trait]
pub trait Tokenizable: Gateway {
    /// Converts raw payment details into a secure, opaque token string.
    async fn tokenize(
        &self,
        request: TokenizationRequest,
    ) -> Result<TokenizationResponse>;
}
