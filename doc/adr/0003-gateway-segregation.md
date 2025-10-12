# [ADR-0003]: Segregation of Identity/Capability (Gateway) and Transactional Logic (Authorizable)

## Context

The design of the `merchant-rs-core` API must maintain maximum modularity and strictly adhere to the **Interface Segregation Principle (ISP)**.

In our system, two distinct concerns are managed:
1.  Gateway Identity and Capabilities: The minimal contract required to identify the adapter (`fn id()`) and declare its features via **Associated Types** (e.g., `TransactionFlow`).
2.  Core Transactional Logic: Methods executing the primary financial operations (`authorize`, `void`).

An independent functionality, such as **Tokenization (`Tokenizable`)**, requires access only to the adapter's **Identity and Capabilities**. Merging these contracts would force independent traits to depend on methods they do not use, directly violating the ISP.

## Decision

The core contracts shall be explicitly segregated into two distinct traits:

* `Gateway`: This trait will define the **minimal, foundational contract**. Its sole responsibility is to define the adapter's **Identity** and the **Manifest of Capabilities** via all associated types.
    * *Role:* Serves as the dependency for all traits requiring only identification or capability checks (e.g., `Tokenizable`).
* `Authorizable`: This trait will define the **core transactional functional methods** (`authorize`, `void`).
    * *Dependency:* It must inherit the `Gateway` trait as its supertrait (`pub trait Authorizable: Gateway`).
    * *Role:* Serves as the dependency for all traits extending transactional capabilities (e.g., `Capturable`, `Refundable`).

This creates a clear hierarchy where `Gateway` is the root of all dependencies.

## Alternatives Considered

### Merge `Gateway` and `Authorizable` into one monolithic trait

* *Rejection:* Directly **violates ISP**. It would force independent traits like `Tokenizable` to depend on and implement the full transaction lifecycle methods (`authorize`, `void`), which are unnecessary for tokenization functionality.

### Use Default Methods for transactional logic in `Gateway`

* *Rejection:* This violates the **"Thin Core" principle** and shifts capability checking from desirable compile-time verification to a runtime error check, diminishing the value of Rust's type system.

### Define `Authorizable` methods with complex generic constraints to disable them for certain types

* *Rejection:* Significantly reduces API readability and maintainability. Trait segregation is the more idiomatic and simpler pattern in Rust for this specific architectural problem.

## Consequences

### Pros

* **ISP Compliance:** Ensured, as traits like `Tokenizable` depend only on the minimal necessary contract (`Gateway`).
* **Clean Hierarchy:** Establishes a logical dependency flow: **Identity/Capabilities** $\rightarrow$ **Core Transaction** $\rightarrow$ **Extensions**.
* **Correct Modeling:** Correctly models independent operations (like tokenization) as peer operations to authorization.
* **Type Safety Integration:** Seamlessly integrates with the **Associated Type Dispatch (ATD)** mechanism, as `Gateway` is the single source of truth for all capability checks.

### Cons

* **Minimal Boilerplate Increase:** Transactional adapters must explicitly implement both `Gateway` and `Authorizable`, a minor cost for significant architectural safety.

## Action: Enforced Dependency Rule

The core development team must adhere to the rule that any new transactional traits (e.g., `SubscriptionManagement`) are declared as supertraits of `Authorizable`, and any new independent utility traits (e.g., `StatusCheckable`) are declared as supertraits of `Gateway`.
