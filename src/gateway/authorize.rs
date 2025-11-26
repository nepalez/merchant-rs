//! Authorization step: prepare a payment method for future charges.
//!
//! This step handles:
//! - Stored credential creation for bank debits (SEPA, ACH, BACS)
//! - SetupIntent for card tokenization with SCA
//! - Passthrough for other payments (no preparation needed)

use async_trait::async_trait;

use crate::types::{AccessConfirmation, Metadata, RequiredAction, payment_methods::*};
use crate::{Error, Gateway};

/// Marker trait for payment method types that can be used as input to the authorization step.
///
/// Implemented for all payment method types that can initiate the authorization flow:
/// cards, bank payments, BNPL, vouchers, etc.
pub(crate) trait OriginalPaymentMethod: PaymentMethod {}
impl OriginalPaymentMethod for BankPayment {}
impl OriginalPaymentMethod for BNPL {}
impl OriginalPaymentMethod for CashVoucher {}
impl OriginalPaymentMethod for CreditCard {}
impl OriginalPaymentMethod for CryptoPayment {}
impl OriginalPaymentMethod for DirectCarrierBilling {}
impl OriginalPaymentMethod for SEPA {}
impl OriginalPaymentMethod for Vault {}

/// Marker trait for payment method types that result from successful authorization.
///
/// Can be either:
/// - `StoredCredential` — when authorization creates a reusable token (mandate, SetupIntent)
/// - Any `OriginalPaymentMethod` — when authorization is a passthrough (no transformation)
pub(crate) trait AuthorizedPaymentMethod: PaymentMethod {}
impl AuthorizedPaymentMethod for StoredCredential {}
impl<T: OriginalPaymentMethod> AuthorizedPaymentMethod for T {}

/// Request for the authorization step.
///
/// Contains the payment method to authorize and optional confirmation data
/// from a previous customer action.
#[allow(private_bounds)]
pub struct Request<PaymentMethod: OriginalPaymentMethod> {
    /// The payment method to authorize.
    pub payment_method: PaymentMethod,

    /// Confirmation data from a completed customer action.
    ///
    /// - `None` — initial request, no prior customer action
    /// - `Some(confirmation)` — customer completed the required action (redirect, approval, etc.)
    pub confirmation: Option<AccessConfirmation>,
}

/// Response from the authorization step.
///
/// Either the authorization is completed successfully, or a customer action is required.
#[allow(private_bounds)]
pub enum Response<PaymentMethod: AuthorizedPaymentMethod> {
    /// Authorization completed successfully.
    Authorized {
        /// The authorized payment method for future charges.
        /// Readiness for use is indicated by the `verified` field.
        payment_method: PaymentMethod,

        /// Whether the payment method is verified and ready for use.
        ///
        /// - `true` — ready for charges (always `true` for a passthrough)
        /// - `false` — pending verification (e.g., micro-deposits for a bank account)
        verified: bool,

        /// Gateway-specific metadata such as a masked card number, last 4 digits, brand, or expiry.
        metadata: Metadata,
    },

    /// Customer action required to complete the authorization.
    RequiresAction(RequiredAction),
}

/// Trait for gateways that support pre-authorization with credential storage.
///
/// Implemented by gateways where the `authorize` creates a `StoredCredential`
/// (mandate for bank debits, SetupIntent for cards). Provides methods to check
/// the verification status and revoke a stored credential.
#[async_trait]
pub trait Authorize: Gateway<AuthorizedPaymentMethod = StoredCredential> {
    /// Check the current verification status of a stored credential.
    ///
    /// When the initial `authorize` call returns a credential with `verified: false`
    /// (e.g., for bank accounts pending micro-deposit verification), use this method
    /// to poll for the updated status.
    async fn check_stored_credential(
        &self,
        stored_credential: StoredCredential,
    ) -> Result<Response<StoredCredential>, Error>;

    /// Revoke a stored credential, making it unusable for future charges.
    async fn revoke_stored_credential(
        &self,
        stored_credential: StoredCredential,
    ) -> Result<(), Error>;
}
