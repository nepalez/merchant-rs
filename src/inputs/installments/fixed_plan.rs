/// Insecure representation of a fixed installment plan.
///
/// ```skip
/// let single = FixedPlan::Single;
/// let count = FixedPlan::Count(6);
/// let id = FixedPlan::Id("INS54434");
/// ```
#[derive(Debug, Clone)]
pub enum FixedPlan<'a> {
    /// Single payment (no installments).
    Single,
    /// Installments with specified count (2-99).
    Count(u8),
    /// Gateway-specific plan ID from installments API.
    Id(&'a str),
}
