use std::convert::TryFrom;

use crate::{Error, types::ReasonText};

/// Semantic reason for reversing a payment transaction.
///
/// Reversal undoes the original settled transaction without creating a new transaction.
/// These are merchant-level semantic reasons, not gateway-specific codes.
#[derive(Clone, Debug)]
pub enum ReversalReason {
    /// Duplicate transaction was processed
    Duplicate,

    /// The transaction amount was incorrect
    IncorrectAmount,

    /// Wrong account or customer was charged
    IncorrectAccount,

    /// Suspected fraudulent transaction
    Fraud,

    /// Merchant processing error occurred
    ProcessingError,

    /// Another reason not covered by standard categories
    Other(ReasonText),
}

impl<'a> TryFrom<&crate::ReversalReason<'a>> for ReversalReason {
    type Error = Error;

    fn try_from(input: &crate::ReversalReason<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::ReversalReason::Duplicate => Ok(Self::Duplicate),
            crate::ReversalReason::IncorrectAmount => Ok(Self::IncorrectAmount),
            crate::ReversalReason::IncorrectAccount => Ok(Self::IncorrectAccount),
            crate::ReversalReason::Fraud => Ok(Self::Fraud),
            crate::ReversalReason::ProcessingError => Ok(Self::ProcessingError),
            crate::ReversalReason::Other(text) => Ok(Self::Other(ReasonText::try_from(*text)?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod construction {
        use super::*;

        #[test]
        fn creates_unit_variants() {
            let reasons = vec![
                ReversalReason::Duplicate,
                ReversalReason::IncorrectAmount,
                ReversalReason::IncorrectAccount,
                ReversalReason::Fraud,
                ReversalReason::ProcessingError,
            ];

            for reason in reasons {
                let debug = format!("{:?}", reason);
                assert!(!debug.is_empty());
            }
        }

        #[test]
        fn creates_other_variant() {
            let text = ReasonText::try_from("Custom reason").unwrap();
            let reason = ReversalReason::Other(text);

            assert!(matches!(reason, ReversalReason::Other(_)));
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn converts_unit_variants() {
            let inputs = vec![
                crate::ReversalReason::Duplicate,
                crate::ReversalReason::IncorrectAmount,
                crate::ReversalReason::IncorrectAccount,
                crate::ReversalReason::Fraud,
                crate::ReversalReason::ProcessingError,
            ];

            for input in inputs {
                let result = ReversalReason::try_from(&input);
                assert!(result.is_ok());
            }
        }

        #[test]
        fn converts_other_with_valid_text() {
            let input = crate::ReversalReason::Other("Custom reason");
            let result = ReversalReason::try_from(&input);

            assert!(result.is_ok());
        }

        #[test]
        fn rejects_other_with_empty_text() {
            let input = crate::ReversalReason::Other("");
            let result = ReversalReason::try_from(&input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_other_with_too_long_text() {
            let text = "a".repeat(256);
            let input = crate::ReversalReason::Other(&text);
            let result = ReversalReason::try_from(&input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
