# CashVoucher: Cash-Based Voucher Payment

## Overview

Customer receives a voucher with a barcode or reference number, then pays with cash at physical locations (convenience stores, banks, post offices). Settlement is asynchronous—payment confirmation arrives via webhook after the customer completes payment at a physical location (typically 1–3 days, up to voucher expiration). Gateway generates the voucher, customer takes it to the payment location.

## Authentication Model

Authentication occurs **through physical payment**, not in authorization request:

### Voucher Generation and Payment Flow

1. **Merchant initiates**: Calls `authorize()` with customer data
2. **Gateway generates voucher**: Creates unique voucher with barcode and reference number
3. **Customer receives voucher**: Via email (PDF) or displayed on screen
4. **Customer goes to location**: Physically visits convenience store, bank, or post office
5. **Customer presents voucher**: Shows barcode or provides reference number to cashier
6. **Cash payment**: Customer pays cash to cashier
7. **Receipt confirmation**: Payment location confirms transaction to gateway
8. **Webhook notification**: Gateway notifies merchant asynchronously (1–3 days)

### Authorization Request Content

The authorization request contains **customer data for voucher generation and compliance**, not authentication credentials. Authentication is inherently physical — a customer's presence at a payment location with cash.

## Standards

- **[FEBRABAN Standards](https://portal.febraban.org.br/)**: Brazilian Federation of Banks (Boleto specifications)
- **OXXO Specifications**: Mexico convenience store payment standards
- **Konbini Standards**: Japan convenience store payment system
- **Multibanco Standards**: Portuguese ATM/retail payment network

## Example Systems

### Latin America
- **Boleto Bancário** (Brazil): Most popular payment method in Brazil, 3-day validity typical, requires CPF/CNPJ
- **OXXO** (Mexico): 18,000+ locations, cash payment at convenience stores
- **PagoEfectivo** (Peru): Bank branches and payment agents
- **Efecty** (Colombia): Payment network with 10,000+ locations
- **Servipag** (Chile): Bill payment and cash collection network

### Asia Pacific
- **Konbini** (Japan): 7-Eleven, FamilyMart, Lawson stores, 30-day validity
- **7-Eleven** (various): Cash payment at 7-Eleven stores in multiple countries
- **Alfamart/Indomaret** (Indonesia): Major retail chains accepting cash vouchers

### Europe
- **Multibanco** (Portugal): ATM and retail payment, widely used in Portugal
- **Barzahlen** (Germany/Austria): Cash payment at retail partners

### Middle East/Africa
- **Fawry** (Egypt): Payment at retail outlets and mobile wallets
- **ePay** (various): Multi-country cash voucher network

## Flow Diagram
```
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ 1. Initiates payment
       ▼
┌─────────────┐
│  Merchant   │
└──────┬──────┘
       │ 2. Calls authorize() with customer data
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 3. Generates unique voucher (barcode + reference)
       │ 4. Returns voucher (PDF or display)
       ▼
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ 5. Receives voucher via email/screen
       │ 6. Goes to physical location (store/bank)
       ▼
┌─────────────┐
│Payment Point│ (OXXO/Konbini/Bank)
└──────┬──────┘
       │ 7. Customer presents voucher + pays cash
       │ 8. Cashier scans barcode / enters reference
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 9. Payment confirmed (async, 1-3 days)
       │ 10. Webhook notification to merchant
       ▼
┌─────────────┐
│  Merchant   │ ← Payment complete, ship goods
└─────────────┘
```

## Security Considerations

### Customer Data Protection
- Full name, email, address, and optionally tax ID (CPF/CNPJ for Boleto)
- All PII must be protected per GDPR, LGPD, or local regulations
- Use appropriate types with masking where required
- Tax IDs (CPF/CNPJ) should use `NationalId` type with SecretString

### Voucher Security
- Unique voucher codes prevent duplicate payments
- Barcodes designed to prevent counterfeiting
- Expiration dates limit a fraud window (typically 1–7 days)
- Payment location validates voucher before accepting cash

### Fraud Prevention
- Validate tax ID format (CPF/CNPJ check digits for Boleto)
- Monitor for duplicate voucher generation attempts
- Track voucher usage patterns
- Implement expiration to limit a fraud window
- Verify customer email for voucher delivery

### Compliance
- **Boleto regulations** (Brazil): CPF/CNPJ required for identification and tax reporting
- **AML/KYC**: Some jurisdictions require customer identification
- **Tax reporting**: Transaction records for tax authorities
- **LGPD/GDPR**: Customer data protection requirements
- **Consumer protection**: Clear expiration dates and payment instructions
