use crate::Error;
use crate::inputs::BNPL as Input;
use crate::types::{
    AccountHolderType, Address, BirthDate, EmailAddress, FullName, Metadata, NationalId,
    PhoneNumber,
};

#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct BNPL {
    billing_address: Address,
    email: EmailAddress,
    full_name: FullName,
    account_holder_type: AccountHolderType,
    date_of_birth: Option<BirthDate>,
    national_id: Option<NationalId>,
    phone: Option<PhoneNumber>,
    metadata: Option<Metadata>,
}

impl BNPL {
    /// User billing address
    pub fn billing_address(&self) -> &Address {
        &self.billing_address
    }

    /// User email address
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    /// User full name
    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    /// Type of account holder (individual or company)
    pub fn account_holder_type(&self) -> AccountHolderType {
        self.account_holder_type
    }

    /// User date of birth
    pub fn date_of_birth(&self) -> Option<&BirthDate> {
        self.date_of_birth.as_ref()
    }

    /// National identification number
    pub fn national_id(&self) -> Option<&NationalId> {
        self.national_id.as_ref()
    }

    /// User phone number
    pub fn phone(&self) -> Option<&PhoneNumber> {
        self.phone.as_ref()
    }

    /// Method-specific extensions
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

impl<'a> TryFrom<Input<'a>> for BNPL {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            billing_address: input.billing_address.try_into()?,
            email: input.email.try_into()?,
            full_name: input.full_name.try_into()?,
            account_holder_type: input.account_holder_type,
            date_of_birth: input.date_of_birth.map(TryFrom::try_from).transpose()?,
            national_id: input.national_id.map(TryFrom::try_from).transpose()?,
            phone: input.phone.map(TryFrom::try_from).transpose()?,
            metadata: input.metadata.map(TryFrom::try_from).transpose()?,
        })
    }
}
