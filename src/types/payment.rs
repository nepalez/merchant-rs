use std::convert::TryFrom;

use iso_currency::Currency;

use crate::Error;
use crate::inputs::Payment as Input;
use crate::types::{
    Destinations, MerchantInitiatedType, PaymentMethod, StoredCredentialUsage,
    TransactionIdempotenceKey,
};

/// Payment data with a raw payment method for direct processing.
///
/// Contains the payment method (e.g., CreditCard, BankAccount) along with transaction metadata
/// such as destination, currency, idempotence key, and merchant-initiated transaction type.
///
/// Used for first-time payments where the customer provides payment details directly,
/// as opposed to tokenized payments using stored credentials.
///
/// # Type Parameter
///
/// * `Method` - The payment method type constrained by PaymentMethod marker trait
#[derive(Debug, Clone)]
#[allow(private_bounds)]
pub struct Payment<Method: PaymentMethod> {
    pub(crate) method: Method,
    pub(crate) currency: Currency,
    pub(crate) destinations: Destinations,
    pub(crate) idempotence_key: TransactionIdempotenceKey,
    pub(crate) merchant_initiated_type: Option<MerchantInitiatedType>,
    pub(crate) stored_credential_usage: Option<StoredCredentialUsage>,
}

#[allow(private_bounds)]
impl<Method: PaymentMethod> Payment<Method> {
    /// The method of the payment to charge funds from
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// The currency for this payment
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// The payment destinations (platform or split between recipients)
    pub fn destinations(&self) -> &Destinations {
        &self.destinations
    }

    /// The idempotence key that can be used to retrieve the transaction id,
    /// and prevent duplication.
    pub fn idempotence_key(&self) -> &TransactionIdempotenceKey {
        &self.idempotence_key
    }

    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    pub fn merchant_initiated_type(&self) -> &Option<MerchantInitiatedType> {
        &self.merchant_initiated_type
    }

    /// Indicates whether this is the first or later use of stored credentials.
    /// Use `None` for one-time payments where credentials are not stored.
    /// Required for Credential-on-File (COF) compliance with card networks.
    pub fn stored_credential_usage(&self) -> &Option<StoredCredentialUsage> {
        &self.stored_credential_usage
    }
}

impl<'a, InputMethod, Method> TryFrom<Input<'a, InputMethod>> for Payment<Method>
where
    InputMethod: 'a,
    Method: PaymentMethod + TryFrom<InputMethod, Error = Error>,
{
    type Error = Error;

    fn try_from(input: Input<'a, InputMethod>) -> Result<Self, Self::Error> {
        Ok(Self {
            method: input.method.try_into()?,
            currency: input.currency,
            destinations: input.destinations.try_into()?,
            idempotence_key: input.idempotence_key.try_into()?,
            merchant_initiated_type: input.merchant_initiated_type,
            stored_credential_usage: input.stored_credential_usage,
        })
    }
}

// TODO: Update tests after inputs are updated
// #[cfg(test)]
// mod tests { ... }
