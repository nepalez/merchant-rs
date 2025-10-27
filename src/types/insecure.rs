//! The module defines insecure types that:
//! * contains references to sensitive data,
//! * exposes sensitive data to the outside world,
//! * neither validated nor sanitized by themselves.
//!
//! # PURPOSE
//!
//! These types are used by Gateway adapters:
//! * to access the sensitive data when building requests.
//! * to construct secure responses from responses of payment systems.
//!
//! They are used by clients:
//! * to prepare secure requests to the gateway from inputs (database records, user input, etc.),
//! * to process secure responses from the gateway.
//!
//! # SECURITY CONSIDERATIONS
//!
//! Both gateway implementations and clients must ensure that exposed data are used carefully:
//! * they are neither logged nor cloned or stored by themselves.
//! * when they become part of the composed structures (like gateway requests,
//!   SQL queries, or responses to end users), those structures must be protected
//!   with the same security measures as secure types (memory tenderization, data masking, etc.).

mod address;
mod birth_date;
mod card_expiry;
mod new_payment;
mod payment_source;

use std::collections::HashMap;

pub type AccountNumber<'a> = &'a str;
pub type AuthorizationCode<'a> = &'a str;
pub type BankCode<'a> = &'a str;
pub type CardHolderName<'a> = &'a str;
pub type City<'a> = &'a str;
pub type CountryCode<'a> = &'a str;
pub type CustomerId<'a> = &'a str;
pub type CVV<'a> = &'a str;
pub type EmailAddress<'a> = &'a str;
pub type FullName<'a> = &'a str;
pub type IBAN<'a> = &'a str;
pub type MerchantReferenceId<'a> = &'a str;
/// Insecure container of additional adapter-specific parameters
/// convertible to `SecureMetadata`.
pub type Metadata<'a> = HashMap<&'static str, &'a str>;
pub type MetadataValue<'a> = &'a str;
pub type NationalId<'a> = &'a str;
pub type PhoneNumber<'a> = &'a str;
pub type PostalCode<'a> = &'a str;
pub type PrimaryAccountNumber<'a> = &'a str;
pub type ReasonForRefund<'a> = &'a str;
pub type RegionCode<'a> = &'a str;
pub type RoutingNumber<'a> = &'a str;
pub type StreetAddress<'a> = &'a str;
pub type Token<'a> = &'a str;
pub type TransactionId<'a> = &'a str;
pub type VirtualPaymentAddress<'a> = &'a str;
pub type WalletAddress<'a> = &'a str;

pub use address::Address;
pub use birth_date::BirthDate;
pub use card_expiry::CardExpiry;
pub use new_payment::NewPayment;
pub use payment_source::PaymentSource;
