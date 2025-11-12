use std::collections::HashMap;

use super::DistributedValue;

/// Insecure representation of payment recipients.
///
/// ```skip
/// let mut recipients = HashMap::new();
/// recipients.insert("merchant_a", DistributedValue::Amount(dec!(50.00)));
/// recipients.insert("merchant_b", DistributedValue::Percent(dec!(10.0)));
/// ```
pub type Recipients<'a> = HashMap<&'a str, DistributedValue>;
