use rust_decimal::Decimal;

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
pub struct CaptureAuthorized;

// Conversions between NotChanged and () for API uniformity

impl From<()> for CaptureAuthorized {
    fn from(_: ()) -> Self {
        CaptureAuthorized
    }
}

impl From<CaptureAuthorized> for () {
    fn from(_: CaptureAuthorized) -> Self {}
}

// Conversions from NotChanged to Option types for API uniformity

impl From<CaptureAuthorized> for Option<Decimal> {
    fn from(_: CaptureAuthorized) -> Self {
        None
    }
}

impl From<CaptureAuthorized> for Option<Recipients> {
    fn from(_: CaptureAuthorized) -> Self {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_from_unit() {
        let not_changed: CaptureAuthorized = ().into();
        let _: () = not_changed.into();
    }

    #[test]
    fn converts_to_option_decimal() {
        let not_changed = CaptureAuthorized;
        let option: Option<Decimal> = not_changed.into();
        assert!(option.is_none());
    }

    #[test]
    fn converts_to_option_recipients() {
        let not_changed = CaptureAuthorized;
        let option: Option<Recipients> = not_changed.into();
        assert!(option.is_none());
    }
}
