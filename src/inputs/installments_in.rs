//! Insecure representation of India installment payment options.

/// Insecure representation of India installment payment options.
///
/// ```skip
/// let total = InstallmentsIN::TotalPayment;
/// let fixed = InstallmentsIN::FixedPlan { count: 6, offer_id: None };
/// let no_cost = InstallmentsIN::FixedPlan { count: 6, offer_id: Some("OFFER123") };
/// let stored = InstallmentsIN::StoredPlan { id: "INS54434", offer_id: None };
/// let stored_promo = InstallmentsIN::StoredPlan { id: "INS54434", offer_id: Some("OFFER456") };
/// ```
#[derive(Debug, Clone)]
pub enum InstallmentsIN<'a> {
    /// Single payment (no installments).
    TotalPayment,
    /// A fixed number of installments (2-99) with optional No Cost EMI offer.
    ///
    /// The `offer_id` represents a No Cost EMI offer where the merchant absorbs interest.
    FixedPlan {
        count: u8,
        offer_id: Option<&'a str>,
    },
    /// Gateway-specific stored installment plan with an optional offer.
    StoredPlan {
        id: &'a str,
        offer_id: Option<&'a str>,
    },
}
