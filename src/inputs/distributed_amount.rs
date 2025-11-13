use rust_decimal::Decimal;

use super::Recipients;

/// Insecure representation of payment amount with optional distribution.
///
/// ```skip
/// let input = DistributedAmount {
///     total: dec!(100.00),
///     recipients: recipients_map,
/// };
/// ```
pub struct DistributedAmount<'a> {
    pub total: Decimal,
    pub recipients: Recipients<'a>,
}
