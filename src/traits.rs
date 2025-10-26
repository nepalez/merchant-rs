//! Defines the **asynchronous, modular trait contracts** for payment gateway adapters.
//!
//! These traits serve as the core's **Gateway Abstraction Layer**, allowing different
//! payment providers (e.g., Stripe, Adyen) to plug into the system without
//! modifying client-side business logic.
//!
//! The interface is split into granular, capability-based traits
//! (`Authorizable`, `Capturable`, `Refundable`, `Tokenizable`) to strictly adhere
//! to the **Interface Segregation Principle (ISP)**. This prevents adapters from
//! being forced to implement methods they do not support (e.g., a one-step payment
//! adapter does not need to implement the optional `Capturable` trait).

mod gateway;
mod protected;

pub mod authorizable;
pub mod cancellable;
pub mod capturable;
mod recoverable;
pub mod refundable;

pub use authorizable::Authorizable;
pub use capturable::Capturable;
pub use gateway::*;
pub use protected::Protected;
pub use recoverable::Recoverable;
pub use refundable::Refundable;
