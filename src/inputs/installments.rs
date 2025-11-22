/// Insecure representation of installment payment options.
///
/// ```skip
/// let total = Installments::TotalPayment;
/// let fixed = Installments::FixedPlan { count: 6 };
/// let stored = Installments::StoredPlan { id: "INS54434" };
/// ```
#[derive(Debug, Clone)]
pub enum Installments<'a> {
    /// Single payment (no installments).
    TotalPayment,
    /// Fixed number of installments (2-99).
    FixedPlan { count: u8 },
    /// Gateway-specific stored installment plan.
    StoredPlan { id: &'a str },
}
