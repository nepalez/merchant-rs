use crate::inputs::CardExpiry;

pub struct CreditCard<'a> {
    /// Card Verification Value (CVV/CVC/CID)
    pub cvv: &'a str,
    /// Primary Account Number (PAN)
    pub number: &'a str,
    /// Card expiration date (month and year)
    pub card_expiry: CardExpiry,
    /// Cardholder name as embossed on the card
    pub holder_name: &'a str,
}
