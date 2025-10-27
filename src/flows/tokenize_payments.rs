/// Optional trait for payment gateways that support tokenizing a payment
/// by exchanging the payment details for a secure token.
///
/// Later the token can be used to charge the payment via `PaymentSource::TokenizedPayment`.
///
/// This method can be used to support 3D Secure payments.
pub trait TokenizePayments {}
