//! The module defines nested structures
//! used to simplify the creation of secure containers.

mod address;
mod bank_account;
mod birth_date;
mod bnpl;
mod card_expiry;
mod cash_voucher;
mod credit_card;
mod external_payment;
mod external_payment_data;
mod instant_account;
mod payment;
mod payment_data;
mod sepa_account;
mod transaction;

pub use address::Address;
pub use bank_account::BankAccount;
pub use birth_date::BirthDate;
pub use bnpl::BNPL;
pub use card_expiry::CardExpiry;
pub use cash_voucher::CashVoucher;
pub use credit_card::CreditCard;
pub use external_payment::ExternalPayment;
pub use external_payment_data::ExternalPaymentData;
pub use instant_account::InstantAccount;
pub use payment::Payment;
pub use payment_data::PaymentData;
pub use sepa_account::SEPAAccount;
pub use transaction::Transaction;

use std::collections::HashMap;
/// Insecure container of additional adapter-specific parameters
/// convertible to `SecureMetadata`.
pub type Metadata<'a> = HashMap<&'static str, &'a str>;
