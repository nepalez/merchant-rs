use std::convert::TryFrom;

use crate::Error;
use crate::inputs::CashVoucher as Input;
use crate::types::{Address, FullName, Metadata, NationalId};

#[derive(Clone, Debug)]
pub struct CashVoucher {
    full_name: FullName,
    billing_address: Option<Address>,
    national_id: Option<NationalId>,
    metadata: Option<Metadata>,
}

impl CashVoucher {
    /// User full name
    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    /// User billing address
    pub fn billing_address(&self) -> Option<&Address> {
        self.billing_address.as_ref()
    }

    /// National identification number (CPF/CNPJ for Boleto)
    pub fn national_id(&self) -> Option<&NationalId> {
        self.national_id.as_ref()
    }

    /// Method-specific extensions
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

impl<'a> TryFrom<Input<'a>> for CashVoucher {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            full_name: input.full_name.try_into()?,
            billing_address: input.billing_address.map(TryFrom::try_from).transpose()?,
            national_id: input.national_id.map(TryFrom::try_from).transpose()?,
            metadata: input.metadata.map(TryFrom::try_from).transpose()?,
        })
    }
}
