# TokenizedPayment: Pre-Authorized Payment Method Token

## Overview

A payment method represented by a secure, opaque token that references stored payment credentials or completed authentication flows. The token encapsulates all necessary payment information and authorization, eliminating the need to handle sensitive data directly. Tokens can represent stored cards, bank accounts, completed device wallet flows (Apple Pay, Google Pay), e-wallet sessions (Alipay, WeChat Pay), or prepaid voucher credentials (Paysafecard, Neosurf).

## When to Use

### Recurring Payments
- **Subscription billing**: Monthly/annual recurring charges
- **Membership fees**: Automatic renewals
- **Usage-based billing**: Metered services (utilities, SaaS)
- **Installment plans**: Scheduled payment series

### Saved Payment Methods
- **One-click checkout**: Customer selects previously saved card/bank account
- **Customer profiles**: Merchant stores customer payment methods for future use
- **Card-on-file**: Stored for repeat purchases

### Device Wallets
- **Apple Pay**: Token from Apple Pay session
- **Google Pay**: Token from Google Pay session  
- **Samsung Pay**: Token from Samsung Pay session
- **Tokenized cards**: Network tokens from Visa Token Service, Mastercard MDES

### E-Wallet Redirects
- **Alipay**: Token after customer authorizes in Alipay app
- **WeChat Pay**: Token after customer authorizes in WeChat
- **PayPal**: PayPal Vault token or session token
- **Venmo**: Token from Venmo authorization

### Prepaid Vouchers (Direct Entry)
- **Paysafecard**: 16-digit PIN entered by customer
- **Neosurf**: 10-character code from voucher
- **Flexepin**: 16-digit voucher code
- **CASHlib**: 16-digit code
- **AstroPay Card**: Virtual prepaid card details

### Privacy & Anonymity
- **Anonymous payments**: No personal information required
- **Cash-based purchasing**: Vouchers bought with cash at retail
- **Privacy-focused**: Minimal KYC for prepaid instruments
- **Gift payments**: Vouchers as presents

## Authentication Model

Authentication is **embedded in the token itself** — the token represents either stored credentials with prior authorization or a completed authentication flow.

### Token Types

#### 1. Stored Payment Method Tokens
Represent previously saved and verified payment methods:
- **Vault tokens**: Created via server-side tokenization (Tokenizable trait in vault extension)
- **Customer profile tokens**: Associated with merchant customer account
- **Network tokens**: EMV tokens from card networks (DPAN replacing PAN)

**Authentication**: Customer authorized storage during initial setup, subsequent use requires only CVV or 3DS for high-risk transactions.

#### 2. Session Tokens from Completed Flows
Represent completed authentication in an external system:
- **Device wallet tokens**: Customer authenticated biometrically in Apple Pay/Google Pay
- **E-wallet tokens**: Customer logged into Alipay/WeChat Pay and approved payment
- **BNPL tokens**: Customer completed credit check and signed payment plan

**Authentication**: External system performed authentication, token proves it occurred.

#### 3. Prepaid Voucher Credentials
Represent physical or digital prepaid instruments:
- **PIN codes**: 10-16 digit codes from retail vouchers
- **Virtual card numbers**: Prepaid card details from digital vouchers

**Authentication**: Possession of the code/number proves ownership (bearer instrument).

### Authorization Flow

1. **Token acquisition** (varies by type):
   - **Vault**: Customer enters payment details, gateway tokenizes
   - **Device wallet**: Customer authorizes in wallet app, gateway receives token
   - **E-wallet**: Customer logs in and approves, gateway receives session token
   - **Prepaid voucher**: Customer purchases voucher, enters code
   
2. **Token storage**: Merchant stores token for future use (vault tokens, customer profiles)
   
3. **Payment initiation**: Merchant calls `authorize()` with `PaymentSource::TokenizedPayment`
   
4. **Gateway processing**: Gateway resolves token to underlying payment method
   
5. **Authorization**: Gateway processes payment using stored credentials
   
6. **Response**: Standard authorization response returned

### Authorization Request Content

The authorization request contains **only the token string**. Authentication is implicit:
- For vault tokens: prior customer consent to store and use
- For session tokens: completed authentication flow with external provider
- For voucher codes: possession of the bearer instrument

## Standards

### Tokenization Standards
- **[EMV Payment Tokenization](https://www.emvco.com/emv-technologies/payment-tokenisation/)**: Card network token specifications
- **[PCI DSS Token Requirements](https://www.pcisecuritystandards.org/documents/Tokenization_Guidelines_Info_Supplement.pdf)**: Security requirements for tokens
- **[W3C Payment Request API](https://www.w3.org/TR/payment-request/)**: Browser-based payment token handling

### Device Wallet Standards
- **[Apple Pay](https://developer.apple.com/apple-pay/)**: Apple Pay integration and token handling
- **[Google Pay](https://developers.google.com/pay/api)**: Google Pay API and tokenization
- **[EMV Secure Remote Commerce](https://www.emvco.com/emv-technologies/src/)**: Remote payment with tokenization

### E-Wallet APIs
- **Alipay**: Alipay+ integration standards
- **WeChat Pay**: WeChat Pay merchant API
- **PayPal**: PayPal Vault and Express Checkout

### Prepaid Voucher Systems
- **Paysafecard**: PIN-based prepaid payment
- **Neosurf**: International voucher network
- **Flexepin**: Digital voucher codes

## Example Systems

### Gateway Vault Tokens
- **Stripe**: Customer and PaymentMethod objects, tokenized via Stripe.js
- **Braintree**: Customer Vault, client-side tokenization via Drop-in UI
- **Adyen**: Recurring payment tokens, stored payment details
- **Square**: Customer cards on file
- **Authorize.net**: Customer payment profiles

### Device Wallets
- **Apple Pay**: Device-specific account numbers (DPAN)
- **Google Pay**: Tokenized card credentials  
- **Samsung Pay**: MST and NFC tokenized payments
- **Fitbit Pay / Garmin Pay**: Wearable device tokenization

### Network Tokenization
- **Visa Token Service (VTS)**: DPAN tokens replacing PAN
- **Mastercard Digital Enablement Service (MDES)**: Mastercard tokenization
- **American Express Token Service**: Amex tokenization
- **Discover Token Service**: Discover tokenization

### E-Wallets (Session Tokens)
- **Alipay**: China's dominant e-wallet, 1B+ users
- **WeChat Pay**: Integrated with WeChat messaging app
- **PayPal**: 400M+ active accounts globally
- **Venmo**: P2P and merchant payments (US)
- **GrabPay**: Southeast Asia super-app wallet
- **Paytm**: India's leading digital wallet
- **MoMo**: Vietnam mobile wallet
- **GCash**: Philippines mobile wallet

### Prepaid Vouchers
- **Paysafecard**: 50+ countries, buy at retail with cash
- **Neosurf**: International, 135+ countries
- **Flexepin**: Australia, Canada, US, Europe
- **CASHlib**: Europe-focused prepaid vouchers
- **AstroPay**: Latin America, Asia prepaid cards
- **Ukash**: Prepaid voucher system (now merged with Paysafecard)

## Flow Diagram

### Vault Token Flow
```
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ [Initial setup - see vault extension docs]
       │ Customer enters payment details
       │ Gateway tokenizes and stores
       ▼
┌─────────────┐
│  Merchant   │ ← Token stored in customer profile
└──────┬──────┘
       │
       │ [Later - recurring payment]
       │ 1. Retrieves token from customer profile
       │ 2. Calls authorize() with TokenizedPayment
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 3. Resolves token to stored payment method
       │ 4. Processes authorization
       ▼
┌─────────────┐
│  Merchant   │ ← Authorization complete
└─────────────┘
```

### Device Wallet Flow
```
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ 1. Taps "Pay with Apple Pay"
       ▼
┌─────────────┐
│  Merchant   │
└──────┬──────┘
       │ 2. Initiates Apple Pay session
       ▼
┌─────────────┐
│ Apple Pay   │
└──────┬──────┘
       │ 3. Customer authenticates (Face ID/Touch ID)
       │ 4. Returns payment token
       ▼
┌─────────────┐
│  Merchant   │
└──────┬──────┘
       │ 5. Calls authorize() with TokenizedPayment
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┘
       │ 6. Decrypts token, processes authorization
       ▼
┌─────────────┐
│  Merchant   │ ← Authorization complete
└─────────────┘
```

### Prepaid Voucher Flow
```
┌─────────────┐
│  Customer   │
└──────┬──────┘
       │ 1. Purchases Paysafecard at retail store
       │ 2. Receives 16-digit PIN
       ▼
┌─────────────┐
│  Merchant   │
└──────┬──────┘
       │ 3. Customer enters PIN at checkout
       │ 4. Calls authorize() with TokenizedPayment(PIN)
       ▼
┌─────────────┐
│   Gateway   │
└──────┬──────┏
       │ 5. Validates PIN with Paysafecard
       │ 6. Checks balance, deducts amount
       ▼
┌─────────────┐
│  Merchant   │ ← Authorization complete
└─────────────┘
```

## Security Considerations

### Token Security
- Tokens are not sensitive authentication data per PCI DSS
- However, tokens provide access to payment methods and must be protected
- Use `Token` type which implements memory zeroization on drop
- Tokens should be masked in logs (full redaction)
- Access only via unsafe `with_exposed_secret()` method

### Token Types and Risks
- **Single-use tokens**: Expire after first use or short time window (most secure)
- **Multi-use tokens**: Can be reused indefinitely (higher risk if leaked)
- **Merchant-specific tokens**: Only work with issuing merchant (more secure)
- **Cross-merchant tokens**: Work across multiple merchants (higher risk)

### Stored Payment Method Security
- Customer must explicitly consent to storage
- PCI DSS requirements apply to vault infrastructure
- Tokens should not be shareable between customers
- Implement tokenization scope (restrict to original merchant)
- Regular token lifecycle management (expire inactive tokens)

### Device Wallet Security
- Biometric authentication on device
- Tokens are device-specific (DPAN)
- Cryptographic domain separation
- If device is compromised, tokens can be remotely deactivated

### Prepaid Voucher Security
- PINs/codes are bearer instruments (possession = ownership)
- No recourse if code is stolen or phished
- Check voucher balance before attempting authorization
- Implement rate limiting on code validation attempts
- Monitor for suspicious patterns (sequential codes, bulk testing)

### Fraud Prevention
- Monitor token usage patterns (geographic, velocity, amount)
- Implement device fingerprinting for stored payment methods
- Require 3DS or step-up authentication for high-risk transactions
- Flag tokens associated with prior fraud
- Implement token reputation scoring

### Compliance
- **PCI DSS**: Tokens are out of scope, but vault infrastructure must be compliant
- **PSD2**: Stored payment methods may require SCA for new transactions
- **GDPR**: Tokens linked to personal data must follow data protection rules
- **Consumer protection**: Clear disclosure of saved payment method usage
- **CCPA**: Customers must be able to request deletion of stored tokens

### Token Lifecycle
- **Creation**: Via tokenization API or external flow completion
- **Storage**: Securely stored with access controls
- **Usage**: Regular authorization with fraud checks
- **Renewal**: Update expired cards (card updater services)
- **Revocation**: Customer can delete, or system expires inactive tokens
- **Rotation**: Periodic token rotation for security

### Business Considerations
- **Recurring payments**: Significantly improves subscription retention
- **One-click checkout**: Reduces cart abandonment
- **Card updates**: Network token updater services reduce payment failures
- **Lower fees**: Device wallets may have lower interchange rates
- **No chargebacks**: Prepaid vouchers are non-reversible
- **Geographic limitations**: Device wallet and e-wallet availability varies by region
