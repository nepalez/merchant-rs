# [ADR-0005]: Domain Type Safety and Security Strategy

## Context

Payment processing involves numerous domain primitives like account numbers, routing codes, card numbers, transaction identifiers, and customer information. These values have specific validation rules, formatting requirements, and varying levels of sensitivity. Some data (CVV, PAN) is highly regulated and must be protected in memory and logs, while other data (transaction IDs, status codes) is safe to expose.

## Problem

How should domain primitives be represented to ensure type safety, enforce domain invariants, and protect sensitive data in memory and debug output?

## Decision

Adopt newtype pattern with graduated security protection for all domain types.

**Type safety through newtypes:**
Every domain value is wrapped in a dedicated newtype struct. The wrapper enforces invariants through sealed traits in the internal/ module: Validated for domain rules, Sanitized for normalization, and SafeWrapper for construction. Once constructed, a type is guaranteed valid - invalid states are unrepresentable at compile time.

**Security through tiered protection:**
Each type is assigned a security tier determining its protection level:
- Tier 1 (Maximum): Full redaction in all contexts - CVV, passwords
- Tier 2 (High): Partial visibility for correlation - PAN shows first 6 + last 4, tokens show prefix
- Tier 3 (Medium): Masked for privacy - email, phone show partial, addresses redacted
- Tier 4 (Operational): Length-only masking - descriptions, references
- Tier 5 (Public): No masking - transaction IDs, status codes, public identifiers

**Implementation:**
Sensitive types (Tier 1-2) wrap SecretString from internal/secret_string.rs, which uses the secrecy crate for memory zeroization. SecretString provides controlled access through unsafe methods while preventing accidental exposure. All newtypes implement custom Debug respecting their assigned tier, preventing sensitive data leakage in logs and error traces.

## Consequences

### Pros
- Invalid states unrepresentable at compile time
- Validation logic encapsulated within type definitions
- Self-documenting API: function signatures express domain concepts clearly
- Memory safety: sensitive data zeroized on drop
- Safe logging: debug output cannot expose sensitive information
- Consistent protection strategy across entire codebase
- Graduated approach balances security with operational needs
- Type system enforces protection (SecretString API requires explicit exposure)

### Cons
- More boilerplate: each domain concept requires type definition
- Conversions required when interoperating with external systems
- Debugging more difficult with redacted values
- Performance overhead from memory zeroization
- Developers must use unsafe methods to access protected data
- Learning curve: understanding newtype pattern and security tiers

## Alternatives Considered

### Raw primitives with validation functions
Using String or `u64` directly with external validation functions. Rejected because validation becomes the caller's responsibility at every use site, nothing prevents passing unvalidated data, function signatures don't express domain concepts, and no protection against logging sensitive values.

### Smart constructors without newtypes
Validation functions that return validated strings. Rejected because the type system cannot distinguish validated from unvalidated strings, and there's no mechanism to prevent exposure of sensitive data in logs.

### No memory protection
Storing sensitive data in plain String types. Rejected because data remains in memory after use (vulnerable to memory dumps and swap files) and is easily leaked through debug output or logging.

### Complete redaction for all sensitive types
Treating all sensitive data as Tier 1 (full redaction). Rejected because operational debugging requires some visibility for transaction correlation, such as matching transactions by last 4 digits of card number.

### Runtime flag for masking
Controlling masking through environment variables or runtime configuration. Rejected because it's error-prone (forgetting to enable masking in production), adds runtime overhead, and cannot leverage compile-time guarantees.