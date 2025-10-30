# [ADR-0006]: Security Strategy for Sensitive Data

## Context

Payment processing handles highly sensitive data including card numbers, CVV codes, bank account details, and authentication tokens. This data must be protected both in memory (to prevent exposure through memory dumps, swap files, or heap inspection) and in logs (to prevent exposure through debug output, error messages, or trace logs). Different types of data have different sensitivity levels and regulatory requirements.

## Problem

How should the system protect sensitive data in memory and prevent accidental exposure through logs, debug output, or error messages?

## Decision

Implement a graduated security strategy with memory protection and debug masking.

**Security tiers:**
Each domain type is assigned a security tier determining its protection level:
- Tier 1 (Maximum): Full redaction—CVV, passwords (shows only "***")
- Tier 2 (High): Partial visibility for correlation—PAN shows first 6 + last 4, tokens show prefix/suffix
- Tier 3 (Medium): Masked for privacy—email: shows `a...@domain`, phone shows last four digits
- Tier 4 (Operational): Length-only masking: descriptions show "[N chars]"
- Tier 5 (Public): No masking: transaction IDs, status codes, public identifiers

**Memory protection:**
Sensitive types (Tier 1–2) use ZeroizeOnDrop from the zeroize_derive crate for automatic memory zeroization on a drop. The AsUnsafeRef trait provides controlled access through unsafe methods while preventing accidental exposure through Clone, Debug, or Display. The Masked trait provides safe methods (first_chars, last_chars) for partial visibility in debug output.

**Debug masking:**
All newtypes implement custom Debug that respects their assigned tier. Debug output is designed for safe logging—no sensitive data exposure while maintaining operational utility for troubleshooting.

This strategy builds on the newtype pattern from ADR-0005: newtypes provide the structure, security tiers determine the protection level.

## Consequences

### Pros
- Memory safety: sensitive data zeroized immediately on a drop
- Safe logging: debug output cannot expose sensitive information
- Graduated approach balances security with operational needs
- Type system enforces protection (AsUnsafeRef trait requires explicit unsafe access)
- Consistent protection strategy across entire codebase
- Operational utility: partial visibility enables transaction correlation without exposing full values
- Compile-time guarantees: impossible to accidentally log sensitive data

### Cons
- Debugging is more challenging with redacted values
- Performance overhead from memory zeroization
- Developers must use unsafe methods to access Tier 1–2 data
- Learning curve: understanding security tiers and when to use unsafe access
- Custom Debug implementations are required for all types

## Alternatives Considered

### No memory protection
Storing sensitive data in plain String types. Rejected because data remains in memory after use (vulnerable to memory dumps and swap files), easily leaked through debug output or logging, and provides no protection against accidental exposure.

### Complete redaction for all sensitive types
Treating all sensitive data as Tier 1 (full redaction). Rejected because operational debugging requires some visibility into transaction correlation, such as matching transactions by last four digits of a card number or recognizing token prefixes.

### Runtime flag for masking
Controlling masking through environment variables or runtime configuration. Rejected because it's error-prone (forgetting to enable masking in production), adds runtime overhead to check flags on every Debug call, and cannot leverage compile-time guarantees.

### No Debug implementation for sensitive types
Removing Debug trait from sensitive types entirely. Rejected because it breaks common debugging workflows, prevents using sensitive types in contexts requiring Debug bounds, and makes development unnecessarily difficult.

### Custom security wrapper per type
Each sensitive type implements its own protection mechanism. Rejected because it leads to inconsistent protection strategies, duplicated security logic across types, and makes it difficult to audit security properties system-wide.