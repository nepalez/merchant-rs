# [ADR-0010]: Monolithic Architecture with Feature Flags

## Context

The merchant-rs project requires a strategy for organizing code across payment flows (authorize, capture, refund, fraud detection, recurring payments, webhooks) while maintaining sealed traits for security and minimizing compilation overhead for gateway adapters and clients.

Key considerations:
- Gateway adapters implement only supported flows (Stripe supports authorize+capture but not refunds)
- Clients use only necessary flows (simple e-commerce needs authorize+capture, not fraud detection)
- Internal sealed traits (`AsUnsafeRef`, `Sanitized`, `Validated`, `Masked`) must remain `pub(crate)` for security
- No heavy dependencies in core (validation uses lightweight crates, ML libraries belong in adapters)

## Problem

Should the library be organized as multiple subcrates (core, flows, utilities) or as a single monolithic crate? How to share sealed internal traits while allowing selective compilation of flows?

## Decision

Adopt single monolithic `merchant-rs` crate with Cargo feature flags for conditional compilation of flows.

**Crate structure:**
- `internal/` module remains `pub(crate)` with sealed traits
- `types/` module compiles always (secure/insecure types, Money, TransactionId, etc.)
- `flows/` module uses feature flags for conditional compilation
- Each flow feature conditionally compiles: trait definition, Request/Response types, and flow-specific types

**Feature flags strategy:**
```toml
[features]
default = ["authorize"]
authorize = []
capture = ["authorize"]
refund = ["capture"]
fraud = []
recurring = []
webhooks = []
```

Gateway adapters and clients select only the necessary features, reducing type checking overhead without sacrificing sealed trait security.

## Alternatives Considered

### Separate core and facade crates
Split into `merchant-rs-core` (implementation) and `merchant-rs` (facade with re-exports).

**Rejection:** Facade adds unnecessary complexity without benefit. If facade provides no additional convenience API, it serves no purpose. Core should be published directly as the single public crate.

### Subcrates per flow
Separate crates (`merchant-rs-flow-auth`, `merchant-rs-flow-capture`, etc.) with shared `merchant-rs-internal` for sealed traits.

**Rejection:** Sealed traits require `pub(crate)` visibility, which only works within a single crate. Making `AsUnsafeRef` or other security-critical traits public (even with `#[doc(hidden)]`) violates security assumptions by allowing end-users to implement sensitive traits.

## Consequences

### Pros
- Sealed traits remain sealed (`pub(crate)`) in single crate
- Unused flows are not compiled (faster type checking and reduced compilation time)
- Simple architecture (one crate, transparent for security audit)
- Gateway adapters choose only supported flows
- Clear import paths (`use merchant_rs::*`)
- Industry pattern (ActiveMerchant is monolithic, Omnipay publishes core directly)

### Cons
- A single crate contains all flows (acceptable—flows only declare interfaces and types)
- Feature flag dependencies require documentation (capture requires authorize, refund requires capture)
