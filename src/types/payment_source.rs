use crate::Error;
use crate::inputs::PaymentSource as Input;
use crate::types::{
    BNPL, BankAccount, CashVoucher, CreditCard, InstantBankAccount, SEPAAccount, Token,
};

/// Payment source used for transaction authorization
///
/// Each variant represents a distinct payment flow type with specific data requirements,
/// authentication models, and settlement characteristics. For detailed documentation
/// about each payment method, see the individual markdown files in this directory.
#[derive(Clone, Debug)]
pub enum PaymentSource {
    /// Direct bank account debit payment.
    ///
    /// Automated recurring debits from user's bank account based on pre-authorized mandate.
    /// Settlement takes 1-3 business days.
    /// Authentication occurs outside the authorization flow
    /// through micro-deposits, instant verification, or signed mandate.
    BankAccount(BankAccount),

    /// Buy Now Pay Later payment.
    ///
    /// User receives goods immediately, pays in installments (3-12 months).
    /// BNPL provider extends credit and assumes payment risk.
    /// Authentication through provider redirect and credit assessment.
    BNPL(BNPL),

    /// Cash-based voucher payment.
    ///
    /// User receives voucher with barcode, pays cash at physical locations
    /// (stores, banks, post offices).
    /// Asynchronous settlement (1-3 days).
    /// Authentication through physical payment confirmation.
    CashVoucher(CashVoucher),

    /// Direct carrier billing payment.
    ///
    /// Payment charged to user's mobile phone bill.
    /// User authorizes via SMS PIN, charge appears on next phone bill.
    /// Authentication through SMS challenge-response with carrier verification.
    // DirectCarrierBilling {
    //    /// User phone number (primary payment identifier)
    //    phone: PhoneNumber,
    //    /// Carrier-specific extensions
    //    metadata: Option<Metadata>,
    // },

    /// Instant bank transfer payment.
    ///
    /// Real-time bank transfer with user redirect to bank for authorization.
    /// Immediate settlement (seconds to minutes).
    /// Authentication through bank login and transaction approval in bank UI.
    InstantBankTransfer(InstantBankAccount),

    /// Credit or debit card payment.
    ///
    /// Traditional payment card transaction where user provides card details for authorization.
    /// Most universal payment method worldwide.
    /// Authentication through CVV and optional 3D Secure.
    CreditCard(CreditCard),

    /// SEPA bank transfer payment.
    ///
    /// EUR-denominated transfers within 36 European countries.
    /// SEPA Instant provides real-time settlement (10 seconds),
    /// standard SEPA takes 1-2 business days.
    /// PSD2 SCA authentication.
    SEPATransfer(SEPAAccount),

    /// Tokenized payment method.
    ///
    /// Pre-authorized payment method represented by secure token. Token references stored payment method
    /// or session token from completed flow (Apple Pay, Alipay, etc.). Authentication embedded in token.
    ///
    /// **Use for:** Recurring payments, saved payment methods, device wallets, e-wallet redirects
    /// anonymous payments, cash-based purchasing, privacy-focused transactions
    /// **Examples:** Stored cards, Apple Pay, Google Pay, Alipay, WeChat Pay, network tokens,
    /// Paysafecard (direct entry), Neosurf, Flexepin, CASHlib, AstroPay
    TokenizedPayment(Token),
}

impl<'a> TryFrom<Input<'a>> for PaymentSource {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(match input {
            Input::BankAccount(input) => Self::BankAccount(input.try_into()?),
            Input::BNPL(input) => Self::BNPL(input.try_into()?),
            Input::CashVoucher(input) => Self::CashVoucher(input.try_into()?),
            Input::InstantBankTransfer(input) => Self::InstantBankTransfer(input.try_into()?),
            Input::CreditCard(input) => Self::CreditCard(input.try_into()?),
            Input::SEPATransfer(input) => Self::SEPATransfer(input.try_into()?),
            Input::TokenizedPayment(input) => Self::TokenizedPayment(input.try_into()?),
        })
    }
}
