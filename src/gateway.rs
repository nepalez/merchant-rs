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

use async_trait::async_trait;

use crate::types::{InstallmentsMarker, PaymentMarker};
use authorize::{AuthorizedPaymentMethod, OriginalPaymentMethod};

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
    type OriginalPaymentMethod: OriginalPaymentMethod;

    /// The output payment method type after successful authorization.
    ///
    /// Either `StoredCredential` (for mandate/SetupIntent) or the original
    /// payment method type unchanged (passthrough).
    type AuthorizedPaymentMethod: AuthorizedPaymentMethod;

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
        request: authorize::Request<Self::OriginalPaymentMethod>,
    ) -> Result<authorize::Response<Self::AuthorizedPaymentMethod>, crate::Error>;
}
