# merchant-rs

The Async, Type-Safe Payment Abstraction for Rust.

This is a foundational library for building reliable, unified payment APIs in Rust.
Its purpose is to define the contracts (traits), data structures, and error handling necessary for payment processing across diverse gateways and payment methods.

This crate contains zero network logic, ensuring it is lightweight and highly focused on type safety and architectural consistency. All gateway-specific implementations (e.g., `Stripe`, `NomuPay`) are handled by separate, optional adapter crates.

## Key Features

**Async-First Contracts**: Defines all gateway operations using #[async_trait], ensuring non-blocking network calls and high throughput essential for modern Rust web services.

**Zero-Cost Abstraction**: Enables the use of trait objects (`Box<dyn PaymentGateway>`), allowing for dynamic gateway switching at runtime without the need to recompile the core business logic.

**Financial Precision**: Strictly enforces type safety (uses the `currencies` crate under the hood) for all currency amounts, eliminating floating-point errors.

**Structured Errors**: Provides a unified `Error` type, allowing downstream applications to handle errors from any provider consistently.

**Data Security**: Implements graduated protection for sensitive data (PCI DSS compliant). All sensitive types (PAN, CVV, account numbers) are wrapped in secure newtypes with memory zeroization on drop and masked debug output. Sealed traits prevent accidental exposure while maintaining compile-time safety guarantees.

## Core Contracts (Traits)
These #[async_trait] definitions form the mandatory contract for all payment adapters:

* `PaymentGateway` -- Handles fundamental transaction flows.
* `VaultingGateway` -- Manages sensitive payment instrument data (PCI compliance)
* `ThreeDSecureGateway` -- Manages complex, multistep 3DS2 authentication flows.
* `RedirectGateway`	-- Manages APM (Alternative Payment Methods) and webhook-driven flows.

## Core Data Structures
This crate defines the essential structures used for communication across the entire ecosystem:

* `AuthorizationRequest` -- Contains all necessary data to request authorization. Uses strong types like `Decimal` and `Currency`.
* `TransactionResponse` -- The standardized output from any gateway operation, including status (`Success | Failure | Pending`), gateway-specific `ID`, and authorization code.
* `Error` -- A robust enum covering all failure categories: `Network, Validation, ProviderError, Fraud, etc.`.

## Payment Instruments

* `PaymentSource` -- an enum abstracting the payment method: `Card(CardDetails)`, `Token(String)`, `EWallet(EWalletDetails)`.
* `CardDetails` -- Structured data for card information (securely handled, often via a tokenization provider).

## Usage Example (Conceptual)
This demonstrates how a downstream application uses the core contracts to remain gateway-agnostic:

```rust
use merchant_rs::traits::PaymentGateway;
use merchant_rs::types::{PurchaseRequest, TransactionResponse};
use merchant_rs::errors::Error;
use std::sync::Arc;

// In a typical web service, you would inject the specific implementation
// wrapped in a dynamic trait object (Dyn Trait).
async fn execute_transaction(
    gateway: Arc<dyn PaymentGateway + Send + Sync>,
    request: PurchaseRequest
) -> Result<TransactionResponse, Error> {
    // The core trait method is called using .await
    let response = gateway.purchase(&request).await?;
    // ... process the unified response
    Ok(response)
}
```

### Related Crates
To gain full "Batteries Included" functionality, you must combine this crate with an adapter crate:

`merchant-rs-{adapter}`: (Optional) Contains concrete implementations for Stripe, NomuPay, etc.
`merchant-rs-testing`: Provides the essential MockGateway for unit testing your business logic.
