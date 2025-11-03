use std::convert::TryFrom;

use crate::Error;
use crate::inputs::BNPL as Input;
use crate::types::{
    AccountHolderType, Address, BirthDate, EmailAddress, ExternalPaymentMethod, FullName, Metadata,
    NationalId, PaymentMethod, PhoneNumber,
};

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
/// - National identifier is required by some providers (SSN in the US, CPF in Brazil, NRIC in Singapore)
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
    /// User billing address
    pub billing_address: Address,
    /// User email address
    pub email: EmailAddress,
    /// User full name
    pub full_name: FullName,
    /// Type of account holder (individual or company)
    pub account_holder_type: AccountHolderType,
    /// User date of birth
    pub date_of_birth: Option<BirthDate>,
    /// National identification number
    pub national_id: Option<NationalId>,
    /// User phone number
    pub phone: Option<PhoneNumber>,
    /// Method-specific extensions
    pub metadata: Option<Metadata>,
}

// Marker implementations

impl PaymentMethod for BNPL {}
impl ExternalPaymentMethod for BNPL {}

impl<'a> TryFrom<Input<'a>> for BNPL {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
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
