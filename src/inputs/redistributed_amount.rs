use rust_decimal::Decimal;

use super::Recipients;

/// Insecure representation of payment redistribution.
///
/// ```skip
/// let input = RedistributedAmount {
///     total: Some(dec!(50.00)),
///     recipients: None,
/// };
/// ```
pub struct RedistributedAmount<'a> {
    pub total: Option<Decimal>,
    pub recipients: Option<Recipients<'a>>,
}
