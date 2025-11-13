use std::convert::TryFrom;

use crate::Error;
use crate::types::{InternalPaymentMethod, PaymentMethod, Token, VaultPaymentMethod};

/// Vault Token Payment Method
///
/// ## Overview
///
/// Vault is a self-contained payment method that uses pre-stored tokenized credentials
/// from a secure vault service. Unlike tokenization within a specific payment method
/// (e.g., `StoredCard` with tokenized credentials), Vault represents a payment method
/// where the token itself is the entire payment instrument.
///
/// The token returned from a vault service encapsulates all necessary payment information,
/// eliminating the need to know the underlying payment method type. This abstraction
/// allows merchants to process payments without handling sensitive payment details.
///
/// ## When to Use
///
/// - **Vault-based payment processing**: When using third-party vault services (e.g., Spreedly, Basis Theory)
///
/// ## Authentication Model
///
/// > Authentication is **delegated to the vault service** that issued the token!
///
/// ### Token Usage Flow
///
/// 1. **Token generation**: Payment method stored in vault, token returned
/// 2. **Merchant stores token**: Token stored in a merchant database (safe to store)
/// 3. **Payment request**: Token passed to gateway via `Vault` payment method
/// 4. **Gateway detokenizes**: Gateway exchanges token for actual payment credentials via vault API
/// 5. **Payment processing**: Gateway processes payment with detokenized credentials
/// 6. **Token remains valid**: Token can be reused for future transactions (unless revoked)
///
/// ### Security Model
///
/// - **Vault service security**: Vault provider secures actual payment credentials
/// - **Token limitations**: Tokens may be restricted by domain, IP, or usage count
/// - **No sensitive data exposure**: Merchant never sees underlying payment details
/// - **Token revocation**: Vault service can invalidate tokens independently
/// - **Audit trail**: Vault services typically log all token usage
///
/// ## Standards
///
/// Vault services typically comply with:
/// - **[PCI DSS](https://www.pcisecuritystandards.org/)**: Vault providers are PCI Level 1 compliant
/// - **[ISO 27001](https://www.iso.org/isoiec-27001-information-security.html)**: Information security management
/// - **[SOC 2 Type II](https://us.aicpa.org/interestareas/frc/assuranceadvisoryservices/aicpasoc2report)**: Security and availability controls
///
/// ## Example Vault Services
///
/// - **Spreedly**: Multi-gateway vault and tokenization
/// - **Basis Theory**: Token vault and privacy service
/// - **VGS (Very Good Security)**: Data protection and tokenization
/// - **Gateway native vaults**: Stripe, Braintree, Adyen customer vault
///
/// ## Security Considerations
///
/// ### Token Protection
/// - **Tokens are sensitive**: While not payment data, they enable transactions
/// - **Treat as authentication credentials**: Protect with encryption at rest
/// - **Zeroization on a drop**: Tokens are automatically zeroed in memory after use
/// - **No logging**: Tokens are masked in debug output
///
/// ### PCI Compliance
/// - **Reduced scope**: Merchant doesn't handle raw payment data
/// - **Vault provider responsibility**: Vault service is PCI DSS compliant
/// - **Token storage allowed**: Tokens are not card data per PCI DSS
/// - **Secure transmission**: Use TLS for token transmission
///
/// ### Fraud Prevention
/// - **Token restrictions**: Limit token usage to specific IPs, domains, or amounts
/// - **Velocity monitoring**: Track token usage frequency
/// - **Token expiration**: Set expiration dates on tokens where supported
/// - **Revocation capabilities**: Ability to invalidate compromised tokens
///
/// ### Compliance
/// - **Data residency**: Vault service may need to comply with regional data laws
/// - **Consent management**: Customer consent required for storing payment methods
/// - **Access controls**: Restrict who can create and use vault tokens
#[derive(Clone, Debug)]
pub struct Vault {
    /// Vault token representing a stored payment method
    pub token: Token,
}

// Marker implementations

impl PaymentMethod for Vault {}
impl InternalPaymentMethod for Vault {}
impl VaultPaymentMethod for Vault {}

impl<'a> TryFrom<&'a str> for Vault {
    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            token: input.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;

    #[test]
    fn constructed_from_valid_input() {
        let vault = Vault::try_from("tok_1234567890abcdef").unwrap();

        unsafe {
            assert_eq!(vault.token.as_ref(), "tok_1234567890abcdef");
        }
    }

    #[test]
    fn rejects_invalid_token() {
        let result = Vault::try_from("short_token"); // Less than 16 characters

        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
