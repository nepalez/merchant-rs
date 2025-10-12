# [ADR-0006]: Data Security and Handling of Sensitive Financial Primitives

## Context

The `merchant-rs-core` processes various financial data primitives, including **Primary Account Numbers (PANs)**, **Card Verification Values (CVVs)**, and bank identifiers like **Account Numbers** and **Routing Numbers**. These fields have different security classifications under external regulatory standards (e.g., PCI DSS, GLBA) but all require strong, enforced protection to prevent accidental leakage in logs or memory.

The development goal is to establish a **Defense-in-Depth** security architecture by using the Rust type system to enforce data handling safety. We need a strategy that guarantees:
1.  Memory Zeroization for highly sensitive data upon drop.
2.  Guaranteed Log Masking for all sensitive fields.
3.  Explicit Consent from the developer (via `unsafe`) to access raw secret data.
4.  Controlled Cloning to allow for transactional resilience (retries) without compromising security.

## Decision

We will use the `secrecy::SecretBox<String>` type as the fundamental wrapper for all financial primitives classified as **Sensitive Authentication Data (SAD)** or **Critical Personally Identifiable Information (PII)**.

Specifically, the following data types will be encapsulated within `SecretBox<String>`:

* PrimaryAccountNumber (SAD/PII)
* CVV (SAD)
* AccountNumber (Critical PII)
* RoutingNumber (Critical PII)

Data types considered **Non-Critical Financial Data (NCFD)** will use simple value wrappers (e.g., `CardExpiry` or `String` wrapper) and are deemed safe from the requirements of `SecretBox`. This includes the **Card Expiration Date (`CardExpiry`)** because, per PCI DSS, it is **not** Sensitive Authentication Data (SAD) and cannot be used alone to initiate a fraudulent transaction. The complexity introduced by `SecretBox` is therefore not justified for NCFD fields.

For fields like **`CardHolderName`** that are **Sensitive Non-critical Data (SND)** (i.e., PII that requires log masking but not memory zeroization or `unsafe` access control), we will use a **simple `String` wrapper** type that implements a **custom `Debug` trait for masking**. This provides log protection without the burden of `SecretBox`.

## Alternatives Considered

### 1. Simple String Wrappers with Manual Logging
* Pros: Simplest code structure, no need for `unsafe` or closures.
* Cons: Relies entirely on developer discipline for logging and memory handling. High risk of accidental leakage in debug logs or uncaught errors. Does not provide memory zeroization. Rejected due to high risk profile.

### 2. Using `SecretBox<String>` only for SAD (PAN, CVV)
* Pros: Reduces code complexity by limiting `unsafe` closures to two types.
* Cons: Exposes bank identifiers (`AccountNumber`, `RoutingNumber`) to logging risk, weakening the overall Defense-in-Depth strategy for critical PII. Rejected because consistent protection for all financial identifiers is preferred.

### 3. Using `SecretBox<String>` for CardExpiry
* Pros: Achieves the highest possible level of protection for all card-related data points.
* Cons: **Unjustified complexity**. `CardExpiry` is NCFD and does not require memory zeroization or access control via `unsafe`. The implementation complexity (closures) outweighs the marginal security gain. Rejected because it violates the principle of using appropriate security measures for the data classification.

### 4. Using `SecretBox<String>` for CardHolderName (SND)
* Pros: Guaranteed log masking and memory protection for the name.
* Cons: **Unjustified overhead**. `CardHolderName` is not required to be zeroized by any standard and does not pose a direct financial access risk like a PAN. A custom `Debug` implementation on a standard `String` wrapper achieves the necessary **log masking** without requiring the use of `unsafe` closures for routine access, which is disproportionate for this type of data. Rejected in favor of custom `Debug` wrapper.

## Consequences

### Pros

* **Guaranteed Security**: Memory zeroization on drop for all critical secrets is guaranteed by `SecretBox`.
* **Prevented Leakage**: Log masking is enforced by the type system for all PII and SAD, virtually eliminating accidental sensitive data exposure in diagnostics.
* **Explicit Risk Acknowledgment**: The use of `unsafe` methods for access forces consuming code (the gateway adapter) to explicitly manage the data lifecycle.
* **Controlled Resilience**: Safe `Clone` implementation allows request objects to be copied for transactional robustness (retries) without creating unsecured raw string copies.

### Cons

* **Code Complexity**: All code consuming the raw values of `PAN`, `CVV`, `AccountNumber`, or `RoutingNumber` must use the `unsafe fn with_exposed_secret` closure pattern, increasing development complexity.
* **Increased Number of Primitives**: Requires maintaining two types of secure wrappers: `SecretBox` wrappers and custom `Debug` wrappers (for SND), slightly increasing the number of types to manage.

### Action

Implement a generic abstraction (e.g., a trait or a custom macro) to reduce boilerplate code associated with implementing `Debug`, `Clone`, and `TryFrom<String>` for the `SecretBox<String>` primitives. Also, define a pattern (e.g., a macro or trait) for creating **Sensitive Non-critical Data (SND)** wrappers to simplify `Debug` masking for types like `CardHolderName`.