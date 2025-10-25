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
mod account_type;

mod customer_category;
mod money;
mod transaction_status;

pub mod insecure;
pub mod secure;

pub use account_type::AccountType;
pub use customer_category::CustomerCategory;
pub use money::Money;
pub use transaction_status::TransactionStatus;
