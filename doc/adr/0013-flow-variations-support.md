# [ADR-0013]: Flow Variations Support via Separate Traits

## Context

Payment gateways implement similar functionality through incompatible interfaces. Example: authorization amount changes.

**Total-based** (Stripe, Adyen, Braintree, PayPal, Square): gateway accepts new complete authorized amount
**Delta-based** (Checkout.com, Worldpay, Authorize.Net): gateway accepts increment/decrement amounts

Both achieve the same business goal (change authorization before capture) but require different parameters and semantics.

**Problem:** If a single trait method accepts parameters that some gateways don't support:
- Client code cannot be deterministic
- Gateway adapters must convert between incompatible models
- Type system doesn't signal required changes when switching gateways

## Decision

Use **separate traits for each variation**, enforced by sealed marker types at compile time.

### Architecture

**Multilayer hierarchy:**

1. **Gateway level**: chooses which flows to support
   - Example: implements `DeferredPayments`, `ImmediatePayments`, `ExternalPayments`

2. **Flow level**: declares variations through associated types
   - Example: `DeferredPayments` has `AuthorizationChanges` variation
   - Each flow can have independent variations

3. **Variation level**: constrains which related traits can be implemented
   - Example: `EditAuthorization`, `AdjustAuthorization`

Authorization changes are scoped to `DeferredPayments` flow only (not applicable to immediate or external payments).

### Mechanism

Sealed pattern with marker types prevents incompatible implementations:

- Module `change_authorization` defines sealed trait and marker types:
  - `pub(crate) Sealed` trait
  - `ChangesNotSupported`, `ChangesByTotal`, `ChangesByDelta` (pub structs implementing the `Sealed`)

- `DeferredPayments` declares: `type AuthorizationChanges: change_authorization::Sealed`

- Variation traits require a specific marker:
  - `EditAuthorization: DeferredPayments<AuthorizationChanges = ChangesByTotal>`
  - `AdjustAuthorization: DeferredPayments<AuthorizationChanges = ChangesByDelta>`

Gateway can only set `AuthorizationChanges` to one value → attempting to implement both `EditAuthorization` and `AdjustAuthorization` produces compilation error.

### Design Principles

1. **Interface Determinism**: Gateway implementing a trait MUST fully support ALL methods. No partial implementations, no runtime errors for "unsupported parameter"

2. **Compile-Time Constraints**: The type system prevents using incompatible gateways. Client code declares capabilities via trait bounds

3. **Explicit Migration**: Switching gateways that change capabilities produces compilation errors, forcing explicit code updates

## Alternatives Rejected

### Single trait with enum parameter
Parameter like `ChangeAmount::Total(Decimal) | Delta(Decimal)` in single method.

**Rejected:** Gateway may not support both variants → runtime errors, no compile-time constraints for client code, adapter must convert models.

### Optional parameters with runtime validation
Single method with `new_total: Option<Decimal>`, `increment: Option<Decimal>`, `decrement: Option<Decimal>`.

**Rejected:** Unclear which combinations are valid, runtime validation overhead, type system provides no safety.

### Runtime capability checking
Methods like `supports_total_changes()`, `supports_delta_changes()` + both implementation methods.

**Rejected:** Runtime checks instead of compile-time safety, gateway must implement unsupported methods (return errors), cannot constrain generic functions.

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

**Status:** Implemented in `src/flows/change_authorization.rs` with `EditAuthorization` and `AdjustAuthorization` traits.
