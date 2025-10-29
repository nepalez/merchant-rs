use crate::types::{
    BNPL, CashVoucher, CreditCard, DirectBankAccount, InstantBankAccount, SEPAAccount, Token,
};

/// A marker trait for types that can be used as payment sources
/// (credit cards etc.)
pub(crate) trait PaymentSource {}

impl PaymentSource for BNPL {}
impl PaymentSource for CashVoucher {}
impl PaymentSource for CreditCard {}
impl PaymentSource for DirectBankAccount {}
impl PaymentSource for InstantBankAccount {}
impl PaymentSource for SEPAAccount {}
impl PaymentSource for Token {}
