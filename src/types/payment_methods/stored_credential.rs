use crate::Error;
use crate::types::{CustomerId, StoredCredentialToken};

/// Stored payment credential for recurring payments (mandates, tokens, setup intents)
#[derive(Clone, Debug)]
pub struct StoredCredential {
    token: StoredCredentialToken,
    customer_id: Option<CustomerId>,
}

impl StoredCredential {
    /// Token representing this stored credential from a payment gateway
    #[inline]
    pub fn token(&self) -> &StoredCredentialToken {
        &self.token
    }

    /// Optional customer identifier associated with this stored credential
    ///
    /// Required for SEPA/ACH direct debit mandates where customer identity
    /// is part of the scheme. Not needed for card tokens which are self-contained.
    ///
    /// Typically set by the adapter when creating the credential, not returned by the gateway.
    /// In this case the adapter should add the customer ID from the request by itself.
    #[inline]
    pub fn customer_id(&self) -> Option<&CustomerId> {
        self.customer_id.as_ref()
    }
}

impl<'a> TryFrom<crate::StoredCredential<'a>> for StoredCredential {
    type Error = Error;

    fn try_from(input: crate::StoredCredential<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            token: input.token.try_into()?,
            customer_id: input.customer_id.map(TryInto::try_into).transpose()?,
        })
    }
}
