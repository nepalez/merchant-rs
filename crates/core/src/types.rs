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
mod authorization_code;
mod authorization_id;
mod bank_account_details;
mod bank_name;
mod card_details;
mod card_expiry;
mod card_holder_name;
mod customer_id;
mod cvv;
mod merchant_reference_id;
mod money;
mod payment_source;
mod payment_token;
mod primary_account_number;
mod reason_for_refund;
mod refund_id;
mod routing_number;
mod transaction_id;
mod transaction_status;

pub use account_number::AccountNumber;
pub use authorization_code::AuthorizationCode;
pub use authorization_id::AuthorizationId;
pub use bank_account_details::BankAccountDetails;
pub use bank_name::BankName;
pub use card_details::CardDetails;
pub use card_expiry::CardExpiry;
pub use card_holder_name::CardHolderName;
pub use customer_id::CustomerId;
pub use cvv::CVV;
pub use iso_currency::Currency;
pub use merchant_reference_id::MerchantReferenceId;
pub use money::Money;
pub use payment_source::PaymentSource;
pub use payment_token::PaymentToken;
pub use primary_account_number::PrimaryAccountNumber;
pub use reason_for_refund::ReasonForRefund;
pub use refund_id::RefundId;
pub use routing_number::RoutingNumber;
pub use rust_decimal::Decimal;
pub use transaction_id::TransactionId;
pub use transaction_status::TransactionStatus;
