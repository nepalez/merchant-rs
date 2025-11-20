//! Japan extended installment plan.

use crate::internal::Validated;
use crate::types::InstallmentPlanId;
use crate::{Error, installments};

/// Plan type for Japan extended installments.
///
/// Japan supports regular installments (2-99), revolving credit,
/// and bonus payment plans (July and December).
#[derive(Clone, Debug, Default)]
pub enum ExtendedPlan {
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

impl Validated for ExtendedPlan {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::Regular(0) | Self::Regular(1) => Err(Error::InvalidInput(
                "Regular installment count must be at least 2".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

impl<'a> TryFrom<installments::ExtendedPlan<'a>> for ExtendedPlan {
    type Error = Error;

    fn try_from(input: installments::ExtendedPlan<'a>) -> Result<Self, Self::Error> {
        match input {
            installments::ExtendedPlan::Single => Ok(Self::Single),
            installments::ExtendedPlan::Regular(n) => Self::Regular(n).validate(),
            installments::ExtendedPlan::Revolving => Ok(Self::Revolving),
            installments::ExtendedPlan::Bonus => Ok(Self::Bonus),
            installments::ExtendedPlan::Id(id) => Ok(Self::Id(id.try_into()?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_single() {
        let plan = ExtendedPlan::default();
        assert!(matches!(plan, ExtendedPlan::Single));
    }

    #[test]
    fn constructed_from_input_single() {
        let plan = ExtendedPlan::try_from(installments::ExtendedPlan::Single).unwrap();
        assert!(matches!(plan, ExtendedPlan::Single));
    }

    #[test]
    fn constructed_from_input_regular() {
        let plan = ExtendedPlan::try_from(installments::ExtendedPlan::Regular(6)).unwrap();
        assert!(matches!(plan, ExtendedPlan::Regular(6)));
    }

    #[test]
    fn constructed_from_input_revolving() {
        let plan = ExtendedPlan::try_from(installments::ExtendedPlan::Revolving).unwrap();
        assert!(matches!(plan, ExtendedPlan::Revolving));
    }

    #[test]
    fn constructed_from_input_bonus() {
        let plan = ExtendedPlan::try_from(installments::ExtendedPlan::Bonus).unwrap();
        assert!(matches!(plan, ExtendedPlan::Bonus));
    }

    #[test]
    fn rejects_regular_zero() {
        let result = ExtendedPlan::try_from(installments::ExtendedPlan::Regular(0));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_regular_one() {
        let result = ExtendedPlan::try_from(installments::ExtendedPlan::Regular(1));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_empty_id() {
        let result = ExtendedPlan::try_from(installments::ExtendedPlan::Id(""));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
