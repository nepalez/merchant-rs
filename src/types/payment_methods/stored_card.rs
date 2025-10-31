use std::convert::TryFrom;

use crate::Error;
use crate::inputs::{StoredCard as Input, StoredCardCredentials as CredentialsInput};
use crate::types::{
    CardExpiry, CardHolderName, Credentials, CreditCard, InternalPaymentMethod, PaymentMethod,
    PrimaryAccountNumber, TokenizablePaymentMethod,
};

/// Stored Credit or Debit Card for Merchant Initiated Transactions
///
/// ## Overview
///
/// Payment card credentials stored after initial authorization for subsequent merchant-initiated
/// transactions (MIT). Used for recurring subscriptions, installment payments, or unscheduled
/// account top-ups. Requires initial customer authorization and consent to store credentials.
/// CVV is never stored per PCI DSS, only PAN, expiry, and cardholder name.
///
/// ## When to Use
///
/// - **Subscription billing**: Recurring charges at regular intervals
/// - **Installment payments**: Split payments over multiple charges
/// - **Account top-ups**: Automatic balance replenishment when funds are low
/// - **Usage-based billing**: Post-usage charges (utilities, cloud services)
/// - **Standing orders**: Pre-authorized recurring payments
///
/// ## Authentication Model
///
/// > Authentication occurs **during initial authorization with CVV**, not in stored credential usage!
///
/// ### Initial Authorization (CIT - Customer Initiated Transaction)
///
/// 1. **Customer provides full card details**: Including CVV for initial transaction
/// 2. **CVV validates card possession**: Proves customer has physical card
/// 3. **Customer grants storage consent**: Explicit permission to store credentials
/// 4. **Gateway tokenizes credentials**: PAN replaced with non-sensitive token
/// 5. **CVV discarded**: Must never be stored per PCI DSS 3.2
///
/// ### Subsequent MIT Transactions
///
/// 1. **Merchant initiates**: Uses stored token without customer interaction
/// 2. **No CVV required**: MIT transactions don't require CVV
/// 3. **Network tokens**: May use EMV tokens (Visa Token Service, Mastercard MDES)
/// 4. **Transaction indicators**: MIT indicator flags inform issuer of transaction type
///
/// ## Differences from CreditCard
///
/// | Aspect | CreditCard | StoredCard |
/// |--------|------------|------------|
/// | **CVV** | Required | Never present (PCI DSS 3.2) |
/// | **Initiation** | Customer Initiated (CIT) | Merchant Initiated (MIT) |
/// | **Authentication** | CVV + optional 3DS | Token-based, no CVV |
/// | **Storage** | Ephemeral, discarded after auth | Tokenized, stored long-term |
/// | **Use case** | One-time payments | Recurring/installment payments |
///
/// ## Security Considerations
///
/// ### PCI DSS Compliance
/// - **Never store CVV**: CVV/CVC must never be stored after initial authorization
/// - **Tokenization required**: Use gateway tokens or network tokens instead of raw PANs
/// - **Encryption at rest**: All stored card data must be encrypted
/// - **Access controls**: Restrict who can access stored credentials
/// - **Audit logging**: Log all access to stored card data
///
/// ### Network Tokenization
/// - **Visa Token Service (VTS)**: Replaces PAN with network token
/// - **Mastercard MDES**: Digital Enablement Service for tokenization
/// - **Benefits**: Tokens are domain-restricted, can be suspended independently
/// - **Dynamic data**: Network tokens may include dynamic CVV equivalents
///
/// ### MIT Indicators
/// - **Recurring**: Regular interval subscriptions
/// - **Installment**: Fixed number of payments
/// - **Unscheduled**: Merchant-initiated when needed (top-ups, usage billing)
/// - **Proper indicators reduce declines**: Issuers apply different risk rules to MIT
///
/// ### Fraud Prevention
/// - **Initial CIT verification**: Strong CVV + 3DS check before storing
/// - **Consent management**: Clear customer consent with opt-out options
/// - **Velocity monitoring**: Track MIT frequency and amounts
/// - **Card updater services**: Automatically update expired cards
/// - **Failed payment monitoring**: Multiple failures may indicate stolen card
///
/// ### Compliance
/// - **PCI DSS**: Tokenization and encryption requirements
/// - **SCA exemptions**: MIT may be exempt from PSD2 SCA after initial CIT
/// - **Customer consent**: Must have explicit permission to store and charge
/// - **Notification requirements**: Inform customers before charging (varies by region)
#[derive(Clone, Debug)]
pub struct StoredCard {
    credentials: Credentials<StoredCardCredentials>,
}

/// Tokenizable credentials for StoredCard
///
/// Contains card details without CVV (never stored per PCI DSS).
/// Should be tokenized by the gateway rather than stored as plain text.
#[derive(Clone, Debug)]
pub struct StoredCardCredentials {
    number: PrimaryAccountNumber,
    card_expiry: CardExpiry,
    holder_name: CardHolderName,
}

// Marker implementations

impl PaymentMethod for StoredCard {}
impl InternalPaymentMethod for StoredCard {}
impl TokenizablePaymentMethod for StoredCard {}

// Converters

impl StoredCard {
    pub fn credentials(&self) -> &Credentials<StoredCardCredentials> {
        &self.credentials
    }
}

impl StoredCardCredentials {
    /// Primary Account Number (PAN)
    pub fn number(&self) -> &PrimaryAccountNumber {
        &self.number
    }

    /// Card expiration date (month and year)
    pub fn card_expiry(&self) -> &CardExpiry {
        &self.card_expiry
    }

    /// Cardholder name as embossed on the card
    pub fn holder_name(&self) -> &CardHolderName {
        &self.holder_name
    }
}

impl<'a> TryFrom<CredentialsInput<'a>> for StoredCardCredentials {
    type Error = Error;

    fn try_from(value: CredentialsInput<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            number: value.number.try_into()?,
            card_expiry: value.card_expiry.try_into()?,
            holder_name: value.holder_name.try_into()?,
        })
    }
}

impl<'a> TryFrom<Input<'a>> for StoredCard {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            credentials: input.credentials.try_into()?,
        })
    }
}

impl From<CreditCard> for StoredCard {
    fn from(input: CreditCard) -> Self {
        Self {
            credentials: Credentials::Plain(StoredCardCredentials {
                number: input.number,
                card_expiry: input.card_expiry,
                holder_name: input.holder_name,
            }),
        }
    }
}
