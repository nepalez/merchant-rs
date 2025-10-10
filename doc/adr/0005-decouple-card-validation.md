# ADR-0005: Decouple CardNumber BIN/Length Validation from Core

## Context

The `CardNumber` type must guarantee two critical invariants: Luhn Check (syntactic correctness) and Structural Validity (adherence to BIN/Length standards). The primary architectural goal for the `core` crate is maximal minimalism (zero-cost, minimal dependencies, no infrastructure dependencies like `regex`).

We identified a conflict: comprehensive Structural Validity requires maintaining an extensive, frequently updated list of BIN ranges, and using a third-party crate like `card-validate` for this purpose would introduce undesirable, heavy dependencies (`regex`, `lazy_static`) into the `core` crate. BIN ranges are constantly changing (monthly/quarterly), which contradicts the goal of making `core` logic stable.

## Decision

We will **decouple** the comprehensive, up-to-date BIN/Length check from the `core` domain type's mandatory invariants.

The `CardNumber` type in the `core` crate will enforce the following, relying on a single lightweight dependency (`luhn3`): Mandatory Luhn Check (using `luhn3`), Secret Storage (`secrecy::SecretBox`), Sanitization (digits only), and a trivial, non-updated Structural Check (basic length and prefix check for major types like Visa, MC, Amex) as a final, low-fidelity safety net.

The **full, comprehensive, and current BIN/Length validation** is relegated to the **Application or Service Layer logic**, which must execute *before* constructing the `CardNumber` type. This calling layer is permitted to use heavy dependencies or external lookup services.

## Alternatives Considered

1.  **Use `card-validate` in `core`:** Rejected. Introduces heavy, unwanted dependencies (`regex`, `lazy_static`), violating the core principle of minimalism.
2.  **Remove all structural checks from `core`:** Rejected. Weakens the domain invariant too much. A valid Luhn number that is 50 digits long would be accepted, which is poor data hygiene and contradicts payment industry best practices.
3.  **Maintain `CARD_RULES` manually in `core`:** Rejected. Requires frequent, manual updates to the core domain code, which contradicts the goal of stability and infrequent change.

## Consequences

### Pros

The `core` crate remains lean, fast, and free of heavy dependencies. The primary safety invariant (Luhn Check) is correctly delegated to a battle-tested, minimal crate (`luhn3`). The domain type enforces data secrecy and basic structural sanity.

### Cons

The `CardNumber` type, on its own, does not guarantee the number corresponds to the latest known BIN ranges. The responsibility for up-to-date BIN validation is explicitly shifted to the consuming layer (Application/Service).

### Action

The documentation for `CardNumber::new` or `try_from` must be updated to clearly state that **full BIN validation is a pre-condition** that must be met by the calling layer before attempting to instantiate the `CardNumber` type.
```
