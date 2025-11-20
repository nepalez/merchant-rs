use strum_macros::{AsRefStr, Display};

/// Categories of account holders for bank accounts and payment methods.
///
/// Distinguishes between individual consumers and business/corporate entities.
/// Used for compliance, risk assessment, and routing decisions by payment gateways.
///
/// # Variants
///
/// * `Individual` - Personal account held by a natural person
/// * `Company` - Business account held by a legal entity (corporation, LLC, partnership, etc.)
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum AccountHolderType {
    /// Personal account held by an individual consumer
    Individual,
    /// Business account held by a company or organization
    Company,
}
