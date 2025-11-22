# [ADR-0013]: Flow Variations Support via Sealed Traits

## Context

Payment gateways implement similar functionality through incompatible interfaces.

**Split payments:**
- No splits: gateway accepts base amount only
- With splits: gateway accepts distribution via `Option<Recipients>`

**Installment payments:**
- Not supported: gateway doesn't support installments
- Basic: gateway supports fixed/stored plans
- Regional: gateway supports region-specific features (Brazil fee, India offer_id, Japan revolving/bonus, GCC shariah compliance)

**Authorization changes:**
- Total-based (Stripe, Adyen): gateway accepts new complete authorized amount
- Delta-based (Checkout.com, Worldpay): gateway accepts increment/decrement amounts

**Capture variations:**
- Full capture only: gateway captures entire authorized amount
- Partial capture: gateway accepts specific amount to capture
- Redistributed capture: gateway allows changing recipient distribution during capture

**Problem:** If a single trait method accepts parameters that some gateways don't support:
- Client code cannot be deterministic
- Gateway adapters must convert between incompatible models
- Type system doesn't signal required changes when switching gateways

## Decision

Use **sealed traits with associated types** to enforce compile-time constraints. Two patterns depending on variation type.

### Pattern 1: Gateway-Level Type Constraint

Private trait constrains allowed types in the root `Gateway` trait. Flow traits access these types via associated type projections.

```rust
// In lib.rs
trait DistributionMarker {}
impl DistributionMarker for NoDistribution {}
impl DistributionMarker for Option<Recipients> {}

trait InstallmentsMarker {}
impl InstallmentsMarker for NoInstallments {}
impl InstallmentsMarker for Installments {}
impl InstallmentsMarker for InstallmentsBR {}
// ... other regional variants

#[allow(private_bounds)]
pub trait Gateway: Send + Sync {
    type PaymentMethod: PaymentMethod;
    type AmountDistribution: DistributionMarker;
    type Installments: InstallmentsMarker;
}

// In flow traits
pub trait ImmediatePayments: Gateway {
    async fn charge(
        &self,
        distribution: <Self as Gateway>::AmountDistribution,
        installments: <Self as Gateway>::Installments,
        ...
    ) -> Result<Transaction, Error>;
}
```

Gateway chooses concrete types once:
- `type AmountDistribution = NoDistribution` or `Option<Recipients>`
- `type Installments = NoInstallments`, `Installments`, `InstallmentsBR`, etc.

All flow traits reuse these declarations.

**Used in:**
- **AmountDistribution**: ImmediatePayments, DeferredPayments, RecurrentPayments, ExternalPayments, RefundPayments, ThreeDSecure
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
- `CapturedDistribution = CaptureAuthorized` (keep) or `Option<Recipients>` (redistribute)

**Used in:** DeferredPayments (CapturedAmount, CapturedDistribution)

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

**Used in:** change_authorization (AuthorizationChanges)

### Pattern Selection Guide

**Use Pattern 1 (Gateway-Level)** when:
- Variation applies across multiple flow traits
- Gateway capability is fundamental (split payments, installments)
- All flows need consistent semantics

**Use Pattern 2 (Flow-Level)** when:
- Variation applies to a single flow trait
- Multiple independent choices within the flow
- Gateway can vary behavior per method

**Use Pattern 3 (Mutual Exclusion)** when:
- Gateway must choose between incompatible approaches
- Implementing both variations would be contradictory
- Need compile-time enforcement of "either A or B, not both"

### Design Principles

1. **Interface Determinism**: Gateway implementing a trait MUST fully support ALL methods. No partial implementations, no runtime errors for "unsupported parameter"

2. **Compile-Time Constraints**: The type system prevents using incompatible gateways. Client code declares capabilities via trait bounds

3. **Explicit Migration**: Switching gateways that change capabilities produces compilation errors, forcing explicit code updates

4. **Centralized vs Localized**: Gateway-level types (Pattern 1) centralize decisions, flow-level types (Pattern 2) localize them. Choose based on scope of variation

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

| Feature | Pattern | Location | Variants |
|---------|---------|----------|----------|
| AmountDistribution | Gateway-Level (1) | `lib.rs` | `NoDistribution`, `Option<Recipients>` |
| Installments | Gateway-Level (1) | `lib.rs` | `NoInstallments`, `Installments`, `InstallmentsBR/IN/JP/GCC` |
| CapturedAmount | Flow-Level (2) | `flows/deferred_payments.rs` | `CaptureAuthorized`, `Option<Decimal>` |
| CapturedDistribution | Flow-Level (2) | `flows/deferred_payments.rs` | `CaptureAuthorized`, `Option<Recipients>` |
| AuthorizationChanges | Mutual Exclusion (3) | `flows/change_authorization.rs` | `ChangesNotSupported`, `ChangesByTotal`, `ChangesByDelta` |
