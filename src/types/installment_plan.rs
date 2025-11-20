//! Fixed the installment plan for universal use.

use crate::Error;
use crate::internal::Validated;
use crate::types::InstallmentPlanId;

/// Plan type for fixed installments.
///
/// Used by gateways that support a simple installment count or plan ID.
#[derive(Clone, Debug, Default)]
pub enum InstallmentPlan {
    /// Single payment (no installments).
    #[default]
    Single,
    /// Installments with a specified count (2-99).
    Regular(u8),
    /// Gateway-specific plan ID from installments API.
    Id(InstallmentPlanId),
}

impl Validated for InstallmentPlan {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::Regular(0) | Self::Regular(1) => Err(Error::InvalidInput(
                "Installment count must be at least 2".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

impl<'a> TryFrom<crate::InstallmentPlan<'a>> for InstallmentPlan {
    type Error = Error;

    fn try_from(input: crate::InstallmentPlan<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::InstallmentPlan::Single => Ok(Self::Single),
            crate::InstallmentPlan::Regular(n) => Self::Regular(n).validate(),
            crate::InstallmentPlan::Id(id) => Ok(Self::Id(id.try_into()?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_single() {
        let plan = InstallmentPlan::default();
        assert!(matches!(plan, InstallmentPlan::Single));
    }

    #[test]
    fn constructed_from_input_single() {
        let plan = InstallmentPlan::try_from(crate::InstallmentPlan::Single).unwrap();
        assert!(matches!(plan, InstallmentPlan::Single));
    }

    #[test]
    fn constructed_from_input_count() {
        let plan = InstallmentPlan::try_from(crate::InstallmentPlan::Regular(6)).unwrap();
        assert!(matches!(plan, InstallmentPlan::Regular(6)));
    }

    #[test]
    fn constructed_from_input_id() {
        let plan = InstallmentPlan::try_from(crate::InstallmentPlan::Id("INS54434")).unwrap();
        assert!(matches!(plan, InstallmentPlan::Id(_)));
    }

    #[test]
    fn rejects_count_zero() {
        let result = InstallmentPlan::try_from(crate::InstallmentPlan::Regular(0));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_count_one() {
        let result = InstallmentPlan::try_from(crate::InstallmentPlan::Regular(1));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_empty_id() {
        let result = InstallmentPlan::try_from(crate::InstallmentPlan::Id(""));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
