# [ADR-0002]: Canonical Transaction Initiation Method (Authorize vs. Purchase)

## Context

The `merchant-rs-core` project defines the canonical interface (`AuthorizeCapture` trait) for initiating financial transactions. The primary goal is to support the full spectrum of payment flows, from traditional card processing to modern e-wallets, micropayment systems, and crypto wallets.

This mandates support for two distinct transaction models:

1.  **Two-Step Flow (Auth/Capture):** Funds are reserved (`authorize`) on the customer's payment source and later confirmed (`capture`). This is the industry standard for card-based transactions where goods are shipped later.
 
2. **One-Step Flow (Sale/Purchase):** Funds are immediately and atomically debited in a single operation. This is common for digital goods, subscriptions, ACH/SEPA transfers, and many e-wallet integrations.

The challenge is to select a single, sufficiently abstract, and industry-idiomatic method name for transaction initiation that is used across the entire system, while maintaining a minimal and clean canonical core interface ("Thin Core" principle).

## Decision

The method for initiating a financial transaction within the `AuthorizeCapture` trait shall be named `authorize`.

```rust
#[async_trait]
pub trait AuthorizeCapture: Gateway {
    /// Initiates a financial transaction.
    /// This operation may result in a simple Authorization (reserved funds)
    /// or an immediate Sale/Purchase (captured funds), depending on the
    /// payment gateway's nature and configuration.
    async fn authorize(
        &self,
        request: AuthorizationRequest,
    ) -> Result<AuthorizationResponse>;
    // ... other methods (capture, void)
}
```

For all **One-Step (Sale/Purchase) flows**, the specific Gateway Adapter must implement the `authorize` method by executing the full `Sale` transaction on the underlying gateway and returning an **`AuthorizationResponse`** with the transaction status explicitly set to **`TransactionStatus::Captured`**.

## **Alternatives Considered**

1.  **Use `purchase` instead of `authorize`:**
    * *Rationale:* `purchase` semantically aligns better with a one-step debit (`Sale`).
    * *Rejection:* The term `purchase` is semantically inaccurate for the two-step flow, where the intent is to **reserve** (authorize) funds. Using `authorize` as the base reflects the most common and critical protocol (card reservation). (Reference: Industry peers like **ActiveMerchant** and **Omnipay** often use both `authorize` and `purchase`, highlighting the semantic gap).
2.  **Use a more generic term like `charge` or `execute`:**
    * *Rationale:* These names are highly abstract and cover both reservation and immediate debit. (Reference: Common in some internal Java PSP SDKs).
    * *Rejection:* They are less immediately recognizable or idiomatic in the payments industry, where `Authorize`/`Capture` is the prevailing terminology for high-value card-centric APIs.
3.  **Define both `authorize` and `purchase` methods:**
    * *Rationale:* This provides clear semantic distinction to the client. (Reference: Explicit separation in **Omnipay**).
    * *Rejection:* This unnecessarily complicates the **canonical core interface** (violating the "Thin Core" principle). It forces every Adapter to implement both, even if the underlying gateway only supports one (e.g., a simple micropayment system might only support `purchase`). Our design prefers to handle this flow variation within the Adapter's single `authorize` implementation.

## **Consequences**

### **Pros**

* **Industry Idiom:** The term `authorize` is the established standard in card processing, which remains the core of most systems, providing immediate familiarity for developers.
* **Minimalist Core:** The single `authorize` method maintains a **minimal, focused core interface**, preventing the core from being polluted by variations in payment flow types.
* **Enforced Protocol:** Naming the trait `AuthorizeCapture` and the method `authorize` sets the expectation for the highest-friction, two-step protocol, promoting safer development practices for merchants.
* **Simple Client Logic:** The client layer (`merchant-rs-client`) only needs to call one method (`authorize`) to start any transaction flow.

### **Cons**

* **Semantic Ambiguity:** For one-step (Sale) transactions, the name `authorize` is semantically misleading, as no reservation occurs. This must be managed purely through documentation and the returned `TransactionStatus`.

### **Action: Delegation of Responsibility**

To resolve the semantic ambiguity, the responsibility for mapping the canonical `authorize` call to the correct underlying gateway operation is delegated to the Adapter layer (Thick Adapter principle):

* When an Adapter is configured for a gateway that **only supports Sale** (e.g., some e-wallets or crypto gateways), its implementation of `authorize` must execute the `Sale` operation.
* The Adapter must then return an **`AuthorizationResponse`** with the `status` field explicitly set to **`TransactionStatus::Captured`** (not `Authorized`), which is the definitive signal to the client that the funds have been debited immediately.