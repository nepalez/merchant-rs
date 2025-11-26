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
mod access_confirmation;
mod account_number;
mod address;
mod authorization_code;
mod bank_code;
mod birth_date;
mod card_expiry;
mod card_holder_name;
mod charge_authorized;
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
mod installment_plan_id;
mod installments;
mod metadata;
mod national_id;
mod offer_id;
pub(crate) mod payment_methods;
pub(crate) mod payment_token;
pub mod payments;
mod phone_number;
mod postal_code;
mod primary_account_number;
mod reason_for_refund;
mod reason_text;
mod recipient_id;
mod recipients;
mod required_action;
mod reversal_reason;
mod routing_number;
mod stored_credential_token;
mod stored_credential_usage;
mod street_address;
mod subscription;
mod subscription_id;
mod subscription_interval;
mod token;
mod total_refund;
mod transaction;
mod transaction_id;
mod transaction_idempotence_key;
mod virtual_payment_address;

pub use access_confirmation::AccessConfirmation;
pub use account_number::AccountNumber;
pub use address::Address;
pub use authorization_code::AuthorizationCode;
pub use bank_code::BankCode;
pub use birth_date::BirthDate;
pub use card_expiry::CardExpiry;
pub use card_holder_name::CardHolderName;
pub use charge_authorized::CaptureAuthorized;
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
pub use installment_plan_id::InstallmentPlanId;
pub use installments::*;
pub use metadata::Metadata;
pub use national_id::NationalId;
pub use offer_id::OfferId;
pub use payment_methods::*;
pub use payment_token::PaymentToken;
pub use payments::*;
pub use phone_number::PhoneNumber;
pub use postal_code::PostalCode;
pub use primary_account_number::PrimaryAccountNumber;
pub use reason_for_refund::ReasonForRefund;
pub use reason_text::ReasonText;
pub use recipient_id::RecipientId;
pub use recipients::Recipients;
pub use required_action::RequiredAction;
pub use reversal_reason::ReversalReason;
pub use routing_number::RoutingNumber;
pub use stored_credential_token::StoredCredentialToken;
pub use stored_credential_usage::StoredCredentialUsage;
pub use street_address::StreetAddress;
pub use subscription::Subscription;
pub use subscription_id::SubscriptionId;
pub use subscription_interval::SubscriptionInterval;
pub use token::Token;
pub use total_refund::TotalRefund;
pub use transaction::Transaction;
pub use transaction_id::TransactionId;
pub use transaction_idempotence_key::TransactionIdempotenceKey;
pub use virtual_payment_address::VirtualPaymentAddress;
