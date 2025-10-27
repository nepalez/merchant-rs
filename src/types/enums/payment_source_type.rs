use strum::Display;

/// The type of payment source accepted by gateways.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum PaymentSourceType {
    #[strum(to_string = "bank accounts")]
    BankAccount,
    #[strum(to_string = "BNPL (buy now pay later)")]
    BNPL,
    #[strum(to_string = "cash vouchers")]
    CashVoucher,
    #[strum(to_string = "crypto payments")]
    CryptoPayment,
    #[strum(to_string = "direct carrier billing")]
    DirectCarrierBilling,
    #[strum(to_string = "instant bank transfers")]
    InstantBankTransfer,
    #[strum(to_string = "payment cards")]
    PaymentCard,
    #[strum(to_string = "SEPA transfers")]
    SEPATransfer,
    #[strum(to_string = "payment tokens")]
    TokenizedPayment,
}
