/// Semantic reason for reversing a payment transaction (input variant).
#[derive(Clone, Debug)]
pub enum ReversalReason<'a> {
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
    Other(&'a str),
}
