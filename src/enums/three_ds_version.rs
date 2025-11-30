use strum_macros::{AsRefStr, Display};

/// 3D Secure protocol version
///
/// Indicates which version of the 3D Secure protocol was used for authentication.
/// Different versions have different capabilities and security features.
///
/// # Data Protection
/// This is a public value, neither secret nor even PII.
/// Protocol versions are standardized identifiers requiring no security protection.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum ThreeDSVersion {
    /// 3D Secure 2.1.0
    V2_1_0,
    /// 3D Secure 2.2.0
    V2_2_0,
}
