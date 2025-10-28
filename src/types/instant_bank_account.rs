use crate::Error;
use crate::inputs::InstantBankAccount as Input;
use crate::types::{
    AccountHolderType, AccountNumber, Address, BankCode, EmailAddress, FullName, Metadata,
    NationalId, PhoneNumber, VirtualPaymentAddress,
};

#[derive(Clone, Debug)]
pub struct InstantBankAccount {
    email: EmailAddress,
    full_name: FullName,
    account_number: Option<AccountNumber>,
    bank_code: Option<BankCode>,
    billing_address: Option<Address>,
    customer_type: Option<AccountHolderType>,
    national_id: Option<NationalId>,
    phone: Option<PhoneNumber>,
    virtual_payment_address: Option<VirtualPaymentAddress>,
    metadata: Option<Metadata>,
}

impl InstantBankAccount {
    /// User email for transaction notifications
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    /// User full name as registered with a bank
    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    /// Bank account number (CLABE for SPEI)
    pub fn account_number(&self) -> Option<&AccountNumber> {
        self.account_number.as_ref()
    }

    /// Bank identifier code
    pub fn bank_code(&self) -> Option<&BankCode> {
        self.bank_code.as_ref()
    }

    /// User billing address
    pub fn billing_address(&self) -> Option<&Address> {
        self.billing_address.as_ref()
    }

    /// Type of user (person or organization)
    pub fn customer_type(&self) -> Option<AccountHolderType> {
        self.customer_type
    }

    /// National identification number (tax ID)
    pub fn national_id(&self) -> Option<&NationalId> {
        self.national_id.as_ref()
    }

    /// User phone number
    pub fn phone(&self) -> Option<&PhoneNumber> {
        self.phone.as_ref()
    }

    /// Virtual Payment Address (UPI)
    pub fn virtual_payment_address(&self) -> Option<&VirtualPaymentAddress> {
        self.virtual_payment_address.as_ref()
    }

    /// Method-specific extensions
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

impl<'a> TryFrom<Input<'a>> for InstantBankAccount {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            email: input.email.try_into()?,
            full_name: input.full_name.try_into()?,
            account_number: input.account_number.map(TryFrom::try_from).transpose()?,
            bank_code: input.bank_code.map(TryFrom::try_from).transpose()?,
            billing_address: input.billing_address.map(TryFrom::try_from).transpose()?,
            customer_type: input.customer_type,
            national_id: input.national_id.map(TryFrom::try_from).transpose()?,
            phone: input.phone.map(TryFrom::try_from).transpose()?,
            virtual_payment_address: input
                .virtual_payment_address
                .map(TryFrom::try_from)
                .transpose()?,
            metadata: input.metadata.map(TryFrom::try_from).transpose()?,
        })
    }
}
