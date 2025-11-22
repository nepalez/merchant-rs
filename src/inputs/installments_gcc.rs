//! Insecure representation of Gulf countries installment payment options.

/// Insecure representation of Gulf countries installment payment options.
///
/// ```skip
/// let total = InstallmentsGCC::TotalPayment;
/// let standard = InstallmentsGCC::FixedPlan { count: 6, shariah_compliant: false };
/// let shariah = InstallmentsGCC::FixedPlan { count: 6, shariah_compliant: true };
/// let stored = InstallmentsGCC::StoredPlan { id: "INS54434" };
/// ```
#[derive(Debug, Clone)]
pub enum InstallmentsGCC<'a> {
    /// Single payment (no installments).
    TotalPayment,
    /// A fixed number of installments (2-99) with Shariah compliance option.
    ///
    /// shariah_compliant indicates whether the plan follows Islamic finance principles.
    FixedPlan { count: u8, shariah_compliant: bool },
    /// Gateway-specific stored installment plan.
    StoredPlan { id: &'a str },
}
