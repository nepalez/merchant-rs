use crate::inputs::PaymentData;

/// Insecure enum representing a payment either as a plain source data
/// or a 3D secure token.
pub enum Payment<'a, Source: 'a> {
    /// The source of the payment to charge funds from
    Plain(PaymentData<'a, Source>),
    /// The amount to charge
    Secure(&'a str),
}
