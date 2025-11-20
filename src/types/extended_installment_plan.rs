//! Japan extended installment plan.

use crate::Error;
use crate::internal::Validated;
use crate::types::InstallmentPlanId;

/// Plan type for Japan extended installments.
///
/// Japan supports regular installments (2-99), revolving credit,
/// and bonus payment plans (July and December).
#[derive(Clone, Debug, Default)]
pub enum ExtendedInstallmentPlan {
    /// Single payment (no installments).
    #[default]
    Single,
    /// Regular installments with a specified count (2-99).
    Regular(u8),
    /// Revolving credit plan.
    Revolving,
    /// Bonus payment plan (July and December).
    Bonus,
    /// Gateway-specific plan ID from installments API.
    Id(InstallmentPlanId),
}

impl Validated for ExtendedInstallmentPlan {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::Regular(0) | Self::Regular(1) => Err(Error::InvalidInput(
                "Regular installment count must be at least 2".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

impl<'a> TryFrom<crate::ExtendedInstallmentPlan<'a>> for ExtendedInstallmentPlan {
    type Error = Error;

    fn try_from(input: crate::ExtendedInstallmentPlan<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::ExtendedInstallmentPlan::Single => Ok(Self::Single),
            crate::ExtendedInstallmentPlan::Regular(n) => Self::Regular(n).validate(),
            crate::ExtendedInstallmentPlan::Revolving => Ok(Self::Revolving),
            crate::ExtendedInstallmentPlan::Bonus => Ok(Self::Bonus),
            crate::ExtendedInstallmentPlan::Id(id) => Ok(Self::Id(id.try_into()?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_single() {
        let plan = ExtendedInstallmentPlan::default();
        assert!(matches!(plan, ExtendedInstallmentPlan::Single));
    }

    #[test]
    fn constructed_from_input_single() {
        let plan =
            ExtendedInstallmentPlan::try_from(crate::ExtendedInstallmentPlan::Single).unwrap();
        assert!(matches!(plan, ExtendedInstallmentPlan::Single));
    }

    #[test]
    fn constructed_from_input_regular() {
        let plan =
            ExtendedInstallmentPlan::try_from(crate::ExtendedInstallmentPlan::Regular(6)).unwrap();
        assert!(matches!(plan, ExtendedInstallmentPlan::Regular(6)));
    }

    #[test]
    fn constructed_from_input_revolving() {
        let plan =
            ExtendedInstallmentPlan::try_from(crate::ExtendedInstallmentPlan::Revolving).unwrap();
        assert!(matches!(plan, ExtendedInstallmentPlan::Revolving));
    }

    #[test]
    fn constructed_from_input_bonus() {
        let plan =
            ExtendedInstallmentPlan::try_from(crate::ExtendedInstallmentPlan::Bonus).unwrap();
        assert!(matches!(plan, ExtendedInstallmentPlan::Bonus));
    }

    #[test]
    fn rejects_regular_zero() {
        let result = ExtendedInstallmentPlan::try_from(crate::ExtendedInstallmentPlan::Regular(0));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_regular_one() {
        let result = ExtendedInstallmentPlan::try_from(crate::ExtendedInstallmentPlan::Regular(1));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_empty_id() {
        let result = ExtendedInstallmentPlan::try_from(crate::ExtendedInstallmentPlan::Id(""));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
