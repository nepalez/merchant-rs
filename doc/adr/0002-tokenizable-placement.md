# [ADR-0002]: Token Usage vs Token Creation Separation

## Context

Payment tokens serve two distinct purposes in payment processing: using stored payment methods for subsequent transactions, and creating new tokens from sensitive payment data.

## Problem

The architectural question is whether both concerns belong in core or should be separated.

Key observations:
- Recurring payments and PCI compliance require using pre-existing tokens as a payment source
- Token creation can happen via server-side APIs, client-side SDKs, or external vault providers
- Not all payment gateways support server-side tokenization APIs
- Production systems must never store raw card data after initial processing

## Decision

Separate token usage (core) from token creation (extension).

**Core includes `PaymentToken` as a `PaymentSource` variant:**
`PaymentSource::Token(PaymentToken)` is a fundamental payment source type because using stored payment methods is essential to production payment flows. This enables recurring payments, subscription billing, and PCI-compliant payment processing where backends work exclusively with tokens after initial capture.

**Extension includes `Tokenizable` trait:**
The `Tokenizable` trait for creating tokens from raw payment data belongs in merchant-rs-vault extension because:
- Token creation is an optional gateway capability, not universally supported
- Many gateways require client-side tokenization (`Stripe.js`, `Square Web SDK`, `Braintree Drop-in`)
- External specialized vault providers exist (`Spreedly`, `TokenEx`)
- Gateway adapters can accept tokens without providing tokenization

This mirrors the `CustomerId` pattern: `CustomerId` exists in core (used in requests), but customer creation is in the vault extension.

**Gateway implementation flexibility:**
A gateway adapter can implement only `Authorizable` (accepts tokens, does not create them), or both `Authorizable` and `Tokenizable` (full server-side token lifecycle). Client applications can obtain tokens through any mechanism and use them uniformly via `PaymentSource::Token`.

## Consequences

### Pros
- Core remains minimal while supporting the most critical use case (using tokens)
- Gateway adapters not providing server-side tokenization have no unused dependencies
- Client applications can obtain tokens from any source (gateway API, client SDK, external vault) and use them uniformly
- Clear separation of concerns: payment execution vs token management
- PCI compliance supported at core level without forcing tokenization implementation

### Cons
- Split between `PaymentToken` (core) and `Tokenizable` (extension) may initially seem inconsistent
- Documentation must clearly explain when to use core vs vault extension

## Alternatives Considered

### Tokenizable in core
Considered including `Tokenizable` trait in core since `PaymentToken` is in core. Rejected because tokenization is not universally supported by gateways and many production systems use client-side or external tokenization, making it an optional capability rather than a core requirement.

### PaymentToken in extension
Considered moving `PaymentToken` to vault extension alongside `Tokenizable`. Rejected because using tokens as a payment source is fundamental to production payment processing (recurring payments, PCI compliance). Every production system must support `PaymentSource::Token` regardless of how tokens are created.

### Combined TokenVault trait
Considered a single trait combining token usage and creation. Rejected because it forces gateway adapters to implement token creation even when only accepting pre-existing tokens, and prevents using tokens from external sources.