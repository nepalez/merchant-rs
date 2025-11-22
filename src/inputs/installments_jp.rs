//! Insecure representation of Japan installment payment options.

/// Insecure representation of Japan installment payment options.
///
/// ```skip
/// let total = InstallmentsJP::TotalPayment;
/// let fixed = InstallmentsJP::FixedPlan { count: 6 };
/// let revolving = InstallmentsJP::RevolvingPlan;
/// let bonus = InstallmentsJP::BonusPlan;
/// let stored = InstallmentsJP::StoredPlan { id: "INS54434" };
/// ```
#[derive(Debug, Clone)]
pub enum InstallmentsJP<'a> {
    /// Single payment (no installments).
    TotalPayment,
    /// A fixed number of installments (2-99).
    FixedPlan { count: u8 },
    /// Revolving credit plan.
    RevolvingPlan,
    /// Bonus payment plan (two payments per year).
    BonusPlan,
    /// Gateway-specific stored installment plan.
    StoredPlan { id: &'a str },
}
