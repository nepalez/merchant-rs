use crate::types::{
    BNPL, BankAccount, CashVoucher, CreditCard, InstantAccount, SEPAAccount, Token,
};

/// A marker trait for types that can be used as payment sources
/// (credit cards etc.)
pub(crate) trait PaymentSource {}

impl PaymentSource for BNPL {}
impl PaymentSource for CashVoucher {}
impl PaymentSource for CreditCard {}
impl PaymentSource for BankAccount {}
impl PaymentSource for InstantAccount {}
impl PaymentSource for SEPAAccount {}
impl PaymentSource for Token {}
