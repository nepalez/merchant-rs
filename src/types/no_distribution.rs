use crate::types::Recipients;

/// Marker type indicating "no change" in payment distribution.
///
/// Used in flows where redistribution is not supported or not desired.
/// Converts to `None` for both amount and recipients.
///
/// # Examples
///
/// ```skip
/// use merchant_rs::types::NotChanged;
///
/// // No change in refund distribution
/// gateway.refund(transaction_id, NotChanged).await?;
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct NoDistribution;

// Conversions between NotChanged and () for API uniformity

impl From<()> for NoDistribution {
    fn from(_: ()) -> Self {
        NoDistribution
    }
}

impl From<NoDistribution> for () {
    fn from(_: NoDistribution) -> Self {}
}

// Conversions from NotChanged to Option types for API uniformity

impl From<NoDistribution> for Option<Recipients> {
    fn from(_: NoDistribution) -> Self {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_from_unit() {
        let not_changed: NoDistribution = ().into();
        let _: () = not_changed.into();
    }

    #[test]
    fn converts_to_option_recipients() {
        let not_changed = NoDistribution;
        let option: Option<Recipients> = not_changed.into();
        assert!(option.is_none());
    }
}
