/// Indicates whether this payment uses stored credentials for the first time
/// or subsequently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoredCredentialUsage<'a> {
    /// The first use of credentials for storage (Customer Initiated Transaction).
    Initial,
    /// A later use of stored credentials (Merchant Initiated Transaction).
    /// Contains reference to the original Initial transaction ID.
    Subsequent(&'a str),
}
