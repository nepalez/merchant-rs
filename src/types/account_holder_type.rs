use std::fmt::Display;

/// Categories of account holders for bank accounts and payment methods.
///
/// Distinguishes between individual consumers and business/corporate entities.
/// Used for compliance, risk assessment, and routing decisions by payment gateways.
///
/// # Variants
///
/// * `Individual` - Personal account held by a natural person
/// * `Company` - Business account held by a legal entity (corporation, LLC, partnership, etc.)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AccountHolderType {
    /// Personal account held by an individual consumer
    Individual,
    /// Business account held by a company or organization
    Company,
}

impl Display for AccountHolderType {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
