use crate::{AccountNumber, BankName, RoutingNumber};

#[derive(Debug, Clone)]
pub struct BankAccountDetails {
    pub account_number: AccountNumber,
    pub routing_number: RoutingNumber,
    pub bank_name: Option<BankName>,
}
