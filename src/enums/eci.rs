use strum_macros::{AsRefStr, Display};

/// Electronic Commerce Indicator from 3D Secure authentication
///
/// ECI indicates the result of the 3D Secure authentication attempt and is used
/// by the card networks to determine liability shift for chargebacks.
///
/// # Data Protection
/// This is a public value, neither secret nor even PII.
/// ECI values are standardized classifiers requiring no security protection.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum ECI {
    /// Full 3D Secure authentication completed successfully.
    ///
    /// Visa/AMEX/JCB/Discover: 05, Mastercard: 02
    FullyAuthenticated,
    /// Authentication was attempted, but the issuer was not enrolled or unavailable.
    ///
    /// Visa/AMEX/JCB/Discover: 06, Mastercard: 01
    AttemptedAuth,
    /// No authentication performed.
    ///
    /// Visa/AMEX/JCB/Discover: 07, Mastercard: 00
    NoAuthentication,
}
