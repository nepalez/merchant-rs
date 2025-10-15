//! Defines all **canonical data structures** used for communication between the
//! `merchant-rs-core` and its gateway adapters.
//!
//! This module ensures **type safety** and consistency across all financial operations.
//! It includes fundamental types for transactions (requests/responses), financial
//! entities (currencies, amounts, tokens), and payment sources (cards, bank accounts).
//!
//! By making these structures canonical, the core decouples the business logic
//! from the specific data formats required by external Payment Gateways (PAGs),
//! upholding the core's role as a stable abstraction layer.

mod account_number;
mod account_type;
mod address;
mod authorization_code;
mod bank_code;
mod birth_date;
mod card_expiry;
mod card_holder_name;
mod city;
mod country_code;
mod customer_category;
mod customer_id;
mod cvv;
mod email_address;
mod full_name;
mod iban;
mod merchant_reference_id;
mod money;
mod national_id;
mod payment_source;
mod phone_number;
mod postal_code;
mod primary_account_number;
mod reason_for_refund;
mod region_code;
mod routing_number;
mod street_address;
mod token;
mod transaction_id;
mod transaction_status;
mod virtual_payment_address;
mod wallet_address;

pub use account_number::AccountNumber;
pub use account_type::AccountType;
pub use address::Address;
pub use authorization_code::AuthorizationCode;
pub use bank_code::BankCode;
pub use birth_date::BirthDate;
pub use card_expiry::CardExpiry;
pub use card_holder_name::CardHolderName;
pub use city::City;
pub use country_code::CountryCode;
pub use customer_category::CustomerCategory;
pub use customer_id::CustomerId;
pub use cvv::CVV;
pub use email_address::EmailAddress;
pub use full_name::FullName;
pub use iban::IBAN;
pub use iso_currency::Currency;
pub use merchant_reference_id::MerchantReferenceId;
pub use money::Money;
pub use national_id::NationalId;
pub use payment_source::PaymentSource;
pub use phone_number::PhoneNumber;
pub use postal_code::PostalCode;
pub use primary_account_number::PrimaryAccountNumber;
pub use reason_for_refund::ReasonForRefund;
pub use region_code::RegionCode;
pub use routing_number::RoutingNumber;
pub use rust_decimal::Decimal;
pub use street_address::StreetAddress;
pub use token::Token;
pub use transaction_id::TransactionId;
pub use transaction_status::TransactionStatus;
pub use virtual_payment_address::VirtualPaymentAddress;
pub use wallet_address::WalletAddress;

pub type Metadata = std::collections::HashMap<String, String>;
