# [ADR-0008]: Bank Routing Number Design

## Context

Bank routing numbers identify financial institutions for electronic fund transfers, but formats vary significantly by country and banking system. Examples include ABA routing numbers (9 digits, US), SWIFT/BIC codes (8-11 characters, international), IFSC codes (11 characters, India), BSB codes (6 digits, Australia), sort codes (6 digits, UK), and numerous others. Each format has different validation rules, length requirements, and structural patterns.

## Problem

Should `RoutingNumber` type in core enforce country-specific validation rules for different routing number formats, or provide a universal container with validation delegated to gateway adapters?

## Decision

Provide universal `RoutingNumber` type in payments/bank/ without country-specific validation.

`RoutingNumber` is a newtype wrapper (per [ADR-0005]) accepting any alphanumeric string within reasonable length bounds (6-20 characters). Country-specific validation (ABA check digit algorithms, SWIFT format structure, IFSC patterns) is delegated to gateway adapters, which validate routing numbers according to their target banking networks and regional requirements.

This aligns with the modular architecture from the [ADR-0001]: core provides payment source types as data containers, adapters implement network-specific business rules and validation. A universal type allows the same `PaymentSource::BankAccount` to work across different banking systems worldwide without requiring core modifications.

## Consequences

### Pros
- Single type works across all banking systems and countries
- No country-specific logic or validation rules in core
- Gateway adapters validate routing numbers for their specific target networks
- Easy to add support for new countries or banking systems without core changes
- Core remains focused on payment processing abstractions, not international banking regulations
- Application can work with any routing number format through uniform interface

### Cons
- Core cannot catch invalid routing numbers at type construction time
- Structural validation is minimal (length only, no format checks)
- Application layer responsible for country-appropriate validation if needed for UX
- Less compile-time type safety compared to country-specific types
- No hints at type level about which routing number format is expected

## Alternatives Considered

### Country-specific types
Separate types like `ABARoutingNumber`, `SwiftCode`, `IFSCCode`, `BSBCode`, etc. Rejected because it fragments the API (different PaymentSource variants needed per country), requires core changes to add support for new countries, forces applications to handle multiple types for conceptually equivalent data, and creates complexity in systems handling multiple countries.

### Enum with country variants
RoutingNumber enum with variants for each country format (US(ABARoutingNumber), International(SwiftCode), India(IFSCCode), etc.). Rejected because it requires core to know all possible routing number formats worldwide, creates tight coupling between core and international banking systems, still requires core updates for new countries, and makes pattern matching complex.

### Validation in core with country parameter
`RoutingNumber::try_new(value, country)` performing country-specific validation based on country code. Rejected because it moves banking business logic into core, requires maintaining validation rules for all countries in core, creates dependencies on country-specific banking knowledge, and needs regular updates as banking systems change.

### Multiple validation traits
Providing traits like `ValidateABA`, `ValidateSWIFT` that adapters can use. Rejected because it doesn't solve the fundamental problem of where validation logic lives, still requires someone to maintain country-specific rules, and adds API complexity without architectural benefit.