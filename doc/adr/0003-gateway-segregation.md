# [ADR-0003]: Gateway Trait Segregation

## Context

Payment gateways have varying capabilities. Some support two-step authorization with delayed capture (`Stripe`, `Braintree`), others only immediate settlement (many crypto processors). Some allow refunds, others do not. Some support tokenization, others require client-side token creation.

## Problem

The architectural question: Should gateway adapters implement a monolithic interface with all possible operations, or segregated traits based on actual capabilities?

## Decision

Apply Interface Segregation Principle: segregate traits by capability rather than requiring a monolithic gateway interface.

**Core transaction traits:**
- `Authorizable`: mandatory for all gateways (authorize and void operations)
- `Capturable`: optional, only for gateways supporting two-step flows
- `Refundable`: optional, only for gateways supporting refunds

**Extension traits follow the same principle** (per [ADR-0001]):
- `Tokenizable` (`vault` extension): only for gateways with server-side tokenization
- `ThreeDSecure` (`3ds` extension): only for gateways supporting authentication
- `CustomerVault` (`vault` extension): only for gateways with customer storage

Gateway adapters implement only the traits matching their actual capabilities. This enables the modular extension architecture described in the [ADR-0001].

## Consequences

### Pros
- Adapters not forced to implement unsupported operations with stub/error responses
- Clear compile-time contract: trait presence indicates capability
- Easier to understand adapter capabilities through implemented traits
- Supports modular extensions without core dependencies

### Cons
- Client code must check trait bounds at compile time or handle missing capabilities at runtime
- More traits to understand compared to single monolithic interface

## Alternatives Considered

### Monolithic Gateway trait
Single trait with all operations (`authorize`, `capture`, `refund`, `tokenize`, etc.). Rejected because it forces adapters to implement unsupported operations, typically returning "not supported" errors at runtime rather than preventing misuse at compile time. Violates Interface Segregation Principle and prevents clean extension architecture.