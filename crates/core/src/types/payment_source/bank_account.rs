use std::collections::HashMap;
use std::str::FromStr;

use crate::Error;
use crate::types::{
    AccountNumber, AccountType, CustomerCategory, FullName, Metadata, PaymentSource, RoutingNumber,
};

/// Internal structure to build the bank account payment source.
#[derive(Default)]
pub(crate) struct Builder {
    account_number: Option<AccountNumber>,
    full_name: Option<FullName>,
    routing_number: Option<RoutingNumber>,
    account_type: Option<AccountType>,
    account_holder_type: Option<CustomerCategory>,
    metadata: Option<Metadata>,
}

impl Builder {
    #[inline]
    pub fn account_number(mut self, input: &str) -> Result<Self, Error> {
        self.account_number = Some(AccountNumber::from_str(input)?);
        Ok(self)
    }

    #[inline]
    pub fn full_name(mut self, input: &str) -> Result<Self, Error> {
        self.full_name = Some(FullName::from_str(input)?);
        Ok(self)
    }

    #[inline]
    pub fn routing_number(mut self, input: &str) -> Result<Self, Error> {
        self.routing_number = Some(RoutingNumber::from_str(input)?);
        Ok(self)
    }

    #[inline]
    pub fn account_type(mut self, input: AccountType) -> Result<Self, Error> {
        self.account_type = Some(input);
        Ok(self)
    }

    #[inline]
    pub fn holder_type(mut self, input: CustomerCategory) -> Result<Self, Error> {
        self.account_holder_type = Some(input);
        Ok(self)
    }

    #[inline]
    pub fn metadata(mut self, key: &'static str, input: &str) -> Result<Self, Error> {
        self.metadata.get_or_insert_default().insert(key, input)?;
        Ok(self)
    }

    #[inline]
    pub fn build(self) -> Result<PaymentSource, Error> {
        let Some(account_number) = self.account_number else {
            Err(Error::validation_failed(
                "account_number is missed".to_string(),
            ))?
        };
        let Some(full_name) = self.full_name else {
            Err(Error::validation_failed("full_name is missed".to_string()))?
        };
        let Some(routing_number) = self.routing_number else {
            Err(Error::validation_failed(
                "routing_number is missed".to_string(),
            ))?
        };

        Ok(PaymentSource::BankAccount {
            account_number,
            full_name,
            routing_number,
            account_type: self.account_type,
            account_holder_type: self.account_holder_type,
            metadata: self.metadata,
        })
    }
}
