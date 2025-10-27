//! The module defines secure types that protect sensitive data
//! from accidental exposure in logs and other unencrypted storages,
//! as well as ensure their zeroization upon a drop.
//!
//! # SECURITY CONSIDERATIONS
//!
//! The data security is enforced by using these types in all public APIs
//! represented by the corresponding traits.
//!
//! The gateway implementers can access the sensitive content of requests
//! via exposing them in the controlled environment (see the `Protected` trait definition).
//! In the same way clients can access the sensitive content of gateway responses.
//!
//! The use of these types increases the security of the whole system because
//! it clearly separates the responsibility of handling sensitive data
//! between clients and gateways. If the gateway implementation is secure,
//! the client can only care about the protection of the data before sending them
//! to the gateway's requests and after handling its responses.

mod account_number;
mod address;
mod authorization_code;
mod bank_code;
mod birth_date;
mod card_expiry;
mod card_holder_name;
mod city;
mod country_code;
mod customer_id;
mod cvv;
mod email_address;
mod full_name;
mod iban;
mod merchant_reference_id;
mod metadata;
mod national_id;
mod new_payment;
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
mod virtual_payment_address;
mod wallet_address;

pub use account_number::AccountNumber;
pub use address::Address;
pub use authorization_code::AuthorizationCode;
pub use bank_code::BankCode;
pub use birth_date::BirthDate;
pub use card_expiry::CardExpiry;
pub use card_holder_name::CardHolderName;
pub use city::City;
pub use country_code::CountryCode;
pub use customer_id::CustomerId;
pub use cvv::CVV;
pub use email_address::EmailAddress;
pub use full_name::FullName;
pub use iban::IBAN;
pub use merchant_reference_id::MerchantReferenceId;
pub use metadata::Metadata;
pub use national_id::NationalId;
pub use new_payment::NewPayment;
pub use payment_source::PaymentSource;
pub use phone_number::PhoneNumber;
pub use postal_code::PostalCode;
pub use primary_account_number::PrimaryAccountNumber;
pub use reason_for_refund::ReasonForRefund;
pub use region_code::RegionCode;
pub use routing_number::RoutingNumber;
pub use street_address::StreetAddress;
pub use token::Token;
pub use transaction_id::TransactionId;
pub use virtual_payment_address::VirtualPaymentAddress;
pub use wallet_address::WalletAddress;
