# [ADR-0007]: Decouple CardNumber BIN Validation

## Context

Credit card numbers contain a Bank Identification Number (`BIN`) in the first 6–8 digits that identifies the issuing institution and card network. BIN validation can determine card type (Visa, Mastercard, etc.), country of issuance, and validity against known issuer ranges.

## Problem

Should `PrimaryAccountNumber` type in core include BIN validation logic, or should BIN validation be delegated to gateway adapters and application layer?

## Decision

Decouple BIN validation from core `PrimaryAccountNumber` type.

The `PrimaryAccountNumber` type in types/ performs only structural validation: length bounds (13–19 digits) and Luhn algorithm checksum. BIN-based validation (card network detection, issuer identification, country checks) is delegated to gateway adapters and application layer.

Core validation ensures data integrity: the PAN is structurally valid and passes the Luhn check. Business logic validation (BIN ranges, network rules) belongs in adapters where it can be tailored to gateway-specific requirements.

This follows the principle from the [ADR-0001]: core contains fundamental payment processing contracts and data types, not business logic or external data dependencies. BIN validation requires external databases that change frequently and business rules that vary by gateway, region, and use case.

## Consequences

### Pros
- Core remains lightweight without external data dependencies
- No need to maintain BIN databases or update them as ranges change
- Gateway adapters implement BIN validation according to their specific requirements
- Different gateways can use different BIN validation strategies (some may not validate at all)
- Application layer can implement BIN validation for UX purposes (card type detection) independently
- Core validation (structure + Luhn) is sufficient for data integrity guarantees

### Cons
- Application layer must implement BIN validation separately if needed for UX (card type icons, formatting)
- No centralized BIN database shared across adapters
- Potential duplication of BIN validation logic if multiple adapters need it
- Cannot provide card network hints at type level

## Alternatives Considered

### BIN validation in core
Including BIN databases and validation logic in `PrimaryAccountNumber` type. Rejected because it creates external data dependencies in core (BIN ranges must be updated regularly), imposes business logic that may not apply to all gateways, increases core complexity and maintenance burden, and couples core to specific card network rules.

### Separate BIN validation crate
Creating merchant-rs-bin-validator as a shared optional library. Rejected because BIN validation requirements vary significantly by use case (payment routing decisions vs. UX card type detection vs. fraud prevention), and a one-size-fits-all solution would not serve any use case optimally. Adapters and applications can create their own BIN validators as needed.

### BIN validation as a trait in the core
Providing optional BIN validation trait that types can implement. Rejected because it still requires maintaining BIN data somewhere, and a trait-based approach doesn't solve the fundamental problem of where BIN databases live and who maintains them.
