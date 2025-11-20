use strum_macros::{AsRefStr, Display};

/// Merchant-Initiated Transaction type
///
/// Categorizes transactions initiated by merchant without active customer participation.
/// Required for Visa/Mastercard COF (Credential-on-File) mandate compliance.
#[derive(AsRefStr, Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum MerchantInitiatedType {
    /// Interval recurring payments (subscriptions, memberships)
    Recurring,

    /// Fixed or variable amount installment payments (BNPL, split payments)
    Installment,

    /// Variable amount or timing, event-triggered (auto top-up, usage billing)
    UnscheduledCardOnFile,

    /// Retry after temporary decline (insufficient funds, network timeout)
    Resubmission,

    /// Charge after service delivery (hotel, car rental incidentals)
    DelayedCharge,

    /// Additional amount on existing authorization (hotel extended stay, gas pump)
    Incremental,

    /// New authorization replacing expired one (extended car rental, split shipment)
    Reauthorization,

    /// Penalty charge for unused reservation (hotel, airline no-show)
    NoShow,
}
