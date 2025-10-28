use crate::inputs::Metadata;
use crate::types::{AccountHolderType, AccountType};

pub struct BankAccount<'a> {
    pub account_number: &'a str,
    pub full_name: &'a str,
    pub routing_number: &'a str,
    pub account_type: AccountType,
    pub holder_type: AccountHolderType,
    pub metadata: Option<Metadata<'a>>,
}
