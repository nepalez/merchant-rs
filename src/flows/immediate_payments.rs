use async_trait::async_trait;

use crate::Error;
use crate::internal::PaymentSource;
use crate::types::{
    BankAccount, CreditCard, InstantAccount, Payment, SEPAAccount, Token, Transaction,
};

/// Optional trait for payment gateways that support completing a one-step flow,
/// without the necessity to capture them later.
#[async_trait]
pub trait ImmediatePayments {
    #[allow(private_bounds)]
    type Source: Source;

    /// Immediately charge the payment.
    async fn charge(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>;
}

/// Marker trait for payment sources that can be charged immediately.
trait Source: PaymentSource {}
impl Source for CreditCard {}
impl Source for BankAccount {}
impl Source for InstantAccount {}
impl Source for SEPAAccount {}
impl Source for Token {}
