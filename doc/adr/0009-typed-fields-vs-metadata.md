# [ADR-0009]: Typed Fields with Metadata Escape Hatch

## Context

Payment source variants require different data fields depending on payment systems. Some data are universal and well-standardized (email, addresses), some are region-specific with defined validation (tax IDs, routing codes), and some are emerging or proprietary. The type system must balance compile-time safety with flexibility for evolving standards.

## Problem

How should payment source variants handle the tension between type safety (validation, security, developer experience) and flexibility (emerging standards, regional variations, gateway extensions)?

## Decision

**Type what you know, provide an escape hatch for what you don't.**

All known, standardized, and validatable data must be typed fields. Metadata serves only as an escape hatch for exceptional cases where standards are absent, evolving, or proprietary.

**Typed fields are used when:**
- Data format is defined by international or regional standards
- Field is required by multiple major providers
- Validation rules can be implemented (length, format, checksum)
- Security tier can be assigned (sensitive data needs protection, data size limits prevent DoS)

**Metadata IS permitted when:**
- Variant covers multiple diverse systems with rapidly changing requirements
- Regional systems with poorly documented specifications
- Standards do not cover gateway proprietary extensions
- Expected to be `None` in 95%+ of use cases

**Metadata IS NOT permitted when:**
- Variant follows stable international standards (ISO, PCI DSS, EMV)
- Data format is well-defined across the industry

**Decision criteria:** "Is this field required by 2+ major providers with a defined, validatable format?" → Yes: typed field. No: metadata or omit entirely.

## Alternatives Considered

### Pure metadata approach
All variant-specific fields in `HashMap<String, String>`.

**Rationale:** Maximum flexibility.

**Rejection:** Violates type safety principle (ADR-0005), no validation, security issues (sensitive data exposure in Debug), poor developer experience.

### Zero metadata approach
All possible fields are typed.

**Rationale:** Maximum type safety.

**Rejection:** Requires core changes for every emerging standard, impractical for rapidly evolving payment systems (50+ instant transfer systems worldwide).

## Consequences

### Pros
- Type safety for standardized fields (compile-time validation)
- Security: typed fields use appropriate protection tiers
- Flexibility: metadata handles exceptional/emerging cases
- Stability: core doesn't change for well-defined additions
- Clear signal: metadata presence indicates evolving standards area

### Cons
- Judgment required: "Is this standard enough to type?"
- Metadata still requires security review (may contain sensitive data)

### Action
- Apply this principle to Request/Response types design
- Document metadata purpose explicitly in variant comments
- Periodic review: promote commonly used metadata fields to typed ones
