use crate::inputs::{BNPL, BankAccount, CashVoucher, CreditCard, InstantBankAccount, SEPAAccount};

/// Payment source used for transaction authorization
///
/// Each variant represents a distinct payment flow type with specific data requirements,
/// authentication models, and settlement characteristics. For detailed documentation
/// about each payment method, see the individual Markdown files in this directory.
pub enum PaymentSource<'a> {
    /// Direct bank account debit payment.
    ///
    /// Automated recurring debits from a user's bank account based on a pre-authorized mandate.
    /// Settlement takes 1-3 business days.
    /// Authentication occurs outside the authorization flow
    /// through micro-deposits, instant verification, or signed mandate.
    BankAccount(BankAccount<'a>),

    /// Buy Now Pay Later payment.
    ///
    /// User receives goods immediately, pays in installments (3-12 months).
    /// BNPL provider extends credit and assumes payment risk.
    /// Authentication through provider redirect and credit assessment.
    #[allow(clippy::upper_case_acronyms)]
    BNPL(BNPL<'a>),

    /// Cash-based voucher payment.
    ///
    /// User receives a voucher with barcode, pays cash at physical locations
    /// (stores, banks, post offices).
    /// Asynchronous settlement (1-3 days).
    /// Authentication through physical payment confirmation.
    CashVoucher(CashVoucher<'a>),

    /// Instant bank transfer payment.
    ///
    /// Real-time bank transfer with user redirect to bank for authorization.
    /// Immediate settlement (seconds to minutes).
    /// Authentication through bank login and transaction approval in bank UI.
    InstantBankTransfer(InstantBankAccount<'a>),

    /// Credit or debit card payment.
    ///
    /// Traditional payment card transaction where the user provides card details for authorization.
    /// Most universal payment methods worldwide.
    /// Authentication through CVV and optional 3D Secure.
    CreditCard(CreditCard<'a>),

    /// SEPA bank transfer payment.
    ///
    /// EUR-denominated transfers within 36 European countries.
    /// SEPA Instant provides real-time settlement (10 seconds),
    /// standard SEPA takes 1-2 business days.
    /// PSD2 SCA authentication.
    SEPATransfer(SEPAAccount<'a>),

    /// Tokenized payment method.
    ///
    /// Pre-authorized payment method represented by secure token.
    /// Token references stored payment method or session token
    /// from a completed flow (Apple Pay, Alipay, etc.).
    /// Authentication embedded in a token.
    TokenizedPayment(&'a str),
}
