# [ADR-0013]: Flow Variations Support via Sealed Traits

## Context

Payment gateways implement similar functionality through incompatible interfaces.

**Payment models:**
- Simple payments: gateway accepts payment without split distribution
- Split payments: gateway accepts payment with distribution to multiple recipients

**Installment payments:**
- Not supported: gateway doesn't support installments
- Basic: gateway supports fixed/stored plans
- Regional: gateway supports region-specific features (Brazil fee, India offer_id, Japan revolving/bonus, GCC shariah compliance)

**Authorization changes:**
- Not supported: gateway doesn't allow changing authorization amounts
- Total-based (Stripe, Adyen): gateway accepts new complete authorized amount
- Delta-based (Checkout.com, Worldpay): gateway accepts increment/decrement amounts

**Capture variations:**
- Full capture only: gateway captures entire authorized amount without modifications
- Partial amount: gateway accepts specific amount to capture
- Redistributed recipients: gateway allows changing recipient distribution during capture

**Refund variations:**
- Full refund only: gateway refunds entire transaction amount without modifications
- Partial amount: gateway accepts specific amount to refund
- Redistributed recipients: gateway allows changing recipient distribution during refund

**Problem:** If a single trait method accepts parameters that some gateways don't support:
- Client code cannot be deterministic
- Gateway adapters must convert between incompatible models
- Type system doesn't signal required changes when switching gateways

## Decision

Use **sealed traits with associated types** to enforce compile-time constraints. Three patterns depending on variation type and scope.

### Pattern 1: Gateway-Level Type Constraint

Private trait constrains allowed types in the root `Gateway` trait. Flow traits access these types via associated type projections.

```rust
// In types/payments.rs
pub(crate) trait PaymentMarker {
    type PaymentMethod: PaymentMethod;
}

impl<P: PaymentMethod> PaymentMarker for Payment<P> {
    type PaymentMethod = P;
}

impl<P: PaymentMethod> PaymentMarker for SplitPayment<P> {
    type PaymentMethod = P;
}

// In types/installments.rs
pub(crate) trait InstallmentsMarker {}

impl InstallmentsMarker for NoInstallments {}
impl InstallmentsMarker for Installments {}
impl InstallmentsMarker for InstallmentsBR {}
impl InstallmentsMarker for InstallmentsIN {}
impl InstallmentsMarker for InstallmentsJP {}
impl InstallmentsMarker for InstallmentsGCC {}

// In lib.rs
#[allow(private_bounds)]
pub trait Gateway: Send + Sync {
    type Payment: PaymentMarker;
    type Installments: InstallmentsMarker;
}

// In flow traits
pub trait ImmediatePayments: Gateway {
    async fn charge(
        &self,
        payment: <Self as Gateway>::Payment,
        installments: <Self as Gateway>::Installments,
        ...
    ) -> Result<Transaction, Error>;
}
```

Gateway chooses concrete types once:
- `type Payment = Payment<P>` or `SplitPayment<P>` where P is the payment method
- `type Installments = NoInstallments`, `Installments`, `InstallmentsBR`, `InstallmentsIN`, `InstallmentsJP`, or `InstallmentsGCC`

All flow traits reuse these declarations through associated type projections.

**Type organization:** Alternative types for gateway-level variations are organized in dedicated modules:
- `src/types/payments/` contains `Payment`, `SplitPayment` and their marker trait `PaymentMarker`
- `src/types/installments/` contains `NoInstallments`, `Installments`, regional variants, and marker trait `InstallmentsMarker`
- `src/types/payment_methods/` contains all payment method types and marker traits

**Used in:**
- **Payment**: ImmediatePayments, DeferredPayments, RecurrentPayments, ExternalPayments, ThreeDSecure
- **Installments**: ImmediatePayments, DeferredPayments, RecurrentPayments

### Pattern 2: Flow-Level Type Constraint

Private trait constrains allowed types in a specific flow trait, not in Gateway. Used when variation applies to a single flow.

```rust
// In flows/deferred_payments.rs
trait CapturedAmount {}
impl CapturedAmount for CaptureAuthorized {}  // Full capture
impl CapturedAmount for Option<Decimal> {}    // Partial capture

trait CapturedDistribution {}
impl CapturedDistribution for CaptureAuthorized {}    // Keep original
impl CapturedDistribution for Option<Recipients> {}  // Redistribute

pub trait DeferredPayments: Gateway {
    type CapturedAmount: CapturedAmount;
    type CapturedDistribution: CapturedDistribution;

    async fn capture(
        &self,
        transaction_id: TransactionId,
        captured_amount: Self::CapturedAmount,
        captured_distribution: Self::CapturedDistribution,
    ) -> Result<Transaction, Error>;
}
```

Gateway chooses capture semantics:
- `CapturedAmount = CaptureAuthorized` (full) or `Option<Decimal>` (partial)
- `CapturedDistribution = CaptureAuthorized` (keep original) or `Option<Recipients>` (redistribute)

Similarly for refunds:
```rust
// In flows/refund_payments.rs
trait RefundAmount {}
impl RefundAmount for TotalRefund {}         // Full refund
impl RefundAmount for Option<Decimal> {}     // Partial refund

trait RefundDistribution {}
impl RefundDistribution for TotalRefund {}          // Keep original
impl RefundDistribution for Option<Recipients> {}   // Redistribute

pub trait RefundPayments: Gateway {
    type RefundAmount: RefundAmount;
    type RefundDistribution: RefundDistribution;

    async fn refund(
        &self,
        transaction_id: TransactionId,
        refund_amount: Self::RefundAmount,
        refund_distribution: Self::RefundDistribution,
    ) -> Result<Transaction, Error>;
}
```

Gateway chooses refund semantics:
- `RefundAmount = TotalRefund` (full) or `Option<Decimal>` (partial)
- `RefundDistribution = TotalRefund` (keep original) or `Option<Recipients>` (redistribute)

**Type organization:** Marker types for flow-level variations are defined alongside their flow traits, not in separate modules. They represent operation-specific semantics rather than reusable type families.

**Used in:**
- **DeferredPayments**: CapturedAmount, CapturedDistribution (defined in `flows/deferred_payments.rs`)
- **RefundPayments**: RefundAmount, RefundDistribution (defined in `flows/refund_payments.rs`)

### Pattern 3: Marker Types for Mutual Exclusion

Marker structs determine which additional traits can be implemented, enforcing mutual exclusion.

```rust
// In flows/change_authorization.rs
pub(crate) trait Sealed {}

pub struct ChangesNotSupported;
pub struct ChangesByTotal;
pub struct ChangesByDelta;

impl Sealed for ChangesNotSupported {}
impl Sealed for ChangesByTotal {}
impl Sealed for ChangesByDelta {}

// In flows/deferred_payments.rs
pub trait DeferredPayments: Gateway {
    type AuthorizationChanges: change_authorization::Sealed;
}

// Variation traits require specific marker
pub trait EditAuthorization: DeferredPayments<AuthorizationChanges = ChangesByTotal> {
    async fn edit_authorization(...) -> Result<...>;
}

pub trait AdjustAuthorization: DeferredPayments<AuthorizationChanges = ChangesByDelta> {
    async fn adjust_authorization(...) -> Result<...>;
}
```

Gateway can only set `AuthorizationChanges` to one value → attempting to implement both `EditAuthorization` and `AdjustAuthorization` produces compilation error.

**Type organization:** Marker types for mutual exclusion are defined in dedicated flow modules where they establish the constraint. They serve as compile-time switches between incompatible operation semantics.

**Used in:** change_authorization (ChangesNotSupported, ChangesByTotal, ChangesByDelta - defined in `flows/change_authorization.rs`)

### Pattern Selection Guide

**Use Pattern 1 (Gateway-Level)** when:
- Variation applies across multiple flow traits
- Gateway capability is fundamental (payment models, installments)
- All flows need consistent semantics
- Types form a reusable family with multiple alternatives
- **Organization:** Create `src/types/{feature}/` module with alternative types and marker trait

**Use Pattern 2 (Flow-Level)** when:
- Variation applies to a single flow trait
- Multiple independent choices within the flow
- Gateway can vary behavior per method
- Types represent operation-specific semantics, not reusable abstractions
- **Organization:** Define sealed traits directly in the flow module alongside the trait

**Use Pattern 3 (Mutual Exclusion)** when:
- Gateway must choose between incompatible approaches
- Implementing both variations would be contradictory
- Need compile-time enforcement of "either A or B, not both"
- Additional traits depend on specific marker value
- **Organization:** Define marker types in dedicated flow module, use in associated type constraints

### Type Organization Principles

1. **Gateway-Level Types (Pattern 1)**: Organized in `src/types/{feature}/` subdirectories
   - Multiple alternative types with shared semantics
   - Marker trait defining the type family
   - Used across multiple flows
   - Examples: `payments/`, `installments/`, `payment_methods/`

2. **Flow-Level Types (Pattern 2)**: Co-located with flow trait definition
   - Operation-specific sealed traits
   - Marker types defined in same file or `src/types/` (e.g., `CaptureAuthorized`, `TotalRefund`)
   - Not extracted to separate modules unless they form a reusable family

3. **Mutual Exclusion Types (Pattern 3)**: Defined in flow module
   - Pure marker structs with no data
   - Enable/disable additional trait implementations
   - Live where they enforce the constraint

### Design Principles

1. **Interface Determinism**: Gateway implementing a trait MUST fully support ALL methods. No partial implementations, no runtime errors for "unsupported parameter"

2. **Compile-Time Constraints**: The type system prevents using incompatible gateways. Client code declares capabilities via trait bounds

3. **Explicit Migration**: Switching gateways that change capabilities produces compilation errors, forcing explicit code updates

4. **Centralized vs Localized**: Gateway-level types (Pattern 1) centralize decisions with reusable type families, flow-level types (Pattern 2) localize operation-specific semantics

## Alternatives Rejected

### Separate optional parameters
`fn charge(total_amount: Decimal, recipients: Option<Recipients>, installments: Option<Installments>)`

**Rejected:** No compile-time guarantee that gateway supports features. Client code may pass options to gateway that doesn't support them. Runtime validation required.

### Single trait with enum parameter
Parameter like `Installments::None | Basic(u8) | Brazil { count: u8, fee: Decimal }`.

**Rejected:** Gateway may not support all variants → runtime errors, no compile-time constraints for client code, adapter must validate and convert.

### Feature flags
Using Cargo features to conditionally compile support: `features = ["splits", "installments"]`.

**Rejected:** Features are additive and transitive. Dependencies can enable features unexpectedly. No per-gateway granularity (one crate may need multiple gateway types).

### Runtime capability checking
Methods like `supports_installments()`, `supports_splits()` + both implementation methods.

**Rejected:** Runtime checks instead of compile-time safety, gateway must implement unsupported methods (return errors), cannot constrain generic functions.

### Generic method parameters
`fn charge<I: Into<Installments>>(installments: I)`

**Rejected:** Loss of object safety, cannot use trait objects. Generic pollution across entire API.

## Consequences

**Pros:**
- Type safety: impossible to call unsupported operations at compile time
- Migration safety: capability changes produce compilation errors
- Zero runtime overhead: no validation of parameter combinations
- Clear contracts: trait signature describes exactly what's supported

**Cons:**
- More associated types: gateway must declare multiple type parameters
- Boilerplate: sealed trait definitions for each variation point
- Learning curve: developers must understand pattern selection and usage
- Verbose syntax: `<Self as Gateway>::Installments` instead of simple type

**Implementation Status:**

| Feature | Pattern | Marker Trait Location | Type Definitions | Variants |
|---------|---------|----------------------|------------------|----------|
| Payment | Gateway-Level (1) | `types/payments.rs` | `types/payments/{payment,split_payment}.rs` | `Payment<P>`, `SplitPayment<P>` |
| Installments | Gateway-Level (1) | `types/installments.rs` | `types/installments/*.rs` | `NoInstallments`, `Installments`, `InstallmentsBR`, `InstallmentsIN`, `InstallmentsJP`, `InstallmentsGCC` |
| PaymentMethod | Gateway-Level (1) | `types/payment_methods.rs` | `types/payment_methods/*.rs` | `CreditCard`, `BankPayment`, `BNPL`, `CashVoucher`, `CryptoPayment`, `DirectCarrierBilling`, `InstantAccount`, `SEPA`, `StoredCard`, `Vault` |
| CapturedAmount | Flow-Level (2) | `flows/deferred_payments.rs` | `types/charge_authorized.rs` | `CaptureAuthorized`, `Option<Decimal>` |
| CapturedDistribution | Flow-Level (2) | `flows/deferred_payments.rs` | `types/charge_authorized.rs`, stdlib | `CaptureAuthorized`, `Option<Recipients>` |
| RefundAmount | Flow-Level (2) | `flows/refund_payments.rs` | `types/total_refund.rs` | `TotalRefund`, `Option<Decimal>` |
| RefundDistribution | Flow-Level (2) | `flows/refund_payments.rs` | `types/total_refund.rs`, stdlib | `TotalRefund`, `Option<Recipients>` |
| AuthorizationChanges | Mutual Exclusion (3) | `flows/change_authorization.rs` | `flows/change_authorization.rs` | `ChangesNotSupported`, `ChangesByTotal`, `ChangesByDelta` |
