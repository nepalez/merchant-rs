/// Insecure representation of a Mexico MSI installment plan.
///
/// ```skip
/// let single = MexicoPlan::Single;
/// let three = MexicoPlan::Three;
/// let id = MexicoPlan::Id("plan_123");
/// ```
#[derive(Debug, Clone)]
pub enum MexicoPlan<'a> {
    /// Single payment (no installments).
    Single,
    /// 3 months without interest.
    Three,
    /// 6 months without interest.
    Six,
    /// 9 months without interest.
    Nine,
    /// 12 months without interest.
    Twelve,
    /// 18 months without interest.
    Eighteen,
    /// Gateway-specific plan ID from installments API.
    Id(&'a str),
}
