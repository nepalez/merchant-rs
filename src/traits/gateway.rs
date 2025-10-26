/// The base trait defining a payment gateway adapter's core identity and capabilities.
/// This trait is the minimal requirement for any adapter.
#[allow(private_bounds)]
pub trait Gateway {
    /// Associated type defining the fundamental transaction flow style of the adapter.
    type TransactionFlow: TransactionFlow;

    /// Associated type defining the adapter's capability of
    /// transaction recovery.
    type RecoveryCapability: RecoveryCapability;

    /// Associated type defining the adapter's capability of
    /// transaction cancellation.
    type CancellationCapability: CancellationCapability;

    /// Associated type defining the adapter's capability of
    /// transaction refunds.
    type RefundsCapability: RefundsCapability;

    /// Associated type defining the adapter's capability of
    /// API-based card tokenization.
    type TokenizationCapability: TokenizationCapability;

    /// Returns a unique identifier for the gateway adapter
    /// (e.g., "stripe", "adyen").
    fn id(&self) -> &str;
}

/// The private seal trait for the associated type `Gateway::TransactionFlow`.
pub(super) trait TransactionFlow {}

/// Transaction Style: Sale in a single step (Authorize = Captured).
/// This is the default style, applicable to gateways
/// that only support immediate sale flows (e.g., Adyen).
pub struct OneStepFlow;
impl TransactionFlow for OneStepFlow {}

/// The seal trait for the associated type `Gateway::RecoveryCapability`.
pub(super) trait RecoveryCapability {}

/// Indicates that the adapter does not support transaction recovery (the default).
/// This style is applicable to gateways that do not allow
/// getting a transaction's status by idempotency key.
pub struct RecoveryDisabled;
impl RecoveryCapability for RecoveryDisabled {}

/// The seal trait for the associated type `Gateway::CancellationCapability`.
pub(super) trait CancellationCapability {}

/// Indicates that the adapter does not support cancelling transactions.
/// This style is applicable to gateways that do not allow
/// programmatic cancellation (e.g., some crypto payment processors).
pub struct CancellationDisabled;
impl CancellationCapability for CancellationDisabled {}

/// The seal trait for the associated type `Gateway::RefundsCapability`.
pub(super) trait RefundsCapability {}

/// Indicates that the adapter does not support refunding transactions (the default).
/// This is the default style, applicable to gateways that do not allow
/// programmatic refunds (e.g., some crypto payment processors).
pub struct RefundsDisabled;
impl RefundsCapability for RefundsDisabled {}

/// The seal trait for the associated type `Gateway::TokenizationSupport`.
pub(super) trait TokenizationCapability {}

/// Indicates the gateway does NOT support API-based token creation (the default).
/// This is the default style, applicable to gateways
/// that require client-side tokenization or do not offer the service
/// (e.g., some legacy systems).
pub struct TokenizationDisabled;
impl TokenizationCapability for TokenizationDisabled {}
