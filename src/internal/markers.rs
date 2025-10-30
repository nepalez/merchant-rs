//! The module defines marker traits to classify payment sources
//! by their abilities to participate in specific flows.

/// Marker trait for types that can be used as payment sources.
pub(crate) trait PaymentSource {}

/// Marker trait for payment sources that can be used
/// in the internal payment flows (`ThreeDSecure`, `ImmediatePayments`, `DeferredPayments`, `Token`).
pub(crate) trait InternalPaymentSource: PaymentSource {}

/// Marker trait for payment sources that can be used
/// in the external payment flows (`CashVoucher`, `BNPL`, `CreditCard`, `InstantAccount`, `SEPAAccount`, `Token`).
pub(crate) trait ExternalPaymentSource: PaymentSource {}

/// Marker trait for payment sources that can be tokenized (exchanged to tokens).
pub(crate) trait TokenizablePaymentSource: PaymentSource {}
