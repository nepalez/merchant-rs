# [ADR-0004]: Adoption of Newtype Wrappers for Financial Primitives

## Context

The `merchant-rs-core` API currently uses primitive types (`String`, `u8`, `u16`) for sensitive and semantically critical financial data (e.g., Primary Account Number, CVV, Customer ID). This practice introduces several risks:

1. Risk of Type Confusion (Typo-squatting): When multiple `String` arguments are used in a function signature, they can be accidentally swapped without triggering a compiler error.
2. Lack of Centralized Validation: Validation logic for critical invariants (e.g., CVV length, Expiry Month range of 1-12) must be duplicated across multiple processing functions.

To achieve the project's goal of ensuring **type safety and financial accuracy**, this practice must be corrected.

## Decision

All semantically critical financial primitives and identifiers will be replaced with dedicated **Newtype Wrapper structs** (e.g., `pub struct Cvv(String)`, `pub struct ExpiryMonth(u8)`).

* Implementation: These wrappers will implement necessary traits (`Debug`, `Clone`, `Serialize`, `Deserialize`).
* Validation: Structs representing bounded values (e.g., `ExpiryMonth`) will implement a custom `new()` constructor that returns a `Result<Self, Error>`, enforcing invariants upon instantiation.
* Scope: This change will be applied to all core data structures, including `CardDetails`, `TokenizationRequest`, and all transaction request/response bodies.

## Alternatives Considered

### Keep Using Primitives (`String`, `u8`)

* Rejection: Fails to meet the project's foundational safety and maintenance requirements. It accepts the risks of type confusion and decentralized validation logic.

### Use Rust Type Aliases (`type Cvv = String`)

* Rejection: Type aliases offer zero type-safety. They are purely a convenience tool for naming and do not prevent accidental swapping of semantically distinct but structurally identical types.

## Consequences

### Pros

* Compile-Time Safety: The compiler will prevent the accidental swapping of semantically distinct arguments.
* Centralized Validation: Validation logic is encapsulated within the wrapper's constructor.
* Improved API Clarity: The API contract becomes self-documenting.

### Cons

* Increased Boilerplate: Requires defining many small wrapper structs.
* Minor Friction: Developers must use the inner type accessor or conversion methods to interact with external APIs.

## Action: Code Refactoring

The core development team must refactor all structs in `crates/core/src/types.rs` and related trait modules to utilize the new wrapper types. Validation logic must be implemented in the constructors of the new primitive types (e.g., `ExpiryMonth::new`).