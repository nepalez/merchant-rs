//! The module defines marker traits to classify payment methods
//! by their abilities to participate in specific flows.

use std::fmt;

mod bank_payment;
mod bnpl;
mod cash_voucher;
mod credit_card;
mod crypto_payment;
mod direct_carrier_billing;
mod instant_payment;
mod sepa;
mod stored_card;
mod vault;

// --- Marker traits  ---

/// Marker trait for types that can be used as payment methods.
pub(crate) trait PaymentMethod: fmt::Debug {}

/// Marker trait for payment methods that can be used
/// in the internal payment flows (`ThreeDSecure`, `ImmediatePayments`, `DeferredPayments`, `Token`).
pub(crate) trait InternalPaymentMethod: PaymentMethod {}

/// Marker trait for payment methods that can be used
/// in the external payment flows (`CashVoucher`, `BNPL`, `CreditCard`, `InstantAccount`, `SEPAAccount`, `Token`).
pub(crate) trait ExternalPaymentMethod: PaymentMethod {}

/// Marker trait for payment methods that can be stored in gateway vault (exchanged to tokens).
pub(crate) trait StorablePaymentMethod: InternalPaymentMethod {}

// --- Types ---

pub use bank_payment::BankPayment;
pub use bnpl::BNPL;
pub use cash_voucher::CashVoucher;
pub use credit_card::CreditCard;
pub use crypto_payment::CryptoPayment;
pub use direct_carrier_billing::DirectCarrierBilling;
pub use instant_payment::InstantAccount;
pub use sepa::SEPA;
pub use stored_card::StoredCard;
pub use vault::Vault;
