//! Payment types for different distribution models.

#[allow(clippy::module_inception)]
mod payment;
mod split_payment;

// --- Types ---

pub use payment::Payment;
pub use split_payment::SplitPayment;

// --- Marker Traits ---

/// Marker trait for payment types.
pub(crate) trait PaymentMarker {
    type PaymentMethod: super::PaymentMethod;
}

impl<P: super::PaymentMethod> PaymentMarker for Payment<P> {
    type PaymentMethod = P;
}

impl<P: super::PaymentMethod> PaymentMarker for SplitPayment<P> {
    type PaymentMethod = P;
}
