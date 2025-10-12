# [ADR-0007]: Debug Masking Strategy for Payment Data Types

## Context

The payment processing system handles data with varying sensitivity levels per PCI DSS, GDPR, and CCPA. We needed a consistent Debug masking strategy that:
* Prevents PII/financial data exposure in logs
* Maintains debugging utility
* Prevents length disclosure through mask variations
* Provides clear rules for classifying future types

## Decision

Implement a five-tier masking strategy based on data sensitivity:

**Tier 1 - Complete Fixed Masking:** Fixed string regardless of length. Applied to CVV (SAD), AccountNumber, RoutingNumber. Prevents all information disclosure.

**Tier 2 - PCI DSS Partial Display:** First 6 + fixed mask (8 chars) + last 4. Applied to PrimaryAccountNumber, PaymentToken. Follows PCI DSS Requirement 3.3 permitted display.

**Tier 3 - First + Last Character:** `{FIRST_UPPER}***{LAST_UPPER}`. Applied to CardHolderName, CustomerId, AuthorizationCode. Only viable pattern for types with min length 1 - prevents length disclosure through mask collision.

**Tier 4 - Length Only:** `[N chars]`. Applied to ReasonForRefund. Free-text may contain arbitrary PII.

**Tier 5 - No Masking:** Standard Debug derive. Applied to transaction IDs, MerchantReferenceId, BankName, CardExpiry. Operational necessity or public data.

Classification decision tree:
1. SAD per PCI DSS 3.2.2? → Tier 1
2. PAN or PAN-like token? → Tier 2
3. Direct PII or enables correlation? → Tier 3
4. Free-text with unpredictable PII? → Tier 4
5. Operational ID or public data? → Tier 5

## Alternatives Considered

**Alternative 1: Uniform Complete Masking**
* Rationale: Maximum security, simplest policy
* Rejection: Renders debugging impossible for operational IDs, no payment system does this

**Alternative 2: First N + Variable Mask + Last M for All**
* Rationale: More context than single chars, industry standard for PANs
* Rejection: Discloses length when N+M ≥ length (e.g., "test" with 4+4 fully exposed), not suitable for min length 1

**Alternative 3: Hash-Based Masking**
* Rationale: Correlation without disclosure
* Rejection: Over-engineering, still allows correlation attacks, no industry precedent

**Alternative 4: Environment-Dependent Masking**
* Rationale: Full visibility in dev, masked in prod
* Rejection: Security shouldn't depend on environment, debug builds leak to prod logs

**Alternative 5: Feature Flag Configuration**
* Rationale: User-configurable masking level
* Rejection: Security shouldn't be optional, creates ecosystem fragmentation

## Consequences

### Pros
* Regulatory compliance with the strictest GDPR/CCPA/PCI DSS interpretations
* Tier 3 prevents length disclosure via mask collision ("Y", "Yury" both → "Y***Y")
* Tier 5 maintains operational debugging capability
* Clear classification rules for future types
* Industry-aligned (Tier 5 matches Stripe/PayPal/Braintree practice)

### Cons
* Custom Debug implementation overhead
* Reduced debugging context for masked types
* Must remember tier assignment for new types (which is mitigated by relatively stable API)

### Action
* Apply tier classification to all new types before implementation
* Document tier choice in type's Security section
* Add comment above Debug impl explaining masking rationale
* Code review checklist must verify correct tier assignment
* Tests must verify masked output format per tier
