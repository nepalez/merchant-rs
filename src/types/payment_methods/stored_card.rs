use std::convert::TryFrom;

use crate::Error;
use crate::inputs::{StoredCard as Input, StoredCardCredentials as CredentialsInput};
use crate::types::{
    CardExpiry, CardHolderName, Credentials, CreditCard, InternalPaymentMethod, PaymentMethod,
    PrimaryAccountNumber, TokenizablePaymentMethod,
};

/// Credit or Debit Card stored for MTI (Merchant Initiated Transactions)
#[derive(Clone, Debug)]
pub struct StoredCard {
    credentials: Credentials<StoredCardCredentials>,
}

/// Tokenizable credentials for StoredCard
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
