//! Defines all **canonical data structures** used for communication between the
//! `merchant-rs-core` and its gateway adapters.
//!
//! This module ensures **type safety** and consistency across all financial operations.
//! It includes fundamental types for transactions (requests/responses), financial
//! entities (currencies, amounts, tokens), and payment methods (cards, bank accounts).
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
mod card_expiry;
mod card_holder_name;
mod city;
mod country_code;
mod credentials;
mod customer_id;
mod cvv;
mod distributed_value;
mod email_address;
mod external_payment;
mod external_payment_data;
mod full_name;
mod iban;
mod merchant_initiated_type;
mod metadata;
mod national_id;
mod payment;
mod payment_methods;
pub(crate) mod payment_token;
mod phone_number;
mod postal_code;
mod primary_account_number;
mod reason_for_refund;
mod recipient_id;
mod recipients;
mod recurrent_payment;
mod routing_number;
mod stored_credential_usage;
mod street_address;
mod subscription;
mod subscription_id;
mod subscription_interval;
mod subscription_status;
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
pub use card_expiry::CardExpiry;
pub use card_holder_name::CardHolderName;
pub use city::City;
pub use country_code::CountryCode;
pub use credentials::Credentials;
pub use customer_id::CustomerId;
pub use cvv::CVV;
pub use distributed_value::DistributedValue;
pub use email_address::EmailAddress;
pub use external_payment::ExternalPayment;
pub use external_payment_data::ExternalPaymentData;
pub use full_name::FullName;
pub use iban::IBAN;
pub use merchant_initiated_type::MerchantInitiatedType;
pub use metadata::Metadata;
pub use national_id::NationalId;
pub use payment::Payment;
pub use payment_methods::*;
pub use payment_token::PaymentToken;
pub use phone_number::PhoneNumber;
pub use postal_code::PostalCode;
pub use primary_account_number::PrimaryAccountNumber;
pub use reason_for_refund::ReasonForRefund;
pub use recipient_id::RecipientId;
pub use recipients::Recipients;
pub use recurrent_payment::RecurrentPayment;
pub use routing_number::RoutingNumber;
pub use stored_credential_usage::StoredCredentialUsage;
pub use street_address::StreetAddress;
pub use subscription::Subscription;
pub use subscription_id::SubscriptionId;
pub use subscription_interval::SubscriptionInterval;
pub use subscription_status::SubscriptionStatus;
pub use token::Token;
pub use transaction::Transaction;
pub use transaction_id::TransactionId;
pub use transaction_idempotence_key::TransactionIdempotenceKey;
pub use transaction_status::TransactionStatus;
pub use virtual_payment_address::VirtualPaymentAddress;
pub use wallet_address::WalletAddress;
