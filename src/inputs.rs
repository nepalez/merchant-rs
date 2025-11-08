//! The module defines nested structures
//! used to simplify the creation of secure containers.

mod address;
mod bank_payment;
mod birth_date;
mod bnpl;
mod card_expiry;
mod cash_voucher;
mod credentials;
mod credit_card;
mod crypto_payment;
mod destinations;
mod direct_carrier;
mod external_payment;
mod external_payment_data;
mod instant_payment;
mod payment;
mod recurrent_payment;
mod sepa;
mod stored_card;
mod subscription;
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
pub use destinations::Destinations;
pub use direct_carrier::DirectCarrier;
pub use external_payment::ExternalPayment;
pub use external_payment_data::ExternalPaymentData;
pub use instant_payment::InstantPayment;
pub use payment::Payment;
pub use recurrent_payment::RecurrentPayment;
pub use sepa::{SEPA, SEPACredentials};
pub use stored_card::{StoredCard, StoredCardCredentials};
pub use subscription::Subscription;
pub use transaction::Transaction;

use std::collections::HashMap;
/// Insecure container of additional adapter-specific parameters
/// convertible to `SecureMetadata`.
pub type Metadata<'a> = HashMap<&'static str, &'a str>;
