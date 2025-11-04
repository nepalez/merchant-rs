use std::convert::TryFrom;

use crate::Error;
use crate::inputs::{SEPA as Input, SEPACredentials as CredentialsInput};
use crate::types::{
    Address, Credentials, EmailAddress, FullName, IBAN, InternalPaymentMethod, PaymentMethod,
    TokenizablePaymentMethod,
};

/// SEPA Bank Account
///
/// ## Overview
///
/// Bank transfer within the Single Euro Payments Area for EUR-denominated transactions.
/// Enables direct transfers between bank accounts across 36 European countries with unified standards.
/// SEPA Instant provides real-time settlement (10 seconds), standard SEPA takes 1–2 business days.
/// Uses IBAN as the primary account identifier.
///
/// ## Authentication Model
///
/// > Authentication model **depends on SEPA variant**!
///
/// ### SEPA Instant Credit Transfer
/// - **PSD2 Strong Customer Authentication**: Customer redirected to bank for SCA
/// - **Similar to InstantBankTransfer**: Bank login and transaction approval
/// - **Real-time settlement**: 10 seconds maximum
/// - **24/7 availability**: No banking hours restrictions
///
/// ### Standard SEPA Direct Debit
/// - **Pre-authorized mandate**: Customer signs SEPA Direct Debit mandate
/// - **No authentication in authorization request**: Mandate authorizes recurring debits
/// - **Settlement**: 1–2 business days
/// - **Customer protection**: 8-week dispute window for unauthorized debits
///
/// The authorization request contains **only account identification and customer data**.
/// For SEPA Instant, authentication happens in a redirect flow.
/// For SEPA Debit, authentication occurred during mandate setup.
///
/// ## Standards
///
/// - **[ISO 20022](https://www.iso20022.org/)**: XML message format for SEPA payments
/// - **[ISO 13616](https://www.iso.org/standard/81090.html)**: IBAN (International Bank Account Number) standard
/// - **[PSD2](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32015L2366)**: Payment Services Directive (AML/KYC requirements)
/// - **[EPC SEPA Instant Credit Transfer Scheme](https://www.europeanpaymentscouncil.eu/what-we-do/sepa-instant-credit-transfer)**: European Payment Council specifications
///
/// ## SEPA Zone Coverage
///
/// 36 countries participate in SEPA:
///
/// ### EU Member States (27)
/// Austria, Belgium, Bulgaria, Croatia, Cyprus, the Czech Republic, Denmark, Estonia, Finland, France,
/// Germany, Greece, Hungary, Ireland, Italy, Latvia, Lithuania, Luxembourg, Malta, the Netherlands,
/// Poland, Portugal, Romania, Slovakia, Slovenia, Spain, Sweden
///
/// ### EEA Countries (3)
/// Iceland, Liechtenstein, Norway
///
/// ### Other (6)
/// Andorra, Monaco, San Marino, Switzerland, United Kingdom, Vatican City
///
/// ## Security Considerations
///
/// ### IBAN Handling
/// - IBAN is not classified as Sensitive Authentication Data per PCI DSS
/// - However, it is critical PII and financial access data
/// - Use `IBAN` type which implements appropriate protection
/// - Validate IBAN format and check digit per ISO 13616
///
/// ### PSD2 Compliance
/// - **Strong Customer Authentication (SCA)**: Required for SEPA Instant in most cases
/// - **AML/KYC requirements**: Billing address required for AML compliance
/// - **Customer rights**: 8-week dispute window for SEPA Direct Debit
/// - **Data protection**: Comply with GDPR for customer data
///
/// ### Fraud Prevention
/// - Validate IBAN format and check digit
/// - Verify IBAN belongs to SEPA zone
/// - Check account holder name matching (where supported)
/// - Monitor for unusual patterns
/// - Implement velocity limits
///
/// ### Mandate Management (SEPA Debit)
/// - Store mandate reference ID
/// - Track mandate status (active, canceled, expired)
/// - Provide pre-notification before each debit
/// - Handle mandate cancellations
/// - Respect the 8-week dispute window
#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct SEPA {
    /// International Bank Account Number
    pub credentials: Credentials<SEPACredentials>,
    /// User billing address (required per PSD2 AML)
    pub billing_address: Address,
    /// User email for transaction notifications
    pub email: EmailAddress,
    /// User full name as registered with bank
    pub full_name: FullName,
}

#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct SEPACredentials {
    /// International Bank Account Number
    pub iban: IBAN,
}

// Marker implementations

impl PaymentMethod for SEPA {}
impl InternalPaymentMethod for SEPA {}
impl TokenizablePaymentMethod for SEPA {}

impl<'a> TryFrom<Input<'a>> for SEPA {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            credentials: input.credentials.try_into()?,
            email: input.email.try_into()?,
            billing_address: input.billing_address.try_into()?,
            full_name: input.full_name.try_into()?,
        })
    }
}

impl<'a> TryFrom<CredentialsInput<'a>> for SEPACredentials {
    type Error = Error;

    fn try_from(input: CredentialsInput<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            iban: input.iban.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;

    fn valid_input_plain() -> Input<'static> {
        inputs::SEPA {
            credentials: inputs::Credentials::Plain(inputs::SEPACredentials {
                iban: " DE89370400440532013000 \n\t",
            }),
            email: " user@example.com \n\t",
            billing_address: inputs::Address {
                country_code: " DE \n\t",
                postal_code: " 10115 \n\t",
                city: " Berlin \n\t",
                line: " Hauptstrasse 1 \n\t",
            },
            full_name: " john doe \n\t",
        }
    }

    fn valid_input_tokenized() -> Input<'static> {
        inputs::SEPA {
            credentials: inputs::Credentials::Tokenized("tok_sepa1234567890"),
            email: " user@example.com \n\t",
            billing_address: inputs::Address {
                country_code: " DE \n\t",
                postal_code: " 10115 \n\t",
                city: " Berlin \n\t",
                line: " Hauptstrasse 1 \n\t",
            },
            full_name: " john doe \n\t",
        }
    }

    #[test]
    fn constructed_from_valid_input_plain() {
        let input = valid_input_plain();
        let sepa = SEPA::try_from(input).unwrap();

        match sepa.credentials {
            Credentials::Plain(creds) => unsafe {
                assert_eq!(creds.iban.as_ref(), "DE89370400440532013000");
                assert_eq!(sepa.email.as_ref(), "user@example.com");
                assert_eq!(sepa.billing_address.country_code.as_ref(), "DE");
                assert_eq!(sepa.billing_address.postal_code.as_ref(), "10115");
                assert_eq!(sepa.billing_address.city.as_ref(), "Berlin");
                assert_eq!(sepa.billing_address.line.as_ref(), "Hauptstrasse 1");
                assert_eq!(sepa.full_name.as_ref(), "JOHN DOE");
            },
            Credentials::Tokenized(_) => panic!("Expected Plain credentials"),
        }
    }

    #[test]
    fn constructed_from_valid_input_tokenized() {
        let input = valid_input_tokenized();
        let sepa = SEPA::try_from(input).unwrap();

        match sepa.credentials {
            Credentials::Tokenized(token) => unsafe {
                assert_eq!(token.as_ref(), "tok_sepa1234567890");
                assert_eq!(sepa.email.as_ref(), "user@example.com");
                assert_eq!(sepa.full_name.as_ref(), "JOHN DOE");
            },
            Credentials::Plain(_) => panic!("Expected Tokenized credentials"),
        }
    }

    #[test]
    fn rejects_invalid_iban() {
        let mut input = valid_input_plain();
        if let inputs::Credentials::Plain(ref mut creds) = input.credentials {
            creds.iban = "invalid";
        }

        let result = SEPA::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_email() {
        let mut input = valid_input_plain();
        input.email = "invalid";

        let result = SEPA::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_full_name() {
        let mut input = valid_input_plain();
        input.full_name = "X";

        let result = SEPA::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
