//! The module defines nested structures
//! used to simplify the creation of secure containers.

use std::collections::HashMap;

mod address;
mod bank_payment;
mod birth_date;
mod bnpl;
mod card_expiry;
mod cash_voucher;
mod credentials;
mod credit_card;
mod crypto_payment;
mod direct_carrier;
mod distributed_amount;
mod distributed_value;
mod extended_installment_plan;
mod external_payment;
mod external_payment_data;
mod installment_plan;
mod instant_payment;
mod recipients;
mod redistributed_amount;
mod sepa;
mod stored_card;
mod subscription;
mod subscription_interval;
mod transaction;

pub use address::Address;
pub use bank_payment::{BankPayment, BankPaymentCredentials};
pub use birth_date::BirthDate;
pub use bnpl::BNPL;
pub use card_expiry::CardExpiry;
pub use cash_voucher::CashVoucher;
pub use credentials::Credentials;
pub use credit_card::CreditCard;
pub use crypto_payment::CryptoPayment;
pub use direct_carrier::DirectCarrier;
pub use distributed_amount::DistributedAmount;
pub use distributed_value::DistributedValue;
pub use extended_installment_plan::ExtendedInstallmentPlan;
pub use external_payment::ExternalPayment;
pub use external_payment_data::ExternalPaymentData;
pub use installment_plan::InstallmentPlan;
pub use instant_payment::InstantPayment;
pub use recipients::Recipients;
pub use redistributed_amount::RedistributedAmount;
pub use sepa::{SEPA, SEPACredentials};
pub use stored_card::{StoredCard, StoredCardCredentials};
pub use subscription::Subscription;
pub use subscription_interval::SubscriptionInterval;
pub use transaction::Transaction;

/// Insecure container of additional adapter-specific parameters
/// convertible to `SecureMetadata`.
pub type Metadata<'a> = HashMap<&'static str, &'a str>;
