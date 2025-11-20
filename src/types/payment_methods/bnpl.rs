use std::convert::TryFrom;

use crate::types::{
    Address, BirthDate, EmailAddress, ExternalPaymentMethod, FullName, Metadata, NationalId,
    PaymentMethod, PhoneNumber,
};
use crate::{AccountHolderType, Error};

/// Buy Now Pay Later
///
/// ## Overview
///
/// Customer receives goods immediately but pays in installments over time (typically 3–12 months).
/// The BNPL provider extends credit to the customer and assumes payment risk from the merchant.
/// Requires extensive customer information for credit assessment and identity verification.
/// Popular alternative to credit cards, especially among younger consumers.
///
/// ## Authentication Model
///
/// > Authentication occurs **in redirect flow** through the BNPL provider,
/// > not in authorization request!
///
/// ### Redirect Flow Steps
///
/// 1. **Merchant initiates**: Calls `authorize()` with customer data for credit assessment
/// 2. **Gateway redirects**: Returns URL to BNPL provider (Klarna, Afterpay, etc.)
/// 3. **Provider login**: Customer creates an account or logs into an existing account
/// 4. **Credit check**: Provider performs real-time credit assessment using provided data
/// 5. **Identity verification**: Provider verifies customer identity (may require additional documents)
/// 6. **Agreement approval**: Customer reviews and explicitly approves installment payment terms
/// 7. **Return to merchant**: Customer redirected back with an approval/decline result
/// 8. **Merchant fulfillment**: If approved, merchant ships goods; BNPL provider pays merchant
///
/// ### Authorization Request Content
///
/// The authorization request contains **customer data for credit assessment**, not authentication credentials.
/// The more complete the data, the higher the approval rate.
/// Customer authenticates with the BNPL provider directly.
///
/// ## Standards
///
/// - **[Consumer Credit Directive (EU)](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32008L0048)**: Regulates credit agreements for consumers
/// - **[Truth in Lending Act (US)](https://www.consumerfinance.gov/rules-policy/regulations/1026/)**: Disclosure requirements for consumer credit (Regulation Z)
/// - **Consumer Credit Protection Act**: US federal consumer financial law
/// - **[FCA Consumer Credit Rules](https://www.fca.org.uk/firms/consumer-credit)**: Financial Conduct Authority regulations (UK)
/// - **[ASIC Guidelines](https://asic.gov.au/regulatory-resources/credit/)**: Australian Securities and Investments Commission (Australia)
///
/// ## Example Systems
///
/// ### Global Providers
/// - **Klarna**: Sweden-based, operates in 45+ countries, 150M+ customers
/// - **Afterpay/Clearpay**: Australia-based (Block/Square), strong in AU/US/the UK
/// - **Affirm**: US-based, transparent pricing, no late fees model
/// - **PayPal Pay Later**: Integrated with PayPal ecosystem, global reach
///
/// ### Regional Providers
/// - **Zip** (AU/NZ): Australian market leader, expanding to the US
/// - **Sezzle** (US/CA): Focus on North America, millennial-targeted
/// - **Splitit**: Uses existing credit card limits, no credit check
/// - **Atome** (APAC): Singapore-based, Southeast Asia focus
/// - **Scalapay** (EU): Italian-based, Southern Europe focus
/// - **Tabby** (Middle East): UAE-based, MENA region
/// - **Tamara** (Middle East): Saudi Arabia-based, Gulf region
///
/// ## Security Considerations
///
/// ### Customer Data Protection
/// - Extensive PII collected: name, address, DOB, national ID
/// - All data must be protected per GDPR, CCPA, or local regulations
/// - Use appropriate types (`NationalId`, `Date`, `Email`) with proper masking
/// - Minimize data retention after transaction
///
/// ### Credit Assessment Data
/// - Date of birth required for age verification and credit checks
/// - some providers require National identifier (SSN in the US, CPF in Brazil, NRIC in Singapore)
/// - Shipping address compared with billing for fraud detection
/// - Phone number used for identity verification
///
/// ### Fraud Prevention
/// - BNPL providers perform extensive fraud checks
/// - Identity verification through document upload or database checks
/// - Device fingerprinting and behavioral analysis
/// - Merchant bears fraud risk only if shipping before approval
/// - Customer credit checks protect against default risk
///
/// ### Compliance
/// - **Consumer credit regulations**: Must comply with local lending laws
/// - **Truth in Lending**: Clear disclosure of terms, APR, fees (where applicable)
/// - **Age restrictions**: Typically 18+ required
/// - **Credit reporting**: Some providers report to credit bureaus
/// - **Data protection**: GDPR, CCPA compliance for customer data
#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct BNPL {
    pub(crate) billing_address: Address,
    pub(crate) email: EmailAddress,
    pub(crate) full_name: FullName,
    pub(crate) account_holder_type: AccountHolderType,
    pub(crate) date_of_birth: Option<BirthDate>,
    pub(crate) national_id: Option<NationalId>,
    pub(crate) phone: Option<PhoneNumber>,
    pub(crate) metadata: Option<Metadata>,
}

impl BNPL {
    /// User billing address
    #[inline]
    pub fn billing_address(&self) -> &Address {
        &self.billing_address
    }

    /// User email address
    #[inline]
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    /// User full name
    #[inline]
    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    /// Type of account holder (individual or company)
    #[inline]
    pub fn account_holder_type(&self) -> &AccountHolderType {
        &self.account_holder_type
    }

    /// User date of birth
    #[inline]
    pub fn date_of_birth(&self) -> Option<&BirthDate> {
        self.date_of_birth.as_ref()
    }

    /// National identification number
    #[inline]
    pub fn national_id(&self) -> Option<&NationalId> {
        self.national_id.as_ref()
    }

    /// User phone number
    #[inline]
    pub fn phone(&self) -> Option<&PhoneNumber> {
        self.phone.as_ref()
    }

    /// Method-specific extensions
    #[inline]
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

// Marker implementations

impl PaymentMethod for BNPL {}
impl ExternalPaymentMethod for BNPL {}

impl<'a> TryFrom<crate::BNPL<'a>> for BNPL {
    type Error = Error;

    fn try_from(input: crate::BNPL<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            billing_address: input.billing_address.try_into()?,
            email: input.email.try_into()?,
            full_name: input.full_name.try_into()?,
            account_holder_type: input.account_holder_type,
            date_of_birth: input.date_of_birth.map(TryFrom::try_from).transpose()?,
            national_id: input.national_id.map(TryFrom::try_from).transpose()?,
            phone: input.phone.map(TryFrom::try_from).transpose()?,
            metadata: input.metadata.map(TryFrom::try_from).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;

    fn valid_input() -> crate::BNPL<'static> {
        inputs::BNPL {
            billing_address: inputs::Address {
                country_code: " US \n\t",
                postal_code: " 10001 \n\t",
                city: " New York \n\t",
                line: " 123 Main St \n\t",
            },
            email: " user@example.com \n\t",
            full_name: " john doe \n\t",
            account_holder_type: AccountHolderType::Individual,
            date_of_birth: Some(inputs::BirthDate {
                day: 15,
                month: 8,
                year: 1990,
            }),
            national_id: Some(" 123456789 \n\t"),
            phone: Some(" +1234567890 \n\t"),
            metadata: None,
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let bnpl = BNPL::try_from(input).unwrap();

        unsafe {
            assert_eq!(bnpl.billing_address.country_code.as_ref(), "US");
            assert_eq!(bnpl.billing_address.postal_code.as_ref(), "10001");
            assert_eq!(bnpl.billing_address.city.as_ref(), "New York");
            assert_eq!(bnpl.billing_address.line.as_ref(), "123 Main St");
            assert_eq!(bnpl.email.as_ref(), "user@example.com");
            assert_eq!(bnpl.full_name.as_ref(), "JOHN DOE");
            if let Some(ref dob) = bnpl.date_of_birth {
                assert_eq!(*dob.day(), 15);
                assert_eq!(*dob.month(), 8);
                assert_eq!(*dob.year(), 1990);
            }
            if let Some(ref national_id) = bnpl.national_id {
                assert_eq!(national_id.as_ref(), "123456789");
            }
            if let Some(ref phone) = bnpl.phone {
                assert_eq!(phone.as_ref(), "+1234567890");
            }
            assert!(bnpl.metadata.is_none());
        }
    }

    #[test]
    fn rejects_invalid_email() {
        let mut input = valid_input();
        input.email = "invalid";

        let result = BNPL::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_full_name() {
        let mut input = valid_input();
        input.full_name = "X";

        let result = BNPL::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_billing_address() {
        let mut input = valid_input();
        input.billing_address.city = "";

        let result = BNPL::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_phone() {
        let mut input = valid_input();
        input.phone = Some("123");

        let result = BNPL::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
