use std::convert::TryFrom;

use crate::Error;
use crate::inputs::SEPAAccount as Input;
use crate::types::{Address, EmailAddress, FullName, IBAN};

#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct SEPAAccount {
    billing_address: Address,
    email: EmailAddress,
    full_name: FullName,
    iban: IBAN,
}

impl SEPAAccount {
    /// User billing address (required per PSD2 AML)
    pub fn billing_address(&self) -> &Address {
        &self.billing_address
    }

    /// User email for transaction notifications
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    /// User full name as registered with bank
    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    /// International Bank Account Number
    pub fn iban(&self) -> &IBAN {
        &self.iban
    }
}

impl<'a> TryFrom<Input<'a>> for SEPAAccount {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            billing_address: input.billing_address.try_into()?,
            email: input.email.try_into()?,
            full_name: input.full_name.try_into()?,
            iban: input.iban.try_into()?,
        })
    }
}
