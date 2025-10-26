use async_trait::async_trait;

use crate::error::{Error, Result};
use crate::refundable::RefundsSupported;
use crate::traits::gateway::RecoveryCapability;
use crate::traits::{Authorizable, Gateway, TransactionFlow};
use crate::types::{
    Money, TransactionStatus,
    secure::{MerchantReferenceId, TransactionId},
};

/// Optional trait for payment gateways that support completing a two-step flow.
///
/// An adapter should implement this ONLY if it supports the two-step Auth -> Capture model.
/// Gateways that only support Sale/Purchase should NOT implement this trait.
#[async_trait]
pub trait Recoverable
where
    Self: Authorizable,
    Self: Gateway<RecoveryCapability = RecoverySupported>,
{
    /// Confirms and debits the previously authorized funds.
    async fn recover(&self, request: Request) -> Result<Page<Transaction>>;
}

/// Transaction Style: Two-step flow (Authorize + subsequent Capture).
/// Applies to gateways that support delayed capture and full lifecycle management (e.g., Stripe).
pub struct RecoverySupported;
impl RecoveryCapability for RecoverySupported {}

/// Request body for capturing a previously authorized payment.
#[derive(Debug, Clone)]
pub struct Request {
    /// Unique ID provided by the merchant for tracing the transaction.
    pub merchant_reference_id: MerchantReferenceId,
    // TODO: add filters
    pub page: Option<u32>,
}

/// Response body after a successful or failed capture.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// The new transaction ID for the capture operation.
    pub transaction_id: TransactionId,
    /// The canonical status (Should be Captured or Failed).
    pub status: TransactionStatus,
    /// The final amount successfully captured.
    pub amount: Money,
}

pub struct Page<T> {
    pub items: Vec<T>,
    pub next_page: Option<u32>,
}
