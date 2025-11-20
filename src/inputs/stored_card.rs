use crate::{CardExpiry, Credentials};

/// Credit or Debit Card stored for MTI (Merchant Initiated Transactions)
pub struct StoredCard<'a> {
    /// Tokenizable credentials for StoredCard
    pub credentials: Credentials<'a, StoredCardCredentials<'a>>,
}

/// Tokenizable credentials for StoredCard
pub struct StoredCardCredentials<'a> {
    /// Primary Account Number (PAN)
    pub number: &'a str,
    /// Card expiration date (month and year)
    pub card_expiry: CardExpiry,
    /// Cardholder name as embossed on the card
    pub holder_name: &'a str,
}
