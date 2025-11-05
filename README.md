# merchant-rs

Type-Safe Payment Abstraction for Rust

A foundational library for building reliable, unified payment APIs in Rust. This crate defines the contracts (traits), data structures, and error handling necessary for payment processing across diverse gateways and payment methods.

This crate contains zero network logic, ensuring it is lightweight and highly focused on type safety and architectural consistency. All gateway-specific implementations are handled by separate adapter crates.

## Key Features

**Composable Flow Traits**: Defines specialized traits for different payment flows (`ImmediatePayments`, `DeferredPayments`, `ExternalPayments`, `StoreCredentials`, `ThreeDSecure`), allowing gateways to implement only the flows they support.

**Async-First Design**: All gateway operations use `#[async_trait]`, ensuring non-blocking network calls and high throughput essential for modern Rust web services.

**Financial Precision**: Strictly enforces type safety using `rust_decimal::Decimal` and `iso_currency::Currency` for all monetary amounts, eliminating floating-point errors.

**Type-Safe Payment Methods**: Provides comprehensive payment method types (`CreditCard`, `StoredCard`, `BNPL`, `CashVoucher`, `InstantAccount`, `SEPA`, `BankPayment`, `CryptoPayment`, `DirectCarrierBilling`) with marker traits to control which methods work with which flows.

**Data Security**: Implements graduated protection for sensitive data (PCI DSS compliant). All sensitive types (PAN, CVV, account numbers) are wrapped in secure newtypes with memory zeroization on drop and masked debug output. Sealed traits prevent accidental exposure while maintaining compile-time safety guarantees.

**Structured Errors**: Provides a unified `Error` type, allowing downstream applications to handle errors from any provider consistently.

## Payment Flows

The library organizes payment operations into specialized traits, allowing payment gateways to implement only the flows they support:

### Core Transaction Flows

* **`ImmediatePayments`** — One-step payments where authorization and capture occur together (charge). Ideal for digital goods, low-value transactions, and payment methods that don't support separate capture.

* **`DeferredPayments`** — Two-step payments with separate authorization and capture. Used for physical goods (authorize at checkout, capture at shipment), split shipments, and risk review workflows.

* **`ExternalPayments`** — Asynchronous payment flows requiring external completion (redirects, vouchers, QR codes, bank transfers). Returns transaction data with payment instructions for the customer.

### Supporting Flows

* **`StoreCredentials`** — Store payment credentials in gateway vault and retrieve tokens for recurring payments and stored payment methods. Supports removing stored credentials.

* **`ThreeDSecure`** — Manage 3DS authentication flows for card payments requiring Strong Customer Authentication (SCA).

* **`AdjustPayments`** — Modify authorized amounts before capture (increase/decrease reservations).

* **`CancelPayments`** — Cancel authorized payments or pending transactions.

* **`RefundPayments`** — Return funds to customers for captured/settled transactions.

* **`CheckTransaction`** — Query transaction status for async payment flows.

* **`RecoverTransactions`** — Retrieve historical transaction records for reconciliation and reporting.

## Core Data Structures

### Transaction Types

* **`Payment<Method>`** — Payment request with raw payment method, amount, idempotence key, and merchant-initiated transaction type.

* **`Transaction`** — Standardized transaction response including gateway-assigned ID, status, amount, idempotence key, and merchant-initiated type.

* **`TransactionStatus`** — Canonical transaction states (authorized, captured, failed, pending, voided, refunded).

### Payment Methods

* **`CreditCard`** — Credit/debit card with PAN, CVV, expiry, and cardholder name (supports all major card schemes).

* **`StoredCard`** — Tokenized card credentials for recurring/merchant-initiated transactions.

* **`BNPL`** — Buy Now Pay Later services (Klarna, Afterpay, etc.).

* **`CashVoucher`** — Cash payment vouchers (Boleto, OXXO, etc.).

* **`InstantAccount`** — Real-time bank transfers (iDEAL, Sofort, PIX, etc.).

* **`SEPA`** — SEPA Direct Debit for European payments.

* **`BankPayment`** — Generic bank account payments (ACH, BACS, etc.).

* **`CryptoPayment`** — Cryptocurrency payments with wallet addresses.

* **`DirectCarrierBilling`** — Mobile carrier billing payments.

### Financial Types

* **`Money`** — Monetary value with `Decimal` amount and `Currency` code (ISO 4217).

* **`TransactionId`** — Unique gateway-assigned transaction identifier.

* **`TransactionIdempotenceKey`** — Client-provided key for duplicate detection.

### Secure Types

All sensitive data types implement automatic memory zeroization and masked debug output:

* **`PrimaryAccountNumber`** — Card number (PAN) with Luhn validation.
* **`CVV`** — Card verification value (never stored after authorization).
* **`AccountNumber`** — Bank account number.
* **`IBAN`** — International Bank Account Number with validation.
* **`RoutingNumber`** — Bank routing/sort codes.

## Usage Example

This demonstrates how a downstream application uses the flow traits to remain gateway-agnostic:

```rust
use merchant_rs::flows::ImmediatePayments;
use merchant_rs::types::{CreditCard, Payment, Transaction, Money};
use merchant_rs::Error;
use std::sync::Arc;

// In a typical web service, you would inject the specific implementation
// wrapped in a dynamic trait object.
async fn process_payment<G>(
    gateway: Arc<G>,
    payment: Payment<G::Method>
) -> Result<Transaction, Error>
where
    G: ImmediatePayments + Send + Sync,
{
    // The trait method is called using .await
    gateway.charge(payment).await
}
```

## Related Crates

To gain full functionality, combine this crate with gateway adapter crates:

* `merchant-rs-{adapter}` — Concrete gateway implementations (in development)
* `merchant-rs-testing` — Mock gateway for unit testing (in development)
