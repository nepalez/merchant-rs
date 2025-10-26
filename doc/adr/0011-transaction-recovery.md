# [ADR-0011]: Transaction Recovery Mechanisms

## Context

Payment processing requires reliable transaction status retrieval for reconciliation, webhook verification, customer support, and audit compliance. Gateway adapters need mechanisms to recover the transaction state in two distinct scenarios:

1. **Normal operation**: Application has `transaction_id` from authorization response
2. **Failure recovery**: Application lost `transaction_id` due to database rollback, network failure after partial commit, or system crash before persistence

## Problem

Should transaction retrieval by ID be part of the core `Authorizable` contract, or treated as an optional capability? How should recovery from lost transaction IDs be handled when `merchant_reference_id` is the only available identifier?

## Decision

Implement two-tier recovery architecture: mandatory retrieval by transaction ID and optional recovery by merchant reference.

**Add mandatory method to `Authorizable` trait:**

Transaction retrieval by ID is a fundamental operation supported by all payment gateways (transaction_id serves as a primary key in gateway databases). This method enables:
- Webhook verification (validate notification authenticity)
- Reconciliation (match against gateway statements)
- Customer support (investigate disputes)
- Audit compliance (maintain transaction trail)

**Create optional `Recoverable` trait:**

Search by `merchant_reference_id` is a fallback mechanism for disaster recovery when `transaction_id` was lost before persistence. Not all gateways support this capability:
- **Support search**: Stripe, Braintree, PayPal
- **No search API**: Adyen, Authorize.Net, crypto processors, voucher systems

The `Recoverable` trait follows Interface Segregation Principle from ADR-0003: adapters implement only when gateway provides search functionality.

## Consequences

### Pros
- Transaction retrieval by ID universally available (all gateways support)
- Clear separation: `Authorizable` for normal flow, `Recoverable` for disaster recovery
- Type-safe distinction between a single result (by ID) and potential multiple results (by search)
- Aligns with gateway capabilities (search is genuinely optional)

### Cons
- `Authorizable` contract expands beyond transaction initiation
- Applications must handle two retrieval patterns depending on the available identifier
- Search API inconsistency across gateways (different filter support)

## Impact on Previous ADRs

### ADR-0003 (Gateway Trait Segregation)
**Change**: Expands mandatory `Authorizable` contract with `get_transaction()` method.

**Rationale**: Does not violate Interface Segregation Principle because transaction retrieval by ID is universally supported. The principle applies to optional capabilities; this is universal. The `Recoverable` trait for search functionality maintains the segregation pattern.

### ADR-0004 (Transaction Initiation Method)
**Clarification**: `authorize()` remains the canonical transaction initiation method `get_transaction()` is retrieval, not initiation. The ADR title and scope remain accurate.

## Alternatives Considered

### Single unified recovery trait
Combining both retrieval by ID and search by `merchant_reference_id` in one trait.

**Rejection**: Violates Interface Segregation Principle by forcing all adapters to implement search functionality that many gateways don't provide. Transaction retrieval by ID is universal and deserves presence in the core contract.

### Separate RetrievableById trait
Making even transaction ID lookup optional via a dedicated trait.

**Rejection**: Transaction retrieval by ID is not optional—all gateways support it as the primary key lookup. Creating a trait for universal functionality adds unnecessary complexity without architectural benefit.
