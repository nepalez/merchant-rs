# [ADR-0012]: Payment Source Runtime Validation

## Context

Payment gateways support different payment methods with fundamentally different authorization flows. A single gateway often handles multiple payment types: Stripe processes cards (authorize→capture), bank accounts (verification→debit), and instant transfers (redirect→confirm). The type system must support multiple payment methods per gateway while ensuring type safety for payment-specific data.

Key observations from industry analysis:
- ActiveMerchant, Stripe SDK, Adyen, Spreedly, Klarna all use runtime method selection
- Payment method availability depends on runtime factors (amount, currency, geography, customer eligibility)
- Gateways commonly support 5–15+ payment methods through a single API endpoint
- No production payment library uses compile-time enforcement of gateway-method compatibility

## Problem

Should the system enforce gateway-payment method compatibility at compile time (via the type system) or runtime (via validation)?

## Decision

Use marker trait hierarchy with compile-time source classification instead of runtime validation.

**Marker trait architecture:**

```rust
// Base trait - all payment sources implement this
trait PaymentSource {}

// Flow-specific marker traits
trait InternalPaymentSource: PaymentSource {}  // Cards, tokens - synchronous flows
trait ExternalPaymentSource: PaymentSource {}  // Vouchers, BNPL, redirects - async flows
trait TokenizablePaymentSource: PaymentSource {} // Sources that can be tokenized
```

**Flow trait design with associated types:**

Each payment flow trait uses associated type with trait bound to restrict compatible sources:

```rust
trait ImmediatePayments {
    type Source: InternalPaymentSource;
    async fn charge(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>;
}

trait ExternalPayments {
    type Source: ExternalPaymentSource;
    async fn initiate(&self, source: Self::Source) -> Result<ExternalPayment, Error>;
}
```

**Gateway implementation:**

Gateway declares supported source type via associated type (not runtime list):

```rust
impl ImmediatePayments for StripeGateway {
    type Source = CreditCard;  // Only credit cards for immediate flow
    // ...
}
```

**Rationale:**

- **Compile-time safety:** Invalid source-flow combinations prevented at compile time, not runtime
- **No validation overhead:** Type system enforces constraints, no runtime checks needed
- **Explicit contracts:** Associated type makes supported source visible in trait signature
- **Flexible composition:** Payment sources can implement multiple marker traits (e.g., CreditCard is both InternalPaymentSource and TokenizablePaymentSource)

This aligns with Rust's philosophy of zero-cost abstractions and compile-time guarantees.

## Alternatives Considered

### Runtime validation with unified PaymentSource enum
Create unified enum with all payment source variants, validate gateway support at runtime via `supported_sources()` method.

**Rationale:** Type safety for payment data, runtime flexibility for gateway capabilities.

**Rejection:**
- Runtime errors instead of compile-time prevention
- Validation overhead on every request
- No benefit over marker traits which provide same safety at compile time
- Industry practice (ActiveMerchant, Stripe SDK) shows runtime works for dynamic languages, but Rust offers superior compile-time alternative
- Cannot leverage Rust's zero-cost abstraction philosophy

### Compile-time enforcement via trait bounds
Generic authorize method with trait bound requiring gateway to prove support: `authorize<S: PaymentSource>() where Self: Supports<S>`.

**Rationale:** Compile-time guarantee of compatibility.

**Rejection:** Marker trait explosion (one per payment method). Complex trait bounds obscure API. Still cannot handle runtime availability factors (method disabled for specific transaction amounts/currencies). Adds significant API complexity without solving the actual problem.

### Separate Gateway per payment method
One Gateway implementation per supported payment method.

**Rationale:** Perfect separation of concerns.

**Rejection:** NopCommerce (.NET) attempted this; developers explicitly request a single plugin supporting multiple methods. Operationally burdensome (separate configurations, credentials, monitoring per method). Industry consensus: unify payment methods under a single gateway integration.

## Consequences

### Pros
- Compile-time prevention of invalid source-flow combinations
- Zero runtime validation overhead
- Associated types make gateway capabilities explicit in trait signature
- Payment sources can belong to multiple categories via multiple trait impls
- Type safety at every level: flow traits, source types, marker traits
- Clear compilation errors instead of runtime failures

### Cons
- More complex trait hierarchy to understand
- Cannot handle runtime availability factors (method disabled for specific amounts/currencies)
- Adding new source category requires new marker trait
- Less flexible than runtime validation for dynamic gateway capabilities

