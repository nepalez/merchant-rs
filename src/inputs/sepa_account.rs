use crate::inputs::Address;

#[allow(clippy::upper_case_acronyms)]
pub struct SEPAAccount<'a> {
    /// User billing address (required per PSD2 AML)
    pub billing_address: Address<'a>,
    /// User email for transaction notifications
    pub email: &'a str,
    /// User full name as registered with a bank
    pub full_name: &'a str,
    /// International Bank Account Number
    pub iban: &'a str,
}
