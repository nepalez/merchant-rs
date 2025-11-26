use std::convert::TryFrom;

use crate::types::{
    AccountNumber, Address, BankCode, EmailAddress, ExternalPaymentMethod, FullName, Metadata,
    NationalId, PhoneNumber, VirtualPaymentAddress,
};
use crate::{AccountHolderType, Error};

/// Instant payment systems
///
/// ## Overview
///
/// Real-time bank transfer where the customer is redirected to their bank
/// to authorize a one-time payment.
/// Funds are transferred immediately (seconds to minutes) with instant confirmation.
/// Customer actively initiates each payment through their banking interface.
/// Settlement is immediate with bank-level authentication.
///
/// ## When to Use
///
/// - **One-time payments**: Single transactions requiring immediate settlement
/// - **High-value transactions**: When bank-level authentication is required
/// - **Markets without card penetration**: Regions where bank transfers are preferred
/// - **Reduced fraud risk**: Bank authentication provides strong verification
/// - **No chargebacks**: Direct bank transfers are typically final
///
/// ## Authentication Model
///
/// > Authentication occurs **in redirect flow** through the bank's interface,
/// > not in authorization request!
///
/// ### Redirect Flow Steps
///
/// 1. **Merchant initiates**: Calls `authorize()` with customer identification data
/// 2. **Gateway generates redirect**: Returns URL to customer's bank
/// 3. **Customer redirects**: Browser redirects to bank login page
/// 4. **Bank authentication**: Customer logs in with bank credentials (password, biometric, etc.)
/// 5. **Transaction approval**: Customer explicitly approves payment in bank UI
/// 6. **Strong Customer Authentication**: Bank performs SCA per PSD2 or local regulations (2FA, biometric)
/// 7. **Return to merchant**: Customer redirected back with a transaction result
/// 8. **Webhook confirmation**: Gateway sends async notification of payment completion
///
/// ### Authorization Request Content
///
/// The authorization request contains **only customer identification data**
/// for routing and compliance.
/// No authentication credentials are needed — the customer authenticates directly with their bank.
///
/// ## Standards
///
/// - **[ISO 20022](https://www.iso20022.org/)**: Universal financial industry message scheme (global adoption)
/// - **[EMVCo QR Code Standards](https://www.emvco.com/emv-technologies/qrcodes/)**: Used by Asian instant payment systems
/// - **[NPCI Standards](https://www.npci.org.in/)**: National Payments Corporation of India (UPI)
/// - **[BCB PIX Specifications](https://www.bcb.gov.br/estabilidadefinanceira/pix)**: Brazilian Central Bank instant payment system
/// - **[MAS FAST Standards](https://www.mas.gov.sg/development/payments)**: Monetary Authority of Singapore (PayNow)
/// - **BOT Payment System Act**: Bank of Thailand (PromptPay)
/// - **EPI Standards**: European Payments Initiative (iDEAL → Wero migration)
/// - **[The Clearing House RTP](https://www.theclearinghouse.org/payment-systems/rtp)**: Real-Time Payments (United States)
///
/// ## Example Systems
///
/// ### Latin America
/// - **PIX** (Brazil): BCB instant payment, QR code, or tax-ID-based, 24/7 operation
/// - **PSE** (Colombia): Bank selection, redirect-based, requires ID number
/// - **SPEI** (Mexico): CLABE-based transfers, real-time settlement
///
/// ### Asia Pacific
/// - **UPI** (India): Virtual Payment Address (user@bank), QR code support, peer-to-peer
/// - **PayNow** (Singapore): Phone/NRIC based, instant peer-to-peer transfers
/// - **PromptPay** (Thailand): Phone/citizen ID-based, QR code support
/// - **FPX** (Malaysia): Bank selection, online banking redirect
///
/// ### Europe
/// - **iDEAL** (Netherlands): Bank selection, redirect to online banking, dominant in NL
/// - **Bancontact** (Belgium): Card-based instant payment, QR code support
/// - **Giropay** (Germany): Online banking redirect
/// - **SOFORT** (Europe): Multi-country instant transfer via Klarna
/// - **Trustly** (Europe): Pay N Play, account-to-account transfers
///
/// ### North America
/// - **Interac Online** (Canada): Online banking redirect
/// - **RTP** (United States): The Clearing House real-time payment network
/// - **FedNow** (United States): Federal Reserve instant payment service
///
/// ## Security Considerations
///
/// ### Bank-Level Authentication
/// - Customer authenticates with their own bank using existing credentials
/// - Banks implement SCA (Strong Customer Authentication) per PSD2 or local regulations
/// - Typically, 2FA: password + SMS OTP, biometric, hardware token
/// - Merchant never handles banking credentials
///
/// ### Data Protection
/// - Customer identification data (name, email, tax ID) is PII
/// - Use appropriate types (`NationalId`, `Email`) with automatic memory zeroization where applicable
/// - Comply with GDPR, LGPD, or local data protection regulations
/// - Tax IDs and national IDs should be masked in logs
///
/// ### Fraud Prevention
/// - Bank performs fraud checks before approving transfer
/// - Customer explicitly approves each transaction (no stored mandates)
/// - Real-time settlement reduces the window for fraud
/// - Irreversible transactions (no chargebacks in most systems)
///
/// ### Compliance
/// - **AML/KYC**: Banks perform customer verification
/// - **PSD2** (Europe): Requires SCA for most transactions
/// - **PIX regulations** (Brazil): CPF/CNPJ required for identification
/// - **GDPR/LGPD**: Customer data protection requirements
#[derive(Clone, Debug)]
pub struct InstantAccount {
    pub(crate) email: EmailAddress,
    pub(crate) full_name: FullName,
    pub(crate) account_number: Option<AccountNumber>,
    pub(crate) bank_code: Option<BankCode>,
    pub(crate) billing_address: Option<Address>,
    pub(crate) holder_type: AccountHolderType,
    pub(crate) national_id: Option<NationalId>,
    pub(crate) phone: Option<PhoneNumber>,
    pub(crate) virtual_payment_address: Option<VirtualPaymentAddress>,
    pub(crate) metadata: Option<Metadata>,
}

impl InstantAccount {
    /// User email for transaction notifications
    #[inline]
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    /// User full name as registered with a bank
    #[inline]
    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    /// Bank account number (CLABE for SPEI)
    #[inline]
    pub fn account_number(&self) -> Option<&AccountNumber> {
        self.account_number.as_ref()
    }

    /// Bank identifier code
    #[inline]
    pub fn bank_code(&self) -> Option<&BankCode> {
        self.bank_code.as_ref()
    }

    /// User billing address
    #[inline]
    pub fn billing_address(&self) -> Option<&Address> {
        self.billing_address.as_ref()
    }

    /// Type of user (person or organization)
    #[inline]
    pub fn holder_type(&self) -> &AccountHolderType {
        &self.holder_type
    }

    /// National identification number (tax ID)
    #[inline]
    pub fn national_id(&self) -> Option<&NationalId> {
        self.national_id.as_ref()
    }

    /// User phone number
    #[inline]
    pub fn phone(&self) -> Option<&PhoneNumber> {
        self.phone.as_ref()
    }

    /// Virtual Payment Address (UPI)
    #[inline]
    pub fn virtual_payment_address(&self) -> Option<&VirtualPaymentAddress> {
        self.virtual_payment_address.as_ref()
    }

    /// Method-specific extensions
    #[inline]
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

// Marker implementations

impl ExternalPaymentMethod for InstantAccount {}

impl<'a> TryFrom<crate::InstantPayment<'a>> for InstantAccount {
    type Error = Error;

    fn try_from(input: crate::InstantPayment<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            email: input.email.try_into()?,
            full_name: input.full_name.try_into()?,
            account_number: input.account_number.map(TryInto::try_into).transpose()?,
            bank_code: input.bank_code.map(TryInto::try_into).transpose()?,
            billing_address: input.billing_address.map(TryInto::try_into).transpose()?,
            holder_type: input.holder_type,
            national_id: input.national_id.map(TryInto::try_into).transpose()?,
            phone: input.phone.map(TryInto::try_into).transpose()?,
            virtual_payment_address: input
                .virtual_payment_address
                .map(TryInto::try_into)
                .transpose()?,
            metadata: input.metadata.map(TryInto::try_into).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;

    fn valid_input() -> crate::InstantPayment<'static> {
        inputs::InstantPayment {
            email: " user@example.com \n\t",
            full_name: " john doe \n\t",
            account_number: Some(" 1234567890123456 \n\t"),
            bank_code: Some(" 12345678 \n\t"),
            billing_address: Some(inputs::Address {
                country_code: " IN \n\t",
                postal_code: " 110001 \n\t",
                city: " New Delhi \n\t",
                line: " Connaught Place \n\t",
            }),
            holder_type: AccountHolderType::Individual,
            national_id: Some(" ABCDE1234F \n\t"),
            phone: Some(" +911234567890 \n\t"),
            virtual_payment_address: Some(" user@upi \n\t"),
            metadata: None,
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let instant = InstantAccount::try_from(input).unwrap();

        unsafe {
            assert_eq!(instant.email.as_ref(), "user@example.com");
            assert_eq!(instant.full_name.as_ref(), "JOHN DOE");
            if let Some(ref account_number) = instant.account_number {
                assert_eq!(account_number.as_ref(), "1234567890123456");
            }
            if let Some(ref bank_code) = instant.bank_code {
                assert_eq!(bank_code.as_ref(), "12345678");
            }
            assert!(instant.billing_address.is_some());
            if let Some(ref national_id) = instant.national_id {
                assert_eq!(national_id.as_ref(), "ABCDE1234F");
            }
            if let Some(ref phone) = instant.phone {
                assert_eq!(phone.as_ref(), "+911234567890");
            }
            if let Some(ref vpa) = instant.virtual_payment_address {
                assert_eq!(vpa.as_ref(), "user@upi");
            }
            assert!(instant.metadata.is_none());
        }
    }

    #[test]
    fn rejects_invalid_email() {
        let mut input = valid_input();
        input.email = "invalid";

        let result = InstantAccount::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_full_name() {
        let mut input = valid_input();
        input.full_name = "X";

        let result = InstantAccount::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_account_number() {
        let mut input = valid_input();
        input.account_number = Some("123");

        let result = InstantAccount::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_phone() {
        let mut input = valid_input();
        input.phone = Some("123");

        let result = InstantAccount::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
