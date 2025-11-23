# [ADR-0014]: Separation of Input and Type Structures

## Context

Payment clients need to write code working across compatible gateways without
knowing their specific type requirements. Gateway trait methods accept
associated types: `<Self as Gateway>::Payment` resolves to different concrete
types per adapter (`Payment<P>` vs `SplitPayment<P>`, `Installments` vs
`InstallmentsBR`).

Validation before allocation wastes memory on invalid inputs. Owned data in
client structures expands attack surface (requires memory protection before
validation).

## Problem

How to enable universal client code for compatible gateways while preventing
incompatible usage at compile time, ensuring memory efficiency and security?

## Decision

Separate input structures (client API) from type structures (gateway API).

**Inputs module (`inputs/`):**
- Client-facing structures: `Payment<'a, M>`, `CreditCard<'a>`
- Generic over payment method, borrowed data (`&'a str`)
- No validation, security protection, or ownership
- Designed for compatible conversion to multiple type variants
- Re-exported through crate root for direct client access

**Types module (`types/`):**
- Gateway-facing structures: `types::Payment<P>`, `types::SplitPayment<P>`
- Owned data with validation, sanitization, security protection
- Enforce domain invariants (Validated, Sanitized, Masked)
- Consumed by gateway trait methods
- Accessed through `types::` namespace

**Re-export strategy:** Inputs re-exported at crate root (`use
merchant_rs::Payment`), types accessed via namespace (`use
merchant_rs::types::Payment`). This signals that clients primarily work with
inputs, while types are internal gateway contracts.

**Compatible conversion:** Single input converts to multiple compatible type
variants via `TryFrom`. Incompatible conversions fail at compile time (missing
implementation). Validation and allocation happen once during conversion.

```rust
use merchant_rs::{Payment, CreditCard};  // Inputs from root
use merchant_rs::types;                   // Types from namespace

// Client constructs input
let input = Payment {
    payment_method: CreditCard { /* ... */ },
    currency: Currency::USD,
    total_amount: Decimal::new(10000, 2),
    base_amount: Decimal::new(9500, 2),
    idempotence_key: "payment-123",
};

// Gateway converts to its required type
let payment: types::Payment<types::CreditCard> = input.try_into()?;
gateway.charge(payment, /* ... */).await?;
```

## Alternatives Considered

### Only owned types
Client constructs validated types directly.

**Rejection:** No universal interface for compatible gateways. Owned data
expands attack surface. Premature allocation wastes memory on invalid inputs.

### Runtime validation in adapters
Accept raw strings, validate in each adapter.

**Rejection:** Duplicated validation. No type safety or compile-time
compatibility checks.

## Consequences

### Pros
- Universal code for compatible gateways
- Compile-time prevention of incompatible usage
- Clear API: inputs at root, types in namespace
- Compatible conversion: inputs designed for multiple type variants
- Memory efficient: allocate only after validation
- Reduced attack surface: no ownership until validation
- Type safe: invalid states unrepresentable after conversion

### Cons
- Two parallel hierarchies: inputs and types
- Conversion boilerplate: TryFrom for each compatible pair
- Learning curve: when to use inputs vs types

## Impact on Previous ADRs

### ADR-0005 (Domain Type Safety)
Extends: inputs receive unprotected data, types enforce security tiers.

### ADR-0013 (Flow Variations)
Enables: clients provide inputs, gateways convert to associated types.
