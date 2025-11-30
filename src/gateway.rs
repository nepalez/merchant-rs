//! Root trait and pipeline methods for payment gateway adapters.
//!
//! This module defines the universal payment pipeline where each gateway implements
//! the same sequence of steps. Steps that are not applicable for a specific gateway
//! act as passthrough (returning immediately with a marker).
//!
//! The pipeline ensures **interchangeability**: any gateway implementing a flow
//! can be replaced with another gateway implementing the same flow without
//! changing client code.

pub mod authorize;
pub mod secure;

use async_trait::async_trait;

use crate::types::payment_methods::PaymentMethod;
use crate::types::{InstallmentsMarker, PaymentMarker};
use authorize::AuthorizedPaymentMethod;
use secure::SecuredPaymentMarker;

/// Root trait for payment gateway adapters.
///
/// Defines the associated types and pipeline methods that every gateway must implement.
/// The pipeline consists of sequential steps:
///
/// 1. **authorize** — prepare a payment method for future charges (mandate, SetupIntent, or passthrough)
/// 2. **prepare** — secure a specific payment (3DS/SCA for CIT, passthrough for MIT)
/// 3. **reserve** — reserve funds (deferred) or return passthrough (immediate)
/// 4. **charge** — execute the final payment
///
/// Each step returns either a success result or `RequiresAction` for customer interaction.
#[async_trait]
#[allow(private_bounds)]
pub trait Gateway: Send + Sync {
    /// The payment structure supported by this gateway (`Payment` or `SplitPayment`).
    type Payment: PaymentMarker;

    /// The installment options supported by this gateway.
    ///
    /// Can be `NoInstallments` for gateways without installment support,
    /// or a region-specific type (`InstallmentsBR`, `InstallmentsIN`, etc.).
    type Installments: InstallmentsMarker;

    /// The input payment method type accepted by this gateway.
    ///
    /// Examples: `CreditCard`, `BankPayment`, `SEPA`, `BNPL`.
    type PaymentMethod: PaymentMethod;

    /// The output payment method type after successful authorization.
    ///
    /// Either `StoredCredential` (for mandate/SetupIntent) or the original
    /// payment method type unchanged (passthrough).
    type AuthorizedPaymentMethod: AuthorizedPaymentMethod;

    /// The output payment method type after successful securitization (3DS).
    ///
    /// Either `SecuredPayment` (for CIT card payments requiring 3DS) or the original
    /// authorized payment method type unchanged (a passthrough for MIT or non-card).
    type SecuredPaymentMethod: SecuredPaymentMarker;

    /// Authorize a payment method for future charges.
    ///
    /// This step prepares access to funds:
    /// - **Bank debits (SEPA/ACH/BACS)**: creates a mandate (legal authorization)
    /// - **Cards**: creates a SetupIntent (tokenization + SCA for MIT)
    /// - **Other methods**: passthrough (no preparation needed)
    ///
    /// # Returns
    ///
    /// * `Response::Authorized` — payment method ready for future charges
    /// * `Response::RequiresAction` — customer action needed (redirect, approval)
    async fn authorize(
        &self,
        request: authorize::Request<Self::PaymentMethod>,
    ) -> Result<authorize::Response<Self::AuthorizedPaymentMethod>, crate::Error>;

    /// Secure a payment method via 3D Secure authentication.
    ///
    /// This step handles Strong Customer Authentication (SCA) for card payments:
    /// - **CIT card payments**: performs 3DS authentication (frictionless or challenge)
    /// - **MIT payments**: passthrough (authentication not required)
    /// - **Non-card payments**: passthrough (3DS not applicable)
    ///
    /// # Returns
    ///
    /// * `Response::Secured` — payment method secured, ready for charge
    /// * `Response::RequiresAction` — customer action needed (3DS challenge)
    async fn secure(
        &self,
        request: secure::Request<Self::AuthorizedPaymentMethod>,
    ) -> Result<secure::Response<Self::SecuredPaymentMethod>, crate::Error>;
}
