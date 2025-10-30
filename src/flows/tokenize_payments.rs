use async_trait::async_trait;

use crate::Error;
use crate::internal::PaymentSource;
use crate::types::{
    BNPL, BankAccount, CashVoucher, CreditCard, InstantAccount, SEPAAccount, Token,
};

/// Optional trait for payment gateways that support tokenizing payment data.
/// The received token can be used later to either charge or authorize the payment.
///
/// This trait can be used to support 3D Secure payments.
#[async_trait]
pub trait TokenizePayments {
    #[allow(private_bounds)]
    type Source: Source;

    async fn tokenize(&self, source: Self::Source) -> Result<Token, Error>;
}

/// Marker trait for payment sources that can be tokenized.
trait Source: PaymentSource {}
impl Source for BNPL {}
impl Source for BankAccount {}
impl Source for CashVoucher {}
impl Source for CreditCard {}
impl Source for InstantAccount {}
impl Source for SEPAAccount {}
