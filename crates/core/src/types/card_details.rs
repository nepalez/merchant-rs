use crate::{CVV, CardExpiry, CardHolderName, PrimaryAccountNumber};

/// Details about a credit or debit card (Used ONLY for initial tokenization requests).
#[derive(Debug)]
pub struct CardDetails {
    /// The primary account number (PAN).
    pub number: PrimaryAccountNumber,
    /// The card's expiration time (month and year).
    pub card_expiry: CardExpiry,
    /// The card verification value (CVV/CVC), optional for vaulted cards.
    pub cvv: Option<CVV>,
    /// The name of the cardholder, optional for most APIs.
    pub holder_name: Option<CardHolderName>,
}
