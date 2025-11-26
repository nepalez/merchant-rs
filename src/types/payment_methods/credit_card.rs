use std::convert::TryFrom;

use crate::Error;
use crate::types::{CVV, CardExpiry, CardHolderName, InternalPaymentMethod, PrimaryAccountNumber};

/// Credit or Debit Card
///
/// ## Overview
///
/// Traditional payment card transaction where the customer provides card details for authorization.
/// Supports both consumer cards (credit/debit)
/// and commercial cards across global and regional schemes.
/// The most universal payment method, accepted worldwide
/// with established fraud protection mechanisms.
///
/// ## When to Use
///
/// - **E-commerce**: Standard online card-not-present transactions
/// - **Point of Sale**: When processing physical card data
/// - **Recurring payments**: For stored credentials (MIT - Merchant Initiated Transactions)
/// - **One-time payments**: General-purpose consumer and business payments
/// - **International payments**: Cross-border transactions
///
/// ## Authentication Model
///
/// > Authentication occurs **in the authorization request** through multiple layers!
///
/// ### Primary Authentication: CVV/CVC
/// The Card Verification Value (CVV/CVC/CID) is the primary authentication credential
/// for card-not-present transactions:
/// - **Visa**: CVV2 (3 digits on back)
/// - **Mastercard**: CVC2 (3 digits on back)
/// - **American Express**: CID (4 digits on the front)
///
/// CVV proves the customer possesses the physical card at the time of transaction.
/// **Cannot be stored** after authorization per PCI DSS Requirement 3.2.
///
/// ### Secondary Authentication: 3D Secure (Optional)
/// Additional authentication layer via bank redirect:
/// - **3DS 1.0**: Popup window, password-based
/// - **3DS 2.0**: Frictionless flow, biometric support, mobile-optimized
/// - **Required**: PSD2 Strong Customer Authentication (SCA) in EEA
/// - **Handled separately**: Via 3DS extension crate, not core authorization flow
///
/// ### Tertiary: Address Verification System (AVS) (Optional)
/// Fraud prevention through address matching:
/// - Gateway compares billing address with bank records
/// - Returns match codes (full match, partial, no match)
/// - Primarily used in the US, Canada, the UK
/// - Merchant decides acceptance based on AVS response
///
/// ## Standards
///
/// - **[ISO/IEC 7812](https://www.iso.org/standard/70484.html)**: Identification cards — Numbering system (PAN structure)
/// - **[ISO/IEC 7813](https://www.iso.org/standard/43317.html)**: Identification cards — Financial transaction cards
/// - **[ISO/IEC 7816](https://www.iso.org/standard/77180.html)**: Integrated circuit cards (chip cards)
/// - **[EMV](https://www.emvco.com/emv-technologies/emv-contact-chip/)**: Europay, Mastercard, and Visa chip card specifications
/// - **[PCI DSS](https://www.pcisecuritystandards.org/document_library/)**: Payment Card Industry Data Security Standard
/// - **[PSD2](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32015L2366)**: Payment Services Directive 2 (Strong Customer Authentication for EEA)
/// - **RBI Guidelines**: Reserve Bank of India card payment regulations
///
/// ## Card Schemes
///
/// ### Global Schemes
/// - **Visa**: 4xxx, most widely accepted globally
/// - **Mastercard**: 51-55xx, 2221-2720, global acceptance
/// - **American Express**: 34xx, 37xx, premium/corporate focus
/// - **Discover**: 6011, 622126–622925, 644–649, 65, primarily US
/// - **JCB**: 3528–3589, Japan-based, growing international
/// - **Diners Club**: 36xx, 38xx, 30xx-305x, travel/entertainment
/// - **UnionPay**: 62xx, China-based, rapidly expanding globally
///
/// ### Regional Schemes
/// - **RuPay** (India): Domestic debit/credit card scheme
/// - **Elo** (Brazil): Co-branded with major Brazilian banks
/// - **Mir** (Russia): National payment system
/// - **Troy** (Turkey): Turkish domestic scheme
/// - **Cartes Bancaires** (France): Interbank cooperative
/// - **Interac** (Canada): Debit card network
/// - **BC Card** (South Korea): Domestic card scheme
/// - **STAR** (Japan): Domestic debit network
///
/// ## Security Considerations
///
/// ### PCI DSS Compliance Levels
/// - **Level 1**: 6M+ transactions/year, annual onsite audit
/// - **Level 2**: 1M-6M transactions/year, annual self-assessment
/// - **Level 3**: 20K-1M commerce transactions/year, annual self-assessment
/// - **Level 4**: <20K e-commerce or <1M total, annual self-assessment
///
/// ### Minimizing PCI Scope
/// - **Client-side tokenization**: Use Stripe.js, Braintree Drop in to avoid card data touching the server
/// - **Gateway-hosted pages**: iFrame or redirect to gateway for card entry
/// - **Network tokens**: Use EMV tokens from Visa Token Service, Mastercard MDES
/// - **Point-to-point encryption**: P2PE for physical terminals
///
/// ### CVV Handling
/// **CRITICAL**: CVV must never be stored after authorization:
/// - ❌ Do not log CVV
/// - ❌ Do not store in a database
/// - ❌ Do not write to files
/// - ❌ Do not include in error messages
/// - ✅ Pass directly to the gateway
/// - ✅ Protected by `ZeroizeOnDrop` (automatic memory zeroization)
/// - ✅ Memory zeroed immediately after use
///
/// ### Fraud Prevention
/// - **CVV validation**: Verify CVV is present and valid
/// - **AVS checks**: Match billing address with bank records
/// - **Velocity rules**: Limit transactions per card/IP/customer
/// - **3D Secure**: Shift liability to an issuing bank
/// - **Device fingerprinting**: Detect suspicious devices
/// - **Geolocation**: Flag transactions from unusual locations
#[derive(Clone, Debug)]
pub struct CreditCard {
    pub(crate) cvv: CVV,
    pub(crate) number: PrimaryAccountNumber,
    pub(crate) card_expiry: CardExpiry,
    pub(crate) holder_name: CardHolderName,
}

impl CreditCard {
    /// Card Verification Value (CVV/CVC/CID)
    #[inline]
    pub fn cvv(&self) -> &CVV {
        &self.cvv
    }

    /// Primary Account Number (PAN)
    #[inline]
    pub fn number(&self) -> &PrimaryAccountNumber {
        &self.number
    }

    /// Card expiration date (month and year)
    #[inline]
    pub fn card_expiry(&self) -> &CardExpiry {
        &self.card_expiry
    }

    /// Cardholder name as embossed on the card
    #[inline]
    pub fn holder_name(&self) -> &CardHolderName {
        &self.holder_name
    }
}

// Marker implementations

impl InternalPaymentMethod for CreditCard {}

impl<'a> TryFrom<crate::CreditCard<'a>> for CreditCard {
    type Error = Error;

    fn try_from(value: crate::CreditCard<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            cvv: value.cvv.try_into()?,
            number: value.number.try_into()?,
            card_expiry: value.card_expiry.try_into()?,
            holder_name: value.holder_name.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;

    fn valid_input() -> crate::CreditCard<'static> {
        inputs::CreditCard {
            cvv: " 123 \n\t",
            number: " 4532-0151-1283-0366 \n\t",
            card_expiry: inputs::CardExpiry {
                month: 12,
                year: 2030,
            },
            holder_name: " john doe \n\t",
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let card = CreditCard::try_from(input).unwrap();

        unsafe {
            assert_eq!(card.cvv.as_ref(), "123");
            assert_eq!(card.number.as_ref(), "4532015112830366");
            assert_eq!(card.card_expiry.month(), 12);
            assert_eq!(card.card_expiry.year(), 2030);
            assert_eq!(card.holder_name.as_ref(), "JOHN DOE");
        }
    }

    #[test]
    fn rejects_invalid_cvv() {
        let mut input = valid_input();
        input.cvv = "12";

        let result = CreditCard::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_pan() {
        let mut input = valid_input();
        input.number = "1234567890123";

        let result = CreditCard::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_card_expiry() {
        let mut input = valid_input();
        input.card_expiry.month = 13;

        let result = CreditCard::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_holder_name() {
        let mut input = valid_input();
        input.holder_name = "X";

        let result = CreditCard::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
