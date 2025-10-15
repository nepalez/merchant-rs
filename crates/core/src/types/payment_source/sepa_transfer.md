# SEPATransfer: SEPA Bank Transfer Payment

## Overview

Bank transfer within the Single Euro Payments Area for EUR-denominated transactions. Enables direct transfers between bank accounts across 36 European countries with unified standards. SEPA Instant provides real-time settlement (10 seconds), standard SEPA takes 1–2 business days. Uses IBAN as the primary account identifier.

## Authentication Model

Authentication model **depends on SEPA variant**:

### SEPA Instant Credit Transfer
- **PSD2 Strong Customer Authentication**: Customer redirected to bank for SCA
- **Similar to InstantBankTransfer**: Bank login and transaction approval
- **Real-time settlement**: 10 seconds maximum
- **24/7 availability**: No banking hours restrictions

### Standard SEPA Direct Debit
- **Pre-authorized mandate**: Customer signs SEPA Direct Debit mandate
- **No authentication in authorization request**: Mandate authorizes recurring debits
- **Settlement**: 1–2 business days
- **Customer protection**: 8-week dispute window for unauthorized debits

The authorization request contains **only account identification and customer data**. For SEPA Instant, authentication happens in a redirect flow. For SEPA Debit, authentication occurred during mandate setup.

## Standards

- **[ISO 20022](https://www.iso20022.org/)**: XML message format for SEPA payments
- **[ISO 13616](https://www.iso.org/standard/81090.html)**: IBAN (International Bank Account Number) standard
- **[PSD2](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32015L2366)**: Payment Services Directive (AML/KYC requirements)
- **[EPC SEPA Instant Credit Transfer Scheme](https://www.europeanpaymentscouncil.eu/what-we-do/sepa-instant-credit-transfer)**: European Payments Council specifications

## SEPA Zone Coverage

36 countries participate in SEPA:

### EU Member States (27)
Austria, Belgium, Bulgaria, Croatia, Cyprus, the Czech Republic, Denmark, Estonia, Finland, France, Germany, Greece, Hungary, Ireland, Italy, Latvia, Lithuania, Luxembourg, Malta, the Netherlands, Poland, Portugal, Romania, Slovakia, Slovenia, Spain, Sweden

### EEA Countries (3)
Iceland, Liechtenstein, Norway

### Other (6)
Andorra, Monaco, San Marino, Switzerland, United Kingdom, Vatican City

## Flow Diagram

### SEPA Instant Flow
```
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ 1. Provides IBAN
       ▼
┌─────────────┐
│  Merchant   │
└──────┬──────┘
       │ 2. Calls authorize() with SEPATransfer
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 3. Initiates SEPA Instant + PSD2 SCA
       │ 4. Returns redirect URL
       ▼
┌─────────────┐
│  Customer   │ (Redirected to bank)
└──────┬──────┘
       │ 5. Bank login + SCA
       │ 6. Approves payment
       ▼
┌─────────────┐
│     Bank    │
└──────┬──────┘
       │ 7. Real-time settlement (10 seconds)
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 8. Confirmation + redirect back
       ▼
┌─────────────┐
│  Merchant   │ ← Payment complete
└─────────────┘
```

### Standard SEPA Debit Flow
```
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ 1. Signs SEPA mandate (pre-authorization)
       ▼
┌─────────────┐
│   Gateway   │ ← Mandate stored
└──────┬──────┘
       │
       │ [Time passes]
       │
       │ 2. Merchant calls authorize()
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 3. Initiates SEPA Debit (no customer interaction)
       ▼
┌─────────────┐
│     Bank    │
└──────┬──────┘
       │ 4. Settlement (1-2 business days)
       ▼
┌─────────────┐
│  Merchant   │ ← Funds received
└─────────────┘
```

## Security Considerations

### IBAN Handling
- IBAN is not classified as Sensitive Authentication Data per PCI DSS
- However, it is critical PII and financial access data
- Use `IBAN` type which implements appropriate protection
- Validate IBAN format and check digit per ISO 13616

### PSD2 Compliance
- **Strong Customer Authentication (SCA)**: Required for SEPA Instant in most cases
- **AML/KYC requirements**: Billing address required for AML compliance
- **Customer rights**: 8-week dispute window for SEPA Direct Debit
- **Data protection**: Comply with GDPR for customer data

### Fraud Prevention
- Validate IBAN format and check digit
- Verify IBAN belongs to SEPA zone
- Check account holder name matching (where supported)
- Monitor for unusual patterns
- Implement velocity limits

### Mandate Management (SEPA Debit)
- Store mandate reference ID
- Track mandate status (active, canceled, expired)
- Provide pre-notification before each debit
- Handle mandate cancellations
- Respect the 8-week dispute window
