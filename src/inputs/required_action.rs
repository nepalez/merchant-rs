// TODO: Add more variants (Iframe, QRCode, Voucher, BankTransfer, etc.)
/// Customer action required to complete the authorization.
///
/// ```skip
/// let action = RequiredAction::Redirect {
///     url: "https://gateway.example.com/auth/123",
///     return_url: "https://merchant.example.com/callback",
/// }.try_into()?;
/// ```
pub enum RequiredAction<'a> {
    /// Redirect the customer to an external URL for approval.
    Redirect {
        /// The URL to redirect the customer to.
        url: &'a str,
        /// The URL to return to after the customer completes the action.
        return_url: &'a str,
    },
}
