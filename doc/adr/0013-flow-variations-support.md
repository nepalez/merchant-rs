# [ADR-0013]: Flow Variations Support via Sealed Traits

## Context

Payment gateways implement similar functionality through incompatible interfaces.

**Split payments:**
- Simple amount: gateway accepts `Decimal` only
- Distributed amount: gateway accepts `DistributedAmount` (total + recipients)

**Authorization changes:**
- Total-based (Stripe, Adyen): gateway accepts new complete authorized amount
- Delta-based (Checkout.com, Worldpay): gateway accepts increment/decrement amounts

**Problem:** If a single trait method accepts parameters that some gateways don't support:
- Client code cannot be deterministic
- Gateway adapters must convert between incompatible models
- Type system doesn't signal required changes when switching gateways

## Decision

Use **sealed traits with associated types** to enforce compile-time constraints. Two patterns depending on variation type.

### Pattern 1: Type Constraint

Private trait constrains allowed parameter types in method signatures.

```rust
trait Amount {}
impl Amount for Decimal {}
impl Amount for DistributedAmount {}

#[allow(private_bounds)]
pub trait ImmediatePayments: Gateway {
    type Amount: Amount;
    async fn charge(&self, amount: Self::Amount, ...) -> Result<Transaction, Error>;
}
```

Gateway chooses concrete type: `type Amount = Decimal` (no splits) or `type Amount = DistributedAmount` (with splits).

**Used in:** ImmediatePayments, DeferredPayments, RecurrentPayments, ExternalPayments, RefundPayments, ThreeDSecure.

### Pattern 2: Marker Types

Marker structs determine which additional traits can be implemented.

- Module `change_authorization` defines sealed trait and marker types:
  - `pub(crate) Sealed` trait
  - `ChangesNotSupported`, `ChangesByTotal`, `ChangesByDelta` (pub structs implementing the `Sealed`)

- `DeferredPayments` declares: `type AuthorizationChanges: change_authorization::Sealed`

- Variation traits require a specific marker:
  - `EditAuthorization: DeferredPayments<AuthorizationChanges = ChangesByTotal>`
  - `AdjustAuthorization: DeferredPayments<AuthorizationChanges = ChangesByDelta>`

Gateway can only set `AuthorizationChanges` to one value → attempting to implement both `EditAuthorization` and `AdjustAuthorization` produces compilation error.

**Used in:** change_authorization (AuthorizationChanges).

### Design Principles

1. **Interface Determinism**: Gateway implementing a trait MUST fully support ALL methods. No partial implementations, no runtime errors for "unsupported parameter"

2. **Compile-Time Constraints**: The type system prevents using incompatible gateways. Client code declares capabilities via trait bounds

3. **Explicit Migration**: Switching gateways that change capabilities produces compilation errors, forcing explicit code updates

## Alternatives Rejected

### Separate parameters for split payments
`fn charge(total_amount: Decimal, recipients: Option<Recipients>)`

**Rejected:** No compile-time guarantee that gateway supports recipients. Client code may pass recipients to gateway that doesn't support splits.

### Single trait with enum parameter
Parameter like `ChangeAmount::Total(Decimal) | Delta(Decimal)` in single method.

**Rejected:** Gateway may not support both variants → runtime errors, no compile-time constraints for client code, adapter must convert models.

### Optional parameters with runtime validation
Single method with `new_total: Option<Decimal>`, `increment: Option<Decimal>`, `decrement: Option<Decimal>`.

**Rejected:** Unclear which combinations are valid, runtime validation overhead, type system provides no safety.

### Runtime capability checking
Methods like `supports_total_changes()`, `supports_delta_changes()` + both implementation methods.

**Rejected:** Runtime checks instead of compile-time safety, gateway must implement unsupported methods (return errors), cannot constrain generic functions.

### Generic method parameters
`fn charge<A: Into<DistributedAmount>>(amount: A)`

**Rejected:** Loss of object safety, cannot use trait objects.

## Consequences

**Pros:**
- Type safety: impossible to call unsupported operations at compile time
- Migration safety: capability changes produce compilation errors
- Zero runtime overhead: no validation of parameter combinations
- Clear contracts: trait signature describes exactly what's supported

**Cons:**
- More traits: similar functionality requires multiple definitions
- Code duplication: adapters may have similar implementation code
- Learning curve: developers must understand which variation their gateway implements
- Explicit conversions: client must call `.into()` when passing parameters

**Status:**
- Pattern 1 (Type Constraint): all flow traits (ImmediatePayments, DeferredPayments, RecurrentPayments, ExternalPayments, RefundPayments, ThreeDSecure)
- Pattern 2 (Marker Types): `src/flows/change_authorization.rs` with `EditAuthorization` and `AdjustAuthorization` traits
