# [ADR-0002]: Token Usage vs Token Creation Separation

> **Note:** This ADR describes the separation between token usage and token creation in the context of the initial modular architecture. While [ADR-0010] changed the implementation to a monolithic crate, the core principle remains valid: `PaymentToken` as a payment source is part of the core functionality, while tokenization capabilities would be feature-gated if implemented.

## Context

Payment tokens serve two distinct purposes in payment processing: using stored payment methods for later transactions and creating new tokens from sensitive payment data.

## Problem

The architectural question is whether both concerns belong in the core or should be separated.

Key observations:
- Recurring payments and PCI compliance require using pre-existing tokens as a payment source
- Token creation can happen via server-side APIs, client-side SDKs, or external vault providers
- Not all payment gateways support server-side tokenization APIs
- Production systems must never store raw card data after initial processing

## Decision

Separate token usage (core) from token creation (extension).

**Core includes `PaymentToken` as a `PaymentSource` variant:**
`PaymentSource::Token(PaymentToken)` is a fundamental payment source type because using stored payment methods is essential to production payment flows. This enables recurring payments, subscription billing, and PCI-compliant payment processing where backends work exclusively with tokens after initial capture.

**Core includes `TokenizePaymentSources` trait:**
The `TokenizePaymentSources` trait for creating tokens from raw payment data is part of the core flows module (per [ADR-0010] monolithic architecture) as an optional capability:
- Token creation is an optional gateway capability, not universally supported
- Many gateways require client-side tokenization (`Stripe.js`, `Square Web SDK`, `Braintree Drop-in`)
- External specialized vault providers exist (`Spreedly`, `TokenEx`)
- Gateway adapters can accept tokens without providing tokenization
- Following trait segregation principle ([ADR-0003]), adapters implement only when they support server-side tokenization

**Gateway implementation flexibility:**
A gateway adapter can implement payment flow traits (e.g., `ImmediatePayments`, `DeferredPayments`) to accept tokens as payment sources, without implementing `TokenizePaymentSources`. Or it can implement both flow traits and `TokenizePaymentSources` for full server-side token lifecycle. Client applications can obtain tokens through any mechanism and use them uniformly as payment sources.

## Consequences

### Pros
- Core supports both token usage (as payment source) and token creation (optional trait)
- Gateway adapters not providing server-side tokenization simply don't implement `TokenizePaymentSources`
- Client applications can obtain tokens from any source (gateway API, client SDK, external vault) and use them uniformly
- Clear separation: token usage is fundamental, token creation is optional capability
- PCI compliance supported without forcing tokenization implementation
- Follows trait segregation pattern consistently

### Cons
- Token creation capability in core rather than extension (but aligns with monolithic architecture)
- Documentation must clearly explain when to implement `TokenizePaymentSources` vs accepting tokens only

## Alternatives Considered

### TokenizePaymentSources in extension subcrate
Considered moving tokenization to separate vault extension subcrate. Rejected because [ADR-0010] adopted monolithic architecture, eliminating extension subcrates. Tokenization follows the same optional trait pattern as other capabilities (RefundPayments, RecoverTransactions) within the single crate.

### Token as separate payment source type
Considered having tokens only in extension, not as core payment source. Rejected because using tokens as payment source is fundamental to production payment processing (recurring payments, PCI compliance). Every production system must support tokens as payment sources regardless of how tokens are created.

### Combined TokenVault trait
Considered a single trait combining token usage and creation. Rejected because it forces gateway adapters to implement token creation even when only accepting pre-existing tokens and prevents using tokens from external sources.