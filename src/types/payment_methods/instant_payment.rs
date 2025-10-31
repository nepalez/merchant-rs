use std::convert::TryFrom;

use crate::Error;
use crate::inputs::InstantPayment as Input;
use crate::types::{
    AccountHolderType, AccountNumber, Address, BankCode, EmailAddress, ExternalPaymentMethod,
    FullName, Metadata, NationalId, PaymentMethod, PhoneNumber, VirtualPaymentAddress,
};

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
    email: EmailAddress,
    full_name: FullName,
    account_number: Option<AccountNumber>,
    bank_code: Option<BankCode>,
    billing_address: Option<Address>,
    holder_type: AccountHolderType,
    national_id: Option<NationalId>,
    phone: Option<PhoneNumber>,
    virtual_payment_address: Option<VirtualPaymentAddress>,
    metadata: Option<Metadata>,
}

// Marker implementations

impl PaymentMethod for InstantAccount {}
impl ExternalPaymentMethod for InstantAccount {}

// Converters

impl InstantAccount {
    /// User email for transaction notifications
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    /// User full name as registered with a bank
    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    /// Bank account number (CLABE for SPEI)
    pub fn account_number(&self) -> Option<&AccountNumber> {
        self.account_number.as_ref()
    }

    /// Bank identifier code
    pub fn bank_code(&self) -> Option<&BankCode> {
        self.bank_code.as_ref()
    }

    /// User billing address
    pub fn billing_address(&self) -> Option<&Address> {
        self.billing_address.as_ref()
    }

    /// Type of user (person or organization)
    pub fn holder_type(&self) -> AccountHolderType {
        self.holder_type
    }

    /// National identification number (tax ID)
    pub fn national_id(&self) -> Option<&NationalId> {
        self.national_id.as_ref()
    }

    /// User phone number
    pub fn phone(&self) -> Option<&PhoneNumber> {
        self.phone.as_ref()
    }

    /// Virtual Payment Address (UPI)
    pub fn virtual_payment_address(&self) -> Option<&VirtualPaymentAddress> {
        self.virtual_payment_address.as_ref()
    }

    /// Method-specific extensions
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

impl<'a> TryFrom<Input<'a>> for InstantAccount {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
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
