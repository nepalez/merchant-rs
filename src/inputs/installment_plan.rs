/// Insecure representation of a fixed installment plan.
///
/// ```skip
/// let single = FixedPlan::Single;
/// let count = FixedPlan::Count(6);
/// let id = FixedPlan::Id("INS54434");
/// ```
#[derive(Debug, Clone)]
pub enum InstallmentPlan<'a> {
    /// Single payment (no installments).
    Single,
    /// Installments with a specified count (2-99).
    Regular(u8),
    /// Gateway-specific plan ID from installments API.
    Id(&'a str),
}
