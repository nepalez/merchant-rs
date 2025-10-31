# [ADR-0003]: Gateway Trait Segregation

## Context

Payment gateways have varying capabilities. Some support two-step authorization with delayed capture (`Stripe`, `Braintree`), others only immediate settlement (many crypto processors). Some allow refunds, others do not. Some support tokenization, others require client-side token creation.

## Problem

The architectural question: Should gateway adapters implement a monolithic interface with all possible operations, or segregated traits based on actual capabilities?

## Decision

Apply Interface Segregation Principle: segregate traits by capability and payment flow type.

**Base trait (mandatory for all gateways):**
- `CheckTransaction` - retrieve transaction status by ID (minimal requirement for any adapter)

**Payment flow traits (optional, based on gateway capabilities):**

*Synchronous flows:*
- `ImmediatePayments` - one-step charge (authorization and capture in single call)
  - Associated type: `Source: InternalPaymentMethod`
  - Method: `async fn charge(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>`
- `DeferredPayments` - two-step flow (separate authorize and capture operations)
  - Associated type: `Source: InternalPaymentMethod`
  - Methods: `async fn authorize(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>`
  - `async fn capture(&self, transaction_id: TransactionId, amount: Option<Decimal>) -> Result<Transaction, Error>`

*Asynchronous flows:*
- `ExternalPayments` - initiate payment with external completion (redirects, webhooks, vouchers)
  - Associated type: `Source: ExternalPaymentMethod`
  - Methods: `async fn initiate(&self, source: Self::Source) -> Result<ExternalPayment, Error>`
  - `async fn payment_data(&self, transaction_id: TransactionId) -> Result<ExternalPaymentData, Error>`

**Transaction management traits:**
- `RefundPayments` - return funds to customer
- `CancelPayments` - void/cancel authorization or recent transaction
- `AdjustPayments` - modify transaction amount or details

**Advanced capabilities:**
- `RecoverTransactions` - search transactions by idempotence key (disaster recovery)
- `TokenizePaymentSources` - create tokens from payment data
- `ThreeDSecure` - 3D Secure authentication flows

Gateway adapters implement only the traits matching their actual capabilities. Each trait uses associated types with marker trait bounds to restrict compatible payment methods at compile time.

## Consequences

### Pros
- Adapters implement only supported flows and capabilities
- Clear separation: immediate vs deferred vs external payment flows
- Each trait focused on single responsibility (ISP compliance)
- CheckTransaction as minimal base contract reduces adapter complexity
- Compile-time safety through associated types with marker trait bounds
- Flow-specific traits make payment completion semantics explicit
- Associated types prevent invalid payment method-flow combinations at compile time

### Cons
- Client code must check trait bounds at compile time or handle missing capabilities at runtime
- More traits to understand compared to a single monolithic interface
- Associated type syntax adds complexity to trait definitions

## Alternatives Considered

### Monolithic Gateway trait
Single trait with all operations (`authorize`, `capture`, `refund`, `tokenize`, etc.). Rejected because it forces adapters to implement unsupported operations, typically returning "not supported" errors at runtime rather than preventing misuse at compile time. Violates Interface Segregation Principle and prevents clean extension architecture.