//! Securitization step: 3D Secure authentication for card payments.
//!
//! This step handles:
//! - 3DS authentication for CIT card payments (PSD2/SCA compliance)
//! - Passthrough for MIT and non-card payments

use crate::gateway::authorize::AuthorizedPaymentMethod;
use crate::types::{BrowserInfo, Confirmation, RequiredAction, SecuredPayment};

/// Marker trait for payment types that passed the secure() step.
///
/// Provides access to 3DS authentication result data. Returns `None` for all
/// fields when the payment is a passthrough (MIT or non-card).
pub trait SecuredPaymentMarker {}

// Passthrough: all AuthorizedPaymentMethod types automatically implement SecuredPaymentMarker
impl<T: AuthorizedPaymentMethod> SecuredPaymentMarker for T {}

impl SecuredPaymentMarker for SecuredPayment {}

/// Request for the secure step.
#[allow(private_bounds)]
pub struct Request<P: AuthorizedPaymentMethod> {
    /// The authorized payment method to secure.
    pub payment_method: P,
    /// Browser information for risk-based authentication.
    pub browser_info: Option<BrowserInfo>,
    /// Confirmation data from a completed customer action (3DS challenge).
    pub confirmation: Option<Confirmation>,
}

/// Response from the secure step.
#[allow(private_bounds)]
pub enum Response<P: SecuredPaymentMarker> {
    /// Payment secured successfully.
    Secured(P),
    /// Customer action required (3DS challenge).
    RequiresAction(RequiredAction),
}
