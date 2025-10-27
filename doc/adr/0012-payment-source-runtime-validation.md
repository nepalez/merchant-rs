# [ADR-0012]: Payment Source Runtime Validation

## Context

Payment gateways support different payment methods with fundamentally different authorization flows. A single gateway often handles multiple payment types: Stripe processes cards (authorize→capture), bank accounts (verification→debit), and instant transfers (redirect→confirm). The type system must support multiple payment methods per gateway while ensuring type safety for payment-specific data.

Key observations from industry analysis:
- ActiveMerchant, Stripe SDK, Adyen, Spreedly, Klarna all use runtime method selection
- Payment method availability depends on runtime factors (amount, currency, geography, customer eligibility)
- Gateways commonly support 5-15+ payment methods through a single API endpoint
- No production payment library uses compile-time enforcement of gateway-method compatibility

## Problem

Should the system enforce gateway-payment method compatibility at compile time (via type system) or runtime (via validation)?

## Decision

Use runtime validation with explicit capability declaration.

Gateway adapters declare supported payment methods via `supported_sources()` method returning static list of `PaymentSourceType` discriminants. The `Authorizable` trait validates source compatibility before delegating to adapter implementation. Validation occurs early in the request pipeline with clear error messages.

`PaymentSource` remains a unified enum (per ADR-0001) with strongly-typed variants. Gateway adapters use pattern matching to route to method-specific authorization logic. The separation between declaration (`supported_sources`), validation (`validate_source_support`), and implementation (`authorize_impl`) provides clear contract boundaries.

This aligns with ADR-0008 principle: core provides payment source types as data containers, adapters implement network-specific business rules.

## Alternatives Considered

### Compile-time enforcement via associated types
Gateway declares single supported source via associated type, separate traits per payment method.

**Rationale:** Type safety prevents unsupported method usage.

**Rejection:** Incompatible with reality of multi-method gateways. Stripe supporting cards, ACH, SEPA would require three separate Gateway implementations, fragmenting the API. No industry precedent for this approach.

### Compile-time enforcement via trait bounds
Generic authorize method with trait bound requiring gateway to prove support: `authorize<S: PaymentSource>() where Self: Supports<S>`.

**Rationale:** Compile-time guarantee of compatibility.

**Rejection:** Marker trait explosion (one per payment method). Complex trait bounds obscure API. Still cannot handle runtime availability factors (method disabled for specific transaction amounts/currencies). Adds significant API complexity without solving the actual problem.

### Separate Gateway per payment method
One Gateway implementation per supported payment method.

**Rationale:** Perfect separation of concerns.

**Rejection:** NopCommerce (.NET) attempted this; developers explicitly request single plugin supporting multiple methods. Operationally burdensome (separate configurations, credentials, monitoring per method). Industry consensus: unify payment methods under single gateway integration.

## Consequences

### Pros
- Industry alignment: matches ActiveMerchant, Stripe, Adyen, Spreedly architecture patterns
- Operational simplicity: single gateway handles all supported methods
- Flexibility: method availability can depend on runtime factors
- Clear errors: validation failures provide actionable messages
- Extensibility: adding payment methods requires core change (enum variant) but not adapter interface changes
- Type safety preserved: each PaymentSource variant has strongly-typed data

### Cons
- No compile-time prevention of unsupported method usage
- Validation happens at runtime with error result
- Developer must check gateway documentation or handle validation errors

### Action

Implementation tasks tracked in TODO.md:
1. Add `PaymentSourceType` enum with discriminants for all PaymentSource variants
2. Add `supported_sources()` to Gateway trait
3. Add `validate_source_support()` helper to Gateway trait
4. Add `UnsupportedPaymentSource` error variant
5. Update Authorizable trait to validate before delegating to adapter
6. Update all gateway adapters to declare supported sources
