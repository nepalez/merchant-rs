use crate::inputs::{Address, Credentials};

#[allow(clippy::upper_case_acronyms)]
pub struct SEPACredentials<'a> {
    pub iban: &'a str,
}

#[allow(clippy::upper_case_acronyms)]
pub struct SEPA<'a> {
    /// International Bank Account Number (IBAN) that can be tokenized (recommended)
    pub credentials: Credentials<'a, SEPACredentials<'a>>,
    /// User billing address (required per PSD2 AML)
    pub billing_address: Address<'a>,
    /// User email for transaction notifications
    pub email: &'a str,
    /// User full name as registered with a bank
    pub full_name: &'a str,
}
