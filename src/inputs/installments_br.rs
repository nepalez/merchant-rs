use rust_decimal::Decimal;

/// Insecure representation of Brazil installment payment options.
///
/// ```skip
/// let total = InstallmentsBR::TotalPayment;
/// let fixed = InstallmentsBR::FixedPlan { count: 6, fee: None };
/// let with_fee = InstallmentsBR::FixedPlan { count: 6, fee: Some(dec!(5.00)) };
/// let stored = InstallmentsBR::StoredPlan { id: "INS54434" };
/// ```
#[derive(Debug, Clone)]
pub enum InstallmentsBR<'a> {
    /// Single payment (no installments).
    TotalPayment,
    /// Fixed number of installments (2-99) with optional fee.
    ///
    /// Fee represents the known installment financing charge added to base amount.
    FixedPlan { count: u8, fee: Option<Decimal> },
    /// Gateway-specific stored installment plan.
    StoredPlan { id: &'a str },
}
