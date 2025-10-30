# DirectCarrierBilling: Mobile Carrier Billing Payment

## Overview

Payment is charged directly to the customer's mobile phone bill or deducted from the prepaid balance. Customer authorizes the charge via SMS PIN or mobile app confirmation. Charge appears on the next phone bill (postpaid) or is immediately deducted from balance (prepaid). No bank account or credit card is required — the mobile carrier acts as the payment intermediary and assumes fraud risk.

## When to Use

- **Mobile-first markets**: Regions where mobile phone penetration exceeds banking access
- **Digital content purchases**: Games, apps, music, videos, subscriptions
- **Micropayments**: Small transactions where card fees would be prohibitive ($0.50-$50 typical range)
- **Unbanked/underbanked users**: Customers without access to traditional payment methods
- **Youth market**: Users too young for credit cards
- **Convenience**: One-click checkout without entering payment details
- **Emerging markets**: Southeast Asia, Africa, Latin America, Middle East

## Authentication Model

Authentication occurs **via carrier SMS challenge-response**, not in authorization request:

### SMS Authentication Flow Steps

1. **Merchant initiates**: Calls `authorize()` with customer's phone number
2. **Gateway routes to carrier**: Identifies carrier from phone number, initiates DCB request
3. **Carrier sends PIN**: Customer receives SMS with a one-time PIN code
4. **Customer enters PIN**: Types PIN into merchant's checkout or responds to SMS
5. **Carrier validates PIN**: Confirms PIN matches and customer has sufficient balance/credit limit
6. **Carrier authorizes charge**: Approves transaction, adds to bill or deducts from balance
7. **Gateway confirms**: Returns authorization response to merchant
8. **Settlement**: Carrier collects payment from the customer, remits to merchant (typically 30–60 days)

### Authorization Request Content

The authorization request contains **only the phone number as the primary payment identifier**. Authentication happens through the carrier's SMS challenge system, not in the authorization request.

## Standards

- **GSMA Mobile Connect**: [Identity and authentication standard](https://www.gsma.com/identity/mobile-connect)
- **GSMA Carrier Billing**: Best practices and technical specifications
- **3GPP**: Telecommunications standards (TS 24.008 for SMS)
- **MEF (Mobile Ecosystem Forum)**: Industry best practices for carrier billing

## Example Systems

### Global Aggregators
- **Boku**: 200+ carriers in 60+ countries, major DCB aggregator
- **Fortumo**: Strong in Europe, Asia, Latin America
- **Centili**: Emerging markets focus, 80+ carriers
- **Digital Turbine (Trialpay)**: Mobile monetization platform
- **Docomo Digital**: Japan and international markets

### Direct Carrier Integrations
- **Verizon Wireless** (US): Premium SMS and carrier billing
- **AT&T** (US): Direct carrier billing for content
- **Vodafone** (Europe/Africa/Asia): Multi-market presence
- **MTN** (Africa): Major African mobile operator
- **Telkomsel** (Indonesia): Largest Indonesian operator
- **Globe/Smart** (Philippines): Philippine carriers
- **Airtel** (India): Major Indian carrier
- **Orange** (Europe/Africa): International carrier

### Regional Aggregators
- **Mopay** (Europe): European carrier billing
- **Dimoco** (Europe): Focus on Austria, Germany, CEE
- **Ericsson IPX**: Carrier billing infrastructure
- **Danal** (Asia): Korea and Asian markets

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
       │ 2. Calls authorize() with phone number
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 3. Routes to carrier via aggregator
       ▼
┌─────────────┐
│   Carrier   │
└──────┬──────┘
       │ 4. Generates PIN, sends SMS
       ▼
┌─────────────┐
│  Customer   │ ← Receives SMS with PIN
└──────┬──────┘
       │ 5. Enters PIN on merchant site
       ▼
┌─────────────┐
│  Merchant   │
└──────┬──────┘
       │ 6. Submits PIN to gateway
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 7. Validates PIN with carrier
       ▼
┌─────────────┐
│   Carrier   │
└──────┬──────┘
       │ 8. Verifies PIN, checks balance/limit
       │ 9. Authorizes charge
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 10. Returns authorization response
       ▼
┌─────────────┐
│  Merchant   │ ← Charge approved
└──────┬──────┘
       │
       │ [30-60 days later]
       │
       ▼
┌─────────────┐
│   Carrier   │ → Collects from customer, remits to merchant
└─────────────┘
```

## Security Considerations

### Phone Number as Identity
- Phone number is the primary payment identifier
- No credit card or bank account data collected
- SIM swap attacks are a risk (attacker takes over phone number)
- Carrier validates account ownership before authorizing

### SMS Security
- PIN-based authentication via SMS
- SMS interception is possible (SS7 attacks, SIM swapping)
- PINs are one-time use and time-limited
- Some carriers use mobile app confirmation instead of SMS

### Fraud Prevention
- Velocity limits on transactions per phone number
- Carrier performs spending limit checks
- Age verification for restricted content
- Device fingerprinting to detect suspicious patterns
- Monitor for stolen phones being used for fraud

### Spending Limits
- Carriers impose daily/monthly spending limits per account
- Typical limits: $10–50 per day, $50–200 per month
- Limits vary by carrier, account type, and payment history
- Prepaid accounts may have lower limits than postpaid

### Compliance
- **Age restrictions**: Carriers enforce minimum age (typically 18+)
- **Content restrictions**: Adult content, gambling often prohibited
- **Consumer protection**: Clear pricing, easy opt-out for subscriptions
- **GDPR**: Phone numbers are PII, must be protected
- **COPPA** (US): Stricter rules for users under 13
- **TCPA** (US): Regulations on SMS messaging and consent

### Business Model Considerations
- **High merchant fees**: Typically 40–60% of transaction value
- **Delayed settlement**: 30–60 days from transaction to payout
- **Chargebacks**: Customers can dispute charges with the carrier
- **Refunds**: Process varies by carrier, often manual
- **Subscription management**: Carriers may handle recurring billing
- **Geographic limitations**: Coverage varies significantly by region

### User Experience
- **Friction**: SMS delivery delays can slow checkout
- **Failed PIN**: Customers may not receive SMS (network issues, spam filters)
- **Spending limits**: Transactions may fail if the customer hits the daily / monthly limit
- **Prepaid balance**: Prepaid users may have insufficient balance
- **Roaming**: Authentication may fail when a customer is roaming internationally

### Regional Considerations
- **Strong in Asia-Pacific**: Indonesia, Philippines, Thailand, India
- **Growing in Africa**: Kenya, Nigeria, South Africa
- **Limited in North America**: Less common than cards/digital wallets
- **Variable in Europe**: More common in Eastern Europe
- **Latin America**: Growing adoption in Brazil, Mexico
- **Middle East**: Popular in UAE, Saudi Arabia, Egypt
