//! Marker type for payments without installments.

/// Marker type indicating no installment plan.
///
/// Used by gateways that don't support installments.
#[derive(Clone, Debug, Default)]
pub struct NoInstallments;
