use crate::AccountHolderType;
use crate::inputs::{Address, BirthDate, Metadata};

#[allow(clippy::upper_case_acronyms)]
pub struct BNPL<'a> {
    /// User billing address
    pub billing_address: Address<'a>,
    /// User email address
    pub email: &'a str,
    /// User full name
    pub full_name: &'a str,
    /// Type of account holder (individual or company)
    pub account_holder_type: AccountHolderType,
    /// User date of birth
    pub date_of_birth: Option<BirthDate>,
    /// National identification number
    pub national_id: Option<&'a str>,
    /// User phone number
    pub phone: Option<&'a str>,
    /// Method-specific extensions
    pub metadata: Option<Metadata<'a>>,
}
