# [ADR-0004]: Canonical Transaction Initiation Method

## Context

Payment processing terminology varies across the industry. Different systems use `authorize`, `purchase`, `sale`, or `charge` to describe transaction initiation. Some distinguish between authorization-only and immediate capture, others use a single term for both.

## Problem

The architectural question: What should be the canonical method name for initiating a payment transaction in merchant-rs?

## Decision

Use authorize() as the single transaction initiation method for all payment flows.

**Rationale:**
The term "authorize" is semantically neutral and correctly describes both authorization-only (two-step) and authorization-with-immediate-capture (one-step) flows. All payment transactions begin with authorization - the distinction is whether capture happens immediately or later.

The method accepts polymorphic PaymentSource (per ADR-0001), working uniformly across cards, bank transfers, wallets, BNPL, and cryptocurrency.

**Industry alignment:**
Major payment libraries use the authorize concept as their primary transaction method (Stripe: PaymentIntent authorization, Braintree: Transaction.authorize, Adyen: payments API authorization flow).

## Consequences

### Pros
- Single method name for all transaction types reduces API surface
- Semantically accurate for both one-step and two-step flows
- Works uniformly with all PaymentSource variants
- Aligns with industry terminology

### Cons
- Developers accustomed to purchase/sale terminology must learn authorize semantics
- May initially seem verbose for simple immediate-capture use cases

## Alternatives Considered

### purchase() or sale()
Using purchase/sale as the method name. Rejected because these terms imply immediate capture, creating semantic confusion for two-step authorization flows. A gateway supporting only auth-capture would have a purchase() method that doesn't actually purchase, which is misleading.

### charge()
Using charge as the method name. Rejected because "charge" is ambiguous about whether it means authorization or capture, and is less common in formal payment processing terminology compared to authorize.

### Separate authorize() and purchase() methods
Providing both methods where purchase() performs auth+capture. Rejected because it creates API redundancy and forces clients to choose between methods based on internal gateway implementation details rather than business intent.