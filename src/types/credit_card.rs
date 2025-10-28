use crate::Error;
use crate::inputs::CreditCard as Input;
use crate::types::{CVV, CardExpiry, CardHolderName, PrimaryAccountNumber};

#[derive(Clone, Debug)]
pub struct CreditCard {
    cvv: CVV,
    number: PrimaryAccountNumber,
    card_expiry: CardExpiry,
    holder_name: CardHolderName,
}

impl CreditCard {
    /// Card Verification Value (CVV/CVC/CID)
    pub fn cvv(&self) -> &CVV {
        &self.cvv
    }

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

impl<'a> TryFrom<Input<'a>> for CreditCard {
    type Error = Error;

    fn try_from(value: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            cvv: value.cvv.try_into()?,
            number: value.number.try_into()?,
            card_expiry: value.card_expiry.try_into()?,
            holder_name: value.holder_name.try_into()?,
        })
    }
}
