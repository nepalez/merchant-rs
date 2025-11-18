//! Fixed the installment plan for universal use.

use crate::Error;
use crate::inputs::installments::FixedPlan as Input;
use crate::internal::Validated;
use crate::types::InstallmentPlanId;

/// Plan type for fixed installments.
///
/// Used by gateways that support a simple installment count or plan ID.
#[derive(Clone, Debug, Default)]
pub enum FixedPlan {
    /// Single payment (no installments).
    #[default]
    Single,
    /// Installments with a specified count (2-99).
    Count(u8),
    /// Gateway-specific plan ID from installments API.
    Id(InstallmentPlanId),
}

impl Validated for FixedPlan {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::Count(0) | Self::Count(1) => Err(Error::InvalidInput(
                "Installment count must be at least 2".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

impl<'a> TryFrom<Input<'a>> for FixedPlan {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        match input {
            Input::Single => Ok(Self::Single),
            Input::Count(n) => Self::Count(n).validate(),
            Input::Id(id) => Ok(Self::Id(id.try_into()?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_single() {
        let plan = FixedPlan::default();
        assert!(matches!(plan, FixedPlan::Single));
    }

    #[test]
    fn constructed_from_input_single() {
        let plan = FixedPlan::try_from(Input::Single).unwrap();
        assert!(matches!(plan, FixedPlan::Single));
    }

    #[test]
    fn constructed_from_input_count() {
        let plan = FixedPlan::try_from(Input::Count(6)).unwrap();
        assert!(matches!(plan, FixedPlan::Count(6)));
    }

    #[test]
    fn constructed_from_input_id() {
        let plan = FixedPlan::try_from(Input::Id("INS54434")).unwrap();
        assert!(matches!(plan, FixedPlan::Id(_)));
    }

    #[test]
    fn rejects_count_zero() {
        let result = FixedPlan::try_from(Input::Count(0));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_count_one() {
        let result = FixedPlan::try_from(Input::Count(1));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_empty_id() {
        let result = FixedPlan::try_from(Input::Id(""));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
