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
mod account_holder_type;
mod account_number;
mod account_type;
mod address;
mod authorization_code;
mod bank_code;
mod birth_date;
mod bnpl;
mod card_expiry;
mod card_holder_name;
mod cash_voucher;
mod city;
mod country_code;
mod credit_card;
mod customer_id;
mod cvv;
mod direct_bank_account;
mod email_address;
mod full_name;
mod iban;
mod instant_bank_account;
mod merchant_initiated_type;
mod metadata;
mod money;
mod national_id;
mod payment;
mod phone_number;
mod postal_code;
mod primary_account_number;
mod reason_for_refund;
mod routing_number;
mod sepa_account;
mod street_address;
mod token;
mod transaction;
mod transaction_id;
mod transaction_idempotence_key;
mod transaction_status;
mod virtual_payment_address;
mod wallet_address;

pub use account_holder_type::AccountHolderType;
pub use account_number::AccountNumber;
pub use account_type::AccountType;
pub use address::Address;
pub use authorization_code::AuthorizationCode;
pub use bank_code::BankCode;
pub use birth_date::BirthDate;
pub use bnpl::BNPL;
pub use card_expiry::CardExpiry;
pub use card_holder_name::CardHolderName;
pub use cash_voucher::CashVoucher;
pub use city::City;
pub use country_code::CountryCode;
pub use credit_card::CreditCard;
pub use customer_id::CustomerId;
pub use cvv::CVV;
pub use direct_bank_account::DirectBankAccount;
pub use email_address::EmailAddress;
pub use full_name::FullName;
pub use iban::IBAN;
pub use instant_bank_account::InstantBankAccount;
pub use merchant_initiated_type::MerchantInitiatedType;
pub use metadata::Metadata;
pub use money::Money;
pub use national_id::NationalId;
pub use payment::Payment;
pub use phone_number::PhoneNumber;
pub use postal_code::PostalCode;
pub use primary_account_number::PrimaryAccountNumber;
pub use reason_for_refund::ReasonForRefund;
pub use routing_number::RoutingNumber;
pub use sepa_account::SEPAAccount;
pub use street_address::StreetAddress;
pub use token::Token;
pub use transaction::Transaction;
pub use transaction_id::TransactionId;
pub use transaction_idempotence_key::TransactionIdempotenceKey;
pub use transaction_status::TransactionStatus;
pub use virtual_payment_address::VirtualPaymentAddress;
pub use wallet_address::WalletAddress;
