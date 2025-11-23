use crate::Error;
use crate::types::TransactionId;

/// Indicates whether this payment uses stored credentials for the first time or subsequently.
///
/// ## Overview
///
/// Card networks (Visa, Mastercard) require merchants to indicate whether a payment is the first
/// use of stored credentials (Initial) or a later use (Subsequent). This is part of the
/// Credential-on-File (COF) mandate compliance.
///
/// ## Usage Context
///
/// This field should be used in conjunction with:
/// - `StoreCredentials` flow: When storing payment credentials
/// - `MerchantInitiatedType`: For recurring/installment/unscheduled payments
///
/// ## Variants
///
/// ### Initial
/// First use of credentials for storage. Used when:
/// - Customer provides full payment details (including CVV for cards)
/// - Credentials are being stored for future use
/// - This is a Customer Initiated Transaction (CIT) with storage consent
///
/// The gateway will:
/// - Store the credentials in the vault
/// - Return a network transaction ID
/// - This transaction ID must be saved and used in later transactions
///
/// ### Subsequent
/// Subsequent use of stored credentials. Used when:
/// - Merchant initiates payment using previously stored credentials
/// - This is a Merchant Initiated Transaction (MIT)
/// - No CVV is required (for cards)
///
/// Must include:
/// - `TransactionId` of the Initial transaction
/// - This links the MIT to the original CIT for card network compliance
///
/// ## Card Network Compliance
///
/// ### Visa
/// - Requires "Stored Credential Transaction Framework" indicators
/// - Initial transactions must include cardholder-consent-Subsequent transactions
///   must reference the original transaction ID
///
/// ### Mastercard
/// - Requires "Merchant Initiated Transaction" indicators
/// - Must distinguish between first and later uses
/// - Improves authorization rates when properly indicated
///
/// ### Effects on Authorization
/// - Proper indicators can increase approval rates
/// - Issuers apply different risk rules for COF transactions
/// - Reduces false fraud declines for legitimate recurring payments
///
/// ## Example Flows
///
/// ### Subscription Setup (Initial)
/// ```skip
/// // Customer signs up for subscription, provides card details
/// let payment = Payment {
///     method: credit_card,
///     amount: money,
///     idempotence_key: key,
///     merchant_initiated_type: Some(MerchantInitiatedType::Recurring),
///     stored_credential_usage: StoredCredentialUsage::Initial,
/// };
///
/// // After successful payment, save the transaction_id for future use
/// let transaction = gateway.charge(payment).await?;
/// save_transaction_id_for_subscription(transaction.id);
/// ```
///
/// ### Subscription Renewal (Subsequent)
/// ```skip
/// // Monthly renewal - merchant initiates without customer interaction
/// let original_transaction_id = get_saved_transaction_id();
///
/// let payment = Payment {
///     method: stored_card, // Previously tokenized
///     amount: money,
///     idempotence_key: key,
///     merchant_initiated_type: Some(MerchantInitiatedType::Recurring),
///     stored_credential_usage: StoredCredentialUsage::Subsequent(original_transaction_id),
/// };
///
/// gateway.charge(payment).await?;
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StoredCredentialUsage {
    /// The first use of credentials for storage (Customer Initiated Transaction)
    Initial,
    /// A later use of stored credentials (Merchant Initiated Transaction)
    /// Contains reference to the original Initial transaction
    Subsequent(TransactionId),
}

impl TryFrom<crate::StoredCredentialUsage<'_>> for StoredCredentialUsage {
    type Error = Error;

    fn try_from(input: crate::StoredCredentialUsage<'_>) -> Result<Self, Self::Error> {
        Ok(match input {
            crate::StoredCredentialUsage::Initial => Self::Initial,
            crate::StoredCredentialUsage::Subsequent(id) => Self::Subsequent(id.try_into()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;

    #[test]
    fn converts_initial() {
        let input = inputs::StoredCredentialUsage::Initial;
        let result = StoredCredentialUsage::try_from(input).unwrap();

        assert_eq!(result, StoredCredentialUsage::Initial);
    }

    #[test]
    fn converts_subsequent_with_valid_id() {
        let input = inputs::StoredCredentialUsage::Subsequent(" txn_12345 \n\t");
        let result = StoredCredentialUsage::try_from(input).unwrap();

        match result {
            StoredCredentialUsage::Subsequent(id) => unsafe {
                assert_eq!(id.as_ref(), "txn_12345");
            },
            _ => panic!("Expected Subsequent variant"),
        }
    }

    #[test]
    fn rejects_subsequent_with_invalid_id() {
        let input = inputs::StoredCredentialUsage::Subsequent("ab");
        let result = StoredCredentialUsage::try_from(input);

        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
