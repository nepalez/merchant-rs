/// Insecure representation of a Japan extended installment plan.
///
/// ```skip
/// let single = ExtendedPlan::Single;
/// let regular = ExtendedPlan::Regular(6);
/// let revolving = ExtendedPlan::Revolving;
/// ```
#[derive(Debug, Clone)]
pub enum ExtendedPlan<'a> {
    /// Single payment (no installments).
    Single,
    /// Regular installments with specified count (2-99).
    Regular(u8),
    /// Revolving credit plan.
    Revolving,
    /// Bonus payment plan (July and December).
    Bonus,
    /// Gateway-specific plan ID from installments API.
    Id(&'a str),
}
