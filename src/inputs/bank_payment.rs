use crate::{AccountHolderType, AccountType, Credentials, Metadata};

pub struct BankPayment<'a> {
    /// The tokenizable bank payment credentials
    pub credentials: Credentials<'a, BankPaymentCredentials<'a>>,
    /// User full name as registered with the bank account
    pub full_name: &'a str,
    /// Type of bank account (checking or savings)
    pub account_type: AccountType,
    /// Type of account holder (individual or company)
    pub holder_type: AccountHolderType,
    /// Method-specific extensions
    pub metadata: Option<Metadata<'a>>,
}

pub struct BankPaymentCredentials<'a> {
    /// The bank account number
    pub account_number: &'a str,
    /// Bank routing identifier
    pub routing_number: &'a str,
}
