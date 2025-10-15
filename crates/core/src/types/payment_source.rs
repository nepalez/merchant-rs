use crate::{BankAccountDetails, PaymentToken};

#[derive(Debug, Clone)]
pub enum PaymentSource {
    Token(PaymentToken),
    BankAccount(BankAccountDetails),
    Other(String),
}
