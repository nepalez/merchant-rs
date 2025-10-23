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

mod authorizable;
mod capturable;
mod gateway;
mod protected;
mod refundable;

pub use authorizable::*;
pub use capturable::*;
pub use gateway::*;
pub use protected::*;
pub use refundable::*;
