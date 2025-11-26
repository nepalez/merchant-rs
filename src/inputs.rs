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
mod distributed_value;
mod external_payment;
mod external_payment_data;
mod installments;
mod installments_br;
mod installments_gcc;
mod installments_in;
mod installments_jp;
mod instant_payment;
mod payment;
mod recipients;
mod required_action;
mod reversal_reason;
mod sepa;
mod split_payment;
mod stored_credential;
mod stored_credential_usage;
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
pub use distributed_value::DistributedValue;
pub use external_payment::ExternalPayment;
pub use external_payment_data::ExternalPaymentData;
pub use installments::Installments;
pub use installments_br::InstallmentsBR;
pub use installments_gcc::InstallmentsGCC;
pub use installments_in::InstallmentsIN;
pub use installments_jp::InstallmentsJP;
pub use instant_payment::InstantPayment;
pub use payment::Payment;
pub use recipients::Recipients;
pub use required_action::RequiredAction;
pub use reversal_reason::ReversalReason;
pub use sepa::{SEPA, SEPACredentials};
pub use split_payment::SplitPayment;
pub use stored_credential::StoredCredential;
pub use stored_credential_usage::StoredCredentialUsage;
pub use subscription::Subscription;
pub use subscription_interval::SubscriptionInterval;
pub use transaction::Transaction;

/// Insecure container of additional adapter-specific parameters
/// convertible to `SecureMetadata`.
pub type Metadata<'a> = HashMap<&'static str, &'a str>;
