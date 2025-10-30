# [ADR-0004]: Canonical Transaction Initiation Method

## Context

Payment processing terminology varies across the industry. Different systems use `authorize`, `purchase`, `sale`, or `charge` to describe transaction initiation. Some distinguish between authorization-only and immediate capture, others use a single term for both.

## Problem

The architectural question: What should be the canonical method name for initiating a payment transaction in merchant-rs?

## Decision

Use flow-specific transaction initiation methods instead of a single universal method.

**Payment flow separation:**

The library distinguishes three payment flow patterns based on completion mechanism:

1. **Immediate (one-step):** `ImmediatePayments::charge()` - authorization and capture in single operation
2. **Deferred (two-step):** `DeferredPayments::authorize()` - authorization only, followed by separate `capture()`
3. **External (async):** `ExternalPayments::initiate()` - payment initiated, completed outside flow via redirect/webhook

**Rationale:**

Different payment methods have fundamentally different authorization patterns:
- Card payments: can be immediate or deferred
- Bank transfers: typically immediate
- Vouchers, redirects, BNPL: external with async completion

Using distinct methods for distinct flows:
- Makes payment completion semantics explicit at call site
- Enables type-safe source restrictions via associated types
- Avoids runtime branching on payment source type
- Aligns method semantics with actual gateway behavior

**Industry alignment:**
Payment gateways themselves use different endpoints for different flow types (Stripe: charges vs payment_intents, Adyen: authorise vs payments endpoints).

## Consequences

### Pros
- Payment flow type explicit from method name (charge vs authorize vs initiate)
- Each method has clear, unambiguous completion semantics
- Compile-time prevention of invalid source-flow combinations via associated types
- No runtime type discrimination needed
- Aligns with gateway API design patterns

### Cons
- Three methods instead of one increases API surface
- Developers must understand flow type distinctions
- Method selection requires knowledge of payment source characteristics

## Alternatives Considered

### purchase() or sale()
Using purchase/sale as the method name. Rejected because these terms imply immediate capture, creating semantic confusion for two-step authorization flows. A gateway supporting only auth-capture would have the `purchase()` method that doesn't purchase, which is misleading.

### charge()
Using charge as the method name. Rejected because "charge" is ambiguous about whether it means authorization or capture, and is less common in formal payment processing terminology compared to authorize.

### Single authorize() method for all flows
Using universal `authorize()` accepting any payment source. Rejected because:
- Obscures fundamental behavioral differences between flows (immediate vs deferred vs external)
- Requires runtime type discrimination to route to appropriate gateway endpoint
- Single method would have unclear completion semantics (when is payment actually charged?)
- Forces unified PaymentSource enum preventing compile-time source validation
- Does not align with how gateways themselves organize their APIs