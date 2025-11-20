use std::convert::TryFrom;

use crate::error::Error;
use crate::types::{ExternalPaymentMethod, Metadata, PaymentMethod, PhoneNumber};

/// Mobile Carrier Billing Payment
///
/// ## Overview
///
/// Payment is charged directly to the customer's mobile phone bill or deducted from the prepaid balance.
/// Customer authorizes the charge via SMS PIN or mobile app confirmation.
/// Charge appears on the next phone bill (postpaid) or is immediately deducted from balance (prepaid).
/// No bank account or credit card is required — the mobile carrier acts as the payment intermediary
/// and assumes fraud risk.
///
/// ## When to Use
///
/// - **Mobile-first markets**: Regions where mobile phone penetration exceeds banking access
/// - **Digital content purchases**: Games, apps, music, videos, subscriptions
/// - **Micropayments**: Small transactions where card fees would be prohibitive ($0.50-$50 typical range)
/// - **Unbanked/underbanked users**: Customers without access to traditional payment methods
/// - **Youth market**: Users too young for credit cards
/// - **Emerging markets**: Southeast Asia, Africa, Latin America, Middle East
///
/// ## Authentication Model
///
/// > Authentication occurs **via carrier SMS challenge-response**, not in authorization request!
///
/// ### SMS Authentication Flow Steps
///
/// 1. **Merchant initiates**: Calls `authorize()` with the customer's phone number
/// 2. **Gateway routes to carrier**: Identifies carrier from phone number, initiates DCB request
/// 3. **Carrier sends PIN**: Customer receives SMS with a one-time PIN code
/// 4. **Customer enters PIN**: Types PIN into merchant's checkout or responds to SMS
/// 5. **Carrier validates PIN**: Confirms PIN matches and customer has sufficient balance/credit limit
/// 6. **Carrier authorizes charge**: Approves transaction, adds to bill or deducts from balance
/// 7. **Gateway confirms**: Returns authorization response to merchant
/// 8. **Settlement**: Carrier collects payment from the customer, remits to merchant (typically 30–60 days)
///
/// ### Authorization Request Content
///
/// The authorization request contains **only the phone number as the primary payment identifier**.
/// Authentication happens through the carrier's SMS challenge system, not in the authorization request.
///
/// ## Standards
///
/// - **[GSMA Mobile Connect](https://www.gsma.com/identity/mobile-connect)**: Identity and authentication standard
/// - **GSMA Carrier Billing**: Best practices and technical specifications
/// - **3GPP**: Telecommunications standards (TS 24.008 for SMS)
/// - **MEF (Mobile Ecosystem Forum)**: Industry best practices for carrier billing
///
/// ## Example Systems
///
/// ### Global Aggregators
/// - **Boku**: 200+ carriers in 60+ countries, major DCB aggregator
/// - **Fortumo**: Strong in Europe, Asia, Latin America
/// - **Centili**: Emerging markets focus, 80+ carriers
/// - **Digital Turbine (Trialpay)**: Mobile monetization platform
/// - **Docomo Digital**: Japan and international markets
///
/// ### Direct Carrier Integrations
/// - **Verizon Wireless** (US): Premium SMS and carrier billing
/// - **AT&T** (US): Direct carrier billing for content
/// - **Vodafone** (Europe/Africa/Asia): Multi-market presence
/// - **MTN** (Africa): Major African mobile operator
/// - **Telkomsel** (Indonesia): Largest Indonesian operator
///
/// ## Security Considerations
///
/// ### Phone Number as Identity
/// - Phone number is the primary payment identifier
/// - No credit card or bank account data collected
/// - SIM swap attacks are a risk (attacker takes over phone number)
/// - Carrier validates account ownership before authorizing
///
/// ### SMS Security
/// - PIN-based authentication via SMS
/// - SMS interception is possible (SS7 attacks, SIM swapping)
/// - PINs are one-time use and time-limited
/// - Some carriers use mobile app confirmation instead of SMS
///
/// ### Fraud Prevention
/// - Velocity limits on transactions per phone number
/// - Carrier performs spending limit checks
/// - Age verification for restricted content
/// - Device fingerprinting to detect suspicious patterns
/// - Monitor for stolen phones being used for fraud
///
/// ### Compliance
/// - **Age restrictions**: Carriers enforce minimum age (typically 18+)
/// - **Content restrictions**: Adult content, gambling often prohibited
/// - **Consumer protection**: Clear pricing, easy opt-out for subscriptions
/// - **GDPR**: Phone numbers are PII, must be protected
#[derive(Debug, Clone)]
pub struct DirectCarrierBilling {
    pub(crate) phone: PhoneNumber,
    pub(crate) metadata: Option<Metadata>,
}

impl DirectCarrierBilling {
    /// User phone number (primary payment identifier)
    #[inline]
    pub fn phone(&self) -> &PhoneNumber {
        &self.phone
    }

    /// Carrier-specific extensions
    #[inline]
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

// Marker implementations

impl PaymentMethod for DirectCarrierBilling {}
impl ExternalPaymentMethod for DirectCarrierBilling {}

impl<'a> TryFrom<crate::DirectCarrier<'a>> for DirectCarrierBilling {
    type Error = Error;

    fn try_from(input: crate::DirectCarrier<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            phone: input.phone.try_into()?,
            metadata: input.metadata.map(TryFrom::try_from).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;

    fn valid_input() -> crate::DirectCarrier<'static> {
        inputs::DirectCarrier {
            phone: " +1234567890 \n\t",
            metadata: None,
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let dcb = DirectCarrierBilling::try_from(input).unwrap();

        unsafe {
            assert_eq!(dcb.phone.as_ref(), "+1234567890");
            assert!(dcb.metadata.is_none());
        }
    }

    #[test]
    fn rejects_invalid_phone() {
        let mut input = valid_input();
        input.phone = "123";

        let result = DirectCarrierBilling::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
