use crate::inputs::{Address, Metadata};
use crate::types::AccountHolderType;

pub struct InstantAccount<'a> {
    /// User email for transaction notifications
    pub email: &'a str,
    /// User full name as registered with a bank
    pub full_name: &'a str,
    /// Bank account number (CLABE for SPEI)
    pub account_number: Option<&'a str>,
    /// Bank identifier code
    pub bank_code: Option<&'a str>,
    /// User billing address
    pub billing_address: Option<Address<'a>>,
    /// Type of user (person or organization)
    pub holder_type: AccountHolderType,
    /// National identification number (tax ID)
    pub national_id: Option<&'a str>,
    /// User phone number
    pub phone: Option<&'a str>,
    /// Virtual Payment Address (UPI)
    pub virtual_payment_address: Option<&'a str>,
    /// Method-specific extensions
    pub metadata: Option<Metadata<'a>>,
}
