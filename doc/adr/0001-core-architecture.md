# [ADR-0001]: Core Architecture and Modular Design

> **Note:** This ADR describes the initial modular architecture with separate subcrates. This decision was later superseded by [ADR-0010], which adopts a monolithic crate with feature flags instead.
>
> **Implementation Note:** While this ADR describes the architectural vision with unified `PaymentSource` enum and `Authorizable`/`Capturable` traits, the actual implementation evolved to use:
> - Marker trait hierarchy (`InternalPaymentSource`, `ExternalPaymentSource`, `TokenizablePaymentSource`) instead of unified enum - see [ADR-0012]
> - Flow-specific traits (`ImmediatePayments`, `DeferredPayments`, `ExternalPayments`) instead of `Authorizable`/`Capturable` - see [ADR-0003] and [ADR-0004]
> - `CheckTransaction` as base mandatory trait instead of `Authorizable` - see [ADR-0011]
>
> The core principles (payment abstraction, trait segregation, type safety) remain valid, but the specific trait names and architecture differ.

## Context

The merchant-rs project must provide a payment processing abstraction that works across diverse gateways (`Stripe`, `Adyen`, `PayPal`), payment methods (cards, bank transfers, digital wallets, BNPL, cryptocurrency), and value-added services (tokenization, 3D Secure, fraud detection).

Key requirements from stakeholder analysis:
- **Payment clients**: Need maximum simplicity, minimal dependencies, polymorphic interface independent of a payment type
- **Gateway implementers**: Professional developers who can manage complex dependencies and selectively implement capabilities

## Problem

The fundamental architectural question: What belongs in core versus extensions, and how should the system expose functionality to different user types?

## Decision

Adopt a facade pattern with modular workspace architecture optimized for client simplicity.

**Core (merchant-rs-core) contains:**
- Transaction lifecycle traits: `Authorizable`, `Capturable`, `Refundable`
- All payment source types: cards, bank accounts, digital wallets, BNPL, cryptocurrency
- Unified `PaymentSource` enum providing compile-time safe polymorphism
- Shared types: transaction identifiers, money, customer data, addresses
- Single `authorize()` method accepting any `PaymentSource` variant

**Rationale for including all payment types in core:** Industry analysis (`ActiveMerchant`, `Stripe SDK`, `Braintree`, `Adyen`, `Square`, `Omnipay`) shows 90% of payment libraries use a unified polymorphic method for all payment types. This prevents API fragmentation and provides true polymorphism for clients.

**Provider-specific enums excluded from core:** No `WalletProvider`, `BNPLProvider`, or `BlockchainNetwork` enums. Provider selection is a gateway adapter implementation detail, not a core contract.

**Extension subcrates provide optional capabilities:**
- merchant-rs-vault: `Tokenizable`, `CustomerVault` traits
- merchant-rs-3ds: `ThreeDSecure` trait
- merchant-rs-fraud: `FraudDetection` trait
- merchant-rs-recurring: `RecurringPayments` trait

**Facade crate (merchant-rs) for clients:**
Single dependency with optional features. Clients import `merchant-rs = "1.0"` and optionally enable features like `vault`, `three-ds`, `fraud`. The facade re-exports core plus enabled extensions.

**Gateway adapters:**
External crates that depend directly on `merchant-rs-core` and selected extension subcrates. Each adapter implements core traits plus relevant extension traits based on gateway capabilities.

## Consequences

### Pros
- Client simplicity: single dependency, single `authorize()` interface for all payment types
- True polymorphism: client code independent of a payment source type via enum
- Compile-time safety: Rust enum with pattern matching, no runtime type discrimination
- Adapter flexibility: implementers cherry-pick only necessary extension dependencies
- No fragmentation: unified payment flow, not separate APIs per payment type
- Industry alignment: matches architectural patterns of major payment libraries

### Cons
- Core contains all payment source types, not just traditional card/bank
- Adding new payment source type requires core modification
- Clients compile payment types they may not use (mitigated by fast Rust compilation)

## Alternatives Considered

### Feature flags for payment types
Considered using Cargo features to make payment types optional (`features = ["card", "bank", "crypto"]`). Rejected because feature flags are additive and transitive dependencies can enable unwanted features, creating difficult-to-debug compilation matrices. Industry uses unified payment types, not conditional compilation.

### Modular payment type subcrates
Considered `merchant-rs-payment-card`, `merchant-rs-payment-bank`, etc. as separate crates. Rejected because it forces clients to manage multiple dependencies and prevents true polymorphism (no single PaymentSource enum spanning all types without circular dependencies).
