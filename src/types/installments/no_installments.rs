//! Marker type for payments without installments.

use super::Installments;

/// Marker type indicating no installment plan.
///
/// Used by gateways that don't support installments.
#[derive(Clone, Debug, Default)]
pub struct NoInstallments;

impl Installments for NoInstallments {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_creates_instance() {
        let _no_installments = NoInstallments::default();
    }
}
