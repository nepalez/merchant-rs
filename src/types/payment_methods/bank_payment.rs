use std::convert::TryFrom;

use crate::Error;
use crate::inputs::{BankPayment as Input, BankPaymentCredentials as CredentialsInput};
use crate::types::{
    AccountHolderType, AccountNumber, AccountType, Credentials, FullName, InternalPaymentMethod,
    Metadata, PaymentMethod, RoutingNumber, TokenizablePaymentMethod,
};

/// Direct Bank Account
///
/// ## Overview
///
/// Automated recurring debits from a customer's bank account based on a pre-authorized mandate.
/// Funds are pulled by the merchant on a schedule (subscriptions, utility bills, loan payments).
/// Settlement typically takes 1–3 business days.
/// Customer must provide explicit authorization through mandate setup.
///
/// ## When to Use
///
/// - **Subscription billing**: Monthly/annual recurring charges
/// - **Utility bills**: Electricity, water, internet, phone services
/// - **Loan payments**: Mortgage, car loans, student loans
/// - **Membership fees**: Gym memberships, professional associations
/// - **Insurance premiums**: Regular insurance payments
///
/// ## Authentication Model
///
/// > Authentication occurs **outside** the authorization flow, not during a transaction!
///
/// ### Pre-Authorization Methods
///
/// 1. **Micro-deposits verification** (Stripe, Plaid):
/// - Gateway sends two small deposits (e.g., $0.32 and $0.45) to the customer's account
/// - Customer verifies amounts in their bank statement
/// - Customer confirms amounts to gateway
/// - Proves the customer controls the account
///
/// 2. **Instant verification** (Plaid, Yodlee, Tink):
/// - Customer redirected to bank aggregator
/// - Customer logs into a bank through the aggregator's secure interface
/// - Aggregator confirms account ownership and validity
/// - Instant verification without waiting for deposits
///
/// 3. **Mandate authorization**:
/// - Customer signs a direct debit mandate (electronic or physical)
/// - Mandate authorizes merchant to initiate debits
/// - Mandate stored by gateway and/or merchant
/// - NACHA rules (US), Bacs rules (UK), etc. govern mandate requirements
///
/// ### Authorization Flow
///
/// The authorization request contains **only account identification data**,
/// not authentication credentials. Authentication already occurred during mandate setup.
/// Gateway verifies:
/// - Mandate exists and is valid
/// - Account is active
/// - Sufficient funds available (optional pre-notification)
///
/// ## Standards
///
/// - **NACHA Operating Rules**: National Automated Clearing House Association (United States ACH)
/// - **Bacs Payment Schemes**: Direct Debit scheme (United Kingdom)
/// - **Payments Canada Rule H1**: Pre-Authorized Debit (PAD) framework
/// - **[EFT Code of Conduct](https://www.asic.gov.au/regulatory-remethods/financial-services/eft-code-of-conduct/)**: Electronic Funds Transfer (Australia)
/// - **GIRO**: Interbank GIRO system (Singapore)
/// - **CNP Standards**: China National Payment System
/// - **Zengin System**: Japanese bank clearing network
///
/// ## Example Systems
///
/// ### North America
/// - **ACH** (United States): NACHA rules, 1-2-day settlement
/// - **PAD** (Canada): Payments Canada Rule H1, 1-2-day settlement
/// - **AFT** (Mexico): Automated Funds Transfer
///
/// ### Europe
/// - **BACS** (UK): 3-day settlement, Direct Debit Guarantee
/// - **Lastschrift** (Germany): SEPA Direct Debit variant
/// - **Incasso** (Netherlands): SEPA Direct Debit variant
///
/// ### Asia Pacific
/// - **GIRO** (Singapore): Automated clearing house
/// - **Zengin** (Japan): Japanese bank clearing network
/// - **eNETS** (Singapore): Electronic Network for EFT Services
///
/// ### Other
/// - **EFT** (Australia): ASIC regulated
/// - **Autogiro** (Sweden): Swedish direct debit
/// - **Betalingsservice** (Denmark): Danish direct debit
///
/// ## Security Considerations
///
/// ### PCI DSS Compliance
/// Bank account numbers are **not** classified as Sensitive Authentication Data (SAD) under PCI DSS. However, they are critical PII and financial access data.
///
/// ### Fraud Prevention
/// - Verify account ownership through micro-deposits or instant verification
/// - Validate routing numbers against known bank databases
/// - Check account holder name matching
/// - Monitor for unusual patterns (multiple attempts, rapid changes)
/// - Implement velocity limits on failed attempts
///
/// ### Compliance
/// - **NACHA**: Must comply with Operating Rules, including authorization requirements
/// - **Bacs**: Must be registered Service User, follow Direct Debit Guarantee
/// - **PSD2** (Europe): AML/KYC requirements for account verification
/// - **GDPR**: Bank account data is PII, must follow data protection regulations
#[derive(Clone, Debug)]
pub struct BankPayment {
    /// The tokenizable credentials of the account
    pub credentials: Credentials<BankPaymentCredentials>,
    /// User full name as registered with the bank account
    pub full_name: FullName,
    /// Type of bank account (checking or savings)
    pub account_type: AccountType,
    /// Type of account holder (individual or company)
    pub holder_type: AccountHolderType,
    /// Method-specific extensions
    pub metadata: Option<Metadata>,
}

#[derive(Clone, Debug)]
pub struct BankPaymentCredentials {
    /// The bank account number.
    pub account_number: AccountNumber,
    /// Bank routing identifier.
    pub routing_number: RoutingNumber,
}

// Marker implementations

impl PaymentMethod for BankPayment {}
impl InternalPaymentMethod for BankPayment {}
impl TokenizablePaymentMethod for BankPayment {}

impl<'a> TryFrom<CredentialsInput<'a>> for BankPaymentCredentials {
    type Error = Error;

    fn try_from(input: CredentialsInput<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            account_number: input.account_number.try_into()?,
            routing_number: input.routing_number.try_into()?,
        })
    }
}

impl TryFrom<Input<'_>> for BankPayment {
    type Error = Error;

    fn try_from(input: Input<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            credentials: input.credentials.try_into()?,
            full_name: input.full_name.try_into()?,
            account_type: input.account_type,
            holder_type: input.holder_type,
            metadata: input.metadata.map(TryFrom::try_from).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;

    fn valid_input_plain() -> Input<'static> {
        inputs::BankPayment {
            credentials: inputs::Credentials::Plain(inputs::BankPaymentCredentials {
                account_number: " 1234567890 \n\t",
                routing_number: " 123456789 \n\t",
            }),
            full_name: " john doe \n\t",
            account_type: AccountType::Checking,
            holder_type: AccountHolderType::Individual,
            metadata: None,
        }
    }

    fn valid_input_tokenized() -> Input<'static> {
        inputs::BankPayment {
            credentials: inputs::Credentials::Tokenized("tok_bank1234567890"),
            full_name: " john doe \n\t",
            account_type: AccountType::Savings,
            holder_type: AccountHolderType::Company,
            metadata: None,
        }
    }

    #[test]
    fn constructed_from_valid_input_plain() {
        let input = valid_input_plain();
        let bank_payment = BankPayment::try_from(input).unwrap();

        match bank_payment.credentials {
            Credentials::Plain(creds) => unsafe {
                assert_eq!(creds.account_number.as_ref(), "1234567890");
                assert_eq!(creds.routing_number.as_ref(), "123456789");
                assert_eq!(bank_payment.full_name.as_ref(), "JOHN DOE");
                assert!(bank_payment.metadata.is_none());
            },
            Credentials::Tokenized(_) => panic!("Expected Plain credentials"),
        }
    }

    #[test]
    fn constructed_from_valid_input_tokenized() {
        let input = valid_input_tokenized();
        let bank_payment = BankPayment::try_from(input).unwrap();

        match bank_payment.credentials {
            Credentials::Tokenized(token) => unsafe {
                assert_eq!(token.as_ref(), "tok_bank1234567890");
                assert_eq!(bank_payment.full_name.as_ref(), "JOHN DOE");
                assert!(bank_payment.metadata.is_none());
            },
            Credentials::Plain(_) => panic!("Expected Tokenized credentials"),
        }
    }

    #[test]
    fn rejects_invalid_account_number() {
        let mut input = valid_input_plain();
        if let inputs::Credentials::Plain(ref mut creds) = input.credentials {
            creds.account_number = "123";
        }

        let result = BankPayment::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_routing_number() {
        let mut input = valid_input_plain();
        if let inputs::Credentials::Plain(ref mut creds) = input.credentials {
            creds.routing_number = "12345";
        }

        let result = BankPayment::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_full_name() {
        let mut input = valid_input_plain();
        input.full_name = "X";

        let result = BankPayment::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
