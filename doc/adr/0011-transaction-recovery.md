# [ADR-0011]: Transaction Recovery Mechanisms

## Context

Payment processing requires reliable transaction status retrieval for reconciliation, webhook verification, customer support, and audit compliance. Gateway adapters need mechanisms to recover the transaction state in two distinct scenarios:

1. **Normal operation**: Application has `transaction_id` from authorization response
2. **Failure recovery**: Application lost `transaction_id` due to database rollback, network failure after partial commit, or system crash before persistence

## Problem

Should transaction retrieval by ID be part of the core `Authorizable` contract, or treated as an optional capability? How should recovery from lost transaction IDs be handled when `merchant_reference_id` is the only available identifier?

## Decision

Implement two-tier recovery architecture: base transaction lookup and optional search capability.

**CheckTransaction trait (mandatory for all gateways):**

Transaction retrieval by ID is the base contract for all gateway adapters:

```rust
trait CheckTransaction {
    async fn status(&self, transaction_id: TransactionId) -> Result<Transaction, Error>;
}
```

This is the minimal requirement for any adapter and enables:
- Webhook verification (validate notification authenticity)
- Reconciliation (match against gateway statements)
- Customer support (investigate disputes)
- Audit compliance (maintain transaction trail)

**RecoverTransactions trait (optional):**

Search by `TransactionIdempotenceKey` for disaster recovery when `transaction_id` was lost before persistence:

```rust
trait RecoverTransactions {
    type Iterator: TransactionIterator;
    async fn transactions(&self, key: TransactionIdempotenceKey) -> Self::Iterator;
}
```

Returns async iterator (not single result) because:
- Idempotence key may match multiple transactions (retries, duplicates)
- Results require pagination for large result sets
- Network retries may create duplicate transactions

Not all gateways support search capability:
- **Support search**: Stripe, Braintree, PayPal
- **No search API**: Adyen, Authorize.Net, crypto processors, voucher systems

The `RecoverTransactions` trait follows Interface Segregation Principle from ADR-0003: adapters implement only when gateway provides search functionality.

## Consequences

### Pros
- Transaction retrieval by ID universally available (all gateways support)
- CheckTransaction as minimal base contract (no payment flow dependencies)
- Clear separation: CheckTransaction for normal lookup, RecoverTransactions for disaster recovery
- Type-safe distinction: single result (by ID) vs async iterator (by search)
- Aligns with gateway capabilities (search is genuinely optional)
- Iterator pattern handles pagination and large result sets efficiently

### Cons
- CheckTransaction separate from payment flow traits (requires additional trait bound)
- Applications must handle two retrieval patterns depending on available identifier
- Search API inconsistency across gateways (different filter support)
- Iterator pattern more complex than simple Result return

## Impact on Previous ADRs

### ADR-0003 (Gateway Trait Segregation)
**Aligns**: CheckTransaction becomes the base mandatory trait (minimal contract for all adapters). RecoverTransactions follows segregation pattern as optional capability. Does not violate Interface Segregation Principle because transaction retrieval by ID is universally supported, while search is genuinely optional.

### ADR-0004 (Transaction Initiation Methods)
**Independent**: CheckTransaction is orthogonal to payment flow initiation methods (charge, authorize, initiate). Transaction lookup does not affect initiation method design.

## Alternatives Considered

### Single unified recovery trait
Combining both retrieval by ID and search by idempotence key in one trait.

**Rejection**: Violates Interface Segregation Principle by forcing all adapters to implement search functionality that many gateways don't provide. Transaction retrieval by ID is universal and deserves separate base trait.

### Making CheckTransaction optional
Including transaction ID lookup as optional capability via trait.

**Rejection**: Transaction retrieval by ID is not optional—all gateways support it as primary key lookup. This is the minimal requirement for any adapter, making it the appropriate base trait.

### Search by merchant_reference_id instead of idempotence key
Using merchant reference as search key instead of idempotence key.

**Rejection**: Idempotence key is the natural search key for disaster recovery scenarios. Merchant reference may not be unique across retries. Idempotence key specifically designed for detecting duplicate transactions.
