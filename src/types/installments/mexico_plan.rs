//! Mexico MSI (Meses Sin Intereses) installment plan.

use crate::Error;
use crate::inputs::installments::MexicoPlan as Input;
use crate::types::InstallmentPlanId;

/// Plan type for Mexico MSI (Meses Sin Intereses) installments.
///
/// Mexico supports only fixed installment counts: 1, 3, 6, 9, 12, 18 months.
/// All MSI plans (except Single) are interest-free for the customer.
#[derive(Clone, Debug, Default)]
pub enum MexicoPlan {
    /// Single payment (no installments).
    #[default]
    Single,
    /// 3 months without interest.
    Three,
    /// 6 months without interest.
    Six,
    /// 9 months without interest.
    Nine,
    /// 12 months without interest.
    Twelve,
    /// 18 months without interest.
    Eighteen,
    /// Gateway-specific plan ID from installments API.
    Id(InstallmentPlanId),
}

impl<'a> TryFrom<Input<'a>> for MexicoPlan {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(match input {
            Input::Single => Self::Single,
            Input::Three => Self::Three,
            Input::Six => Self::Six,
            Input::Nine => Self::Nine,
            Input::Twelve => Self::Twelve,
            Input::Eighteen => Self::Eighteen,
            Input::Id(id) => Self::Id(id.try_into()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_single() {
        let plan = MexicoPlan::default();
        assert!(matches!(plan, MexicoPlan::Single));
    }

    #[test]
    fn constructed_from_input() {
        let plan = MexicoPlan::try_from(Input::Six).unwrap();
        assert!(matches!(plan, MexicoPlan::Six));
    }

    #[test]
    fn constructed_from_input_id() {
        let plan = MexicoPlan::try_from(Input::Id("plan_123")).unwrap();
        assert!(matches!(plan, MexicoPlan::Id(_)));
    }

    #[test]
    fn rejects_empty_id() {
        let result = MexicoPlan::try_from(Input::Id(""));
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
