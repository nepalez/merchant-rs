/// Stored payment credential for recurring payments (mandates, tokens, setup intents)
pub struct StoredCredential<'a> {
    /// Token representing this stored credential from a payment gateway
    pub token: &'a str,
    /// Optional customer identifier associated with this stored credential
    pub customer_id: Option<&'a str>,
}
