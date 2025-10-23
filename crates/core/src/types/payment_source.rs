use crate::internal::Exposed;
use crate::types::*;
use std::collections::HashMap;

/// Payment source used for transaction authorization
///
/// Each variant represents a distinct payment flow type with specific data requirements,
/// authentication models, and settlement characteristics. For detailed documentation
/// about each payment method, see the individual markdown files in this directory.
#[derive(Clone, Debug)]
pub enum PaymentSource {
    /// Direct bank account debit payment.
    ///
    /// Automated recurring debits from user's bank account based on pre-authorized mandate.
    /// Settlement takes 1-3 business days. Authentication occurs outside authorization flow
    /// through micro-deposits, instant verification, or signed mandate.
    ///
    /// **Use for:** Subscription billing, recurring payments, scheduled debits
    /// **Examples:** ACH (US), BACS (UK), PAD (Canada), GIRO (Singapore), Zengin (Japan)
    ///
    /// See [bank_account.md](payment_source/bank_account.md) for detailed documentation.
    BankAccount {
        /// Bank account number
        account_number: AccountNumber,
        /// User full name as registered with the bank account
        full_name: FullName,
        /// Bank routing identifier
        routing_number: RoutingNumber,

        /// Type of bank account (checking or savings)
        account_type: Option<AccountType>,
        /// Type of account holder (individual or company)
        account_holder_type: Option<CustomerCategory>,
        /// Method-specific extensions
        metadata: Option<Metadata>,
    },

    /// Buy Now Pay Later payment.
    ///
    /// User receives goods immediately, pays in installments (3-12 months). BNPL provider
    /// extends credit and assumes payment risk. Authentication through provider redirect and
    /// credit assessment.
    ///
    /// **Use for:** Installment payment options with gateway-managed credit checks
    /// **Examples:** Klarna, Afterpay, Affirm, Zip, Sezzle, Atome, Scalapay
    ///
    /// See [bnpl.md](payment_source/bnpl.md) for detailed documentation.
    BNPL {
        /// User billing address
        billing_address: Address,
        /// User email address
        email: EmailAddress,
        /// User full name
        full_name: FullName,

        /// Type of user (person or organization)
        customer_category: Option<CustomerCategory>,
        /// User date of birth
        date_of_birth: Option<BirthDate>,
        /// National identification number
        national_id: Option<NationalId>,
        /// User phone number
        phone: Option<PhoneNumber>,
        /// Method-specific extensions
        metadata: Option<Metadata>,
    },

    /// Cash-based voucher payment.
    ///
    /// User receives voucher with barcode, pays cash at physical locations (stores, banks, post offices).
    /// Asynchronous settlement (1-3 days). Authentication through physical payment confirmation.
    ///
    /// **Use for:** Markets with high cash usage, users without bank accounts/cards
    /// **Examples:** Boleto (Brazil), OXXO (Mexico), Konbini (Japan), Multibanco (Portugal)
    ///
    /// See [cash_voucher.md](payment_source/cash_voucher.md) for detailed documentation.
    CashVoucher {
        /// User full name
        full_name: FullName,
        /// User billing address
        billing_address: Option<Address>,
        /// National identification number (CPF/CNPJ for Boleto)
        national_id: Option<NationalId>,
        /// Method-specific extensions
        metadata: Option<Metadata>,
    },

    /// Cryptocurrency payment via blockchain transfer.
    ///
    /// Gateway generates deposit address, user sends crypto from any wallet. Irreversible transaction
    /// requiring blockchain confirmations (1-6 blocks). Authentication through on-chain cryptographic signature.
    ///
    /// **Use for:** Borderless payments, crypto-native users, when traditional rails unavailable
    /// **Examples:** Bitcoin, Ethereum, USDT, USDC via BitPay, Coinbase Commerce, NOWPayments
    ///
    /// See [crypto_payment.md](payment_source/crypto_payment.md) for detailed documentation.
    CryptoPayment {
        /// Currency/network selection and extensions
        metadata: Option<Metadata>,
    },

    /// Direct carrier billing payment.
    ///
    /// Payment charged to user's mobile phone bill. User authorizes via SMS PIN, charge appears
    /// on next phone bill. Authentication through SMS challenge-response with carrier verification.
    ///
    /// **Use for:** Mobile-first markets, digital content, users without cards
    /// **Examples:** Boku, Fortumo, Centili, carrier integrations (Verizon, Vodafone, MTN)
    ///
    /// See [direct_carrier_billing.md](payment_source/direct_carrier_billing.md) for detailed documentation.
    DirectCarrierBilling {
        /// User phone number (primary payment identifier)
        phone: PhoneNumber,
        /// Carrier-specific extensions
        metadata: Option<Metadata>,
    },

    /// Instant bank transfer payment.
    ///
    /// Real-time bank transfer with user redirect to bank for authorization. Immediate settlement
    /// (seconds to minutes). Authentication through bank login and transaction approval in bank UI.
    ///
    /// **Use for:** One-time payments needing immediate settlement and bank-level authentication
    /// **Examples:** PIX (Brazil), UPI (India), iDEAL (Netherlands), PSE (Colombia), SPEI (Mexico)
    ///
    /// See [instant_bank_transfer.md](payment_source/instant_bank_transfer.md) for detailed documentation.
    InstantBankTransfer {
        /// User email for transaction notifications
        email: EmailAddress,
        /// User full name as registered with bank
        full_name: FullName,

        /// Bank account number (CLABE for SPEI)
        account_number: Option<AccountNumber>,
        /// Bank identifier code
        bank_code: Option<BankCode>,
        /// User billing address
        billing_address: Option<Address>,
        /// Type of user (person or organization)
        customer_category: Option<CustomerCategory>,
        /// National identification number (tax ID)
        national_id: Option<NationalId>,
        /// User phone number
        phone: Option<PhoneNumber>,
        /// Virtual Payment Address (UPI)
        virtual_payment_address: Option<VirtualPaymentAddress>,
        /// Method-specific extensions
        metadata: Option<Metadata>,
    },

    /// Credit or debit card payment.
    ///
    /// Traditional payment card transaction where user provides card details for authorization.
    /// Most universal payment method worldwide. Authentication through CVV and optional 3D Secure.
    ///
    /// **Use for:** E-commerce card-not-present transactions, general-purpose payments
    /// **Examples:** Visa, Mastercard, Amex, Discover, JCB, UnionPay, regional schemes
    ///
    /// See [payment_card.md](payment_source/payment_card.md) for detailed documentation.
    PaymentCard {
        /// Card Verification Value (CVV/CVC/CID)
        cvv: CVV,
        /// Primary Account Number (PAN)
        number: PrimaryAccountNumber,
        /// Card expiration date (month and year)
        card_expiry: Option<CardExpiry>,
        /// Cardholder name as embossed on card
        holder_name: Option<CardHolderName>,
    },

    /// SEPA bank transfer payment.
    ///
    /// EUR-denominated transfers within 36 European countries. SEPA Instant provides real-time
    /// settlement (10 seconds), standard SEPA takes 1-2 business days. PSD2 SCA authentication.
    ///
    /// **Use for:** EUR payments within Europe requiring IBAN-based transfers
    /// **Examples:** SEPA Instant Credit Transfer, standard SEPA Direct Debit
    ///
    /// See [sepa_transfer.md](payment_source/sepa_transfer.md) for detailed documentation.
    SEPATransfer {
        /// User billing address (required per PSD2 AML)
        billing_address: Address,
        /// User email for transaction notifications
        email: EmailAddress,
        /// User full name as registered with bank
        full_name: FullName,
        /// International Bank Account Number
        iban: IBAN,
    },

    /// Tokenized payment method.
    ///
    /// Pre-authorized payment method represented by secure token. Token references stored payment method
    /// or session token from completed flow (Apple Pay, Alipay, etc.). Authentication embedded in token.
    ///
    /// **Use for:** Recurring payments, saved payment methods, device wallets, e-wallet redirects
    /// anonymous payments, cash-based purchasing, privacy-focused transactions
    /// **Examples:** Stored cards, Apple Pay, Google Pay, Alipay, WeChat Pay, network tokens,
    /// Paysafecard (direct entry), Neosurf, Flexepin, CASHlib, AstroPay
    ///
    /// See [tokenized_payment.md](payment_source/tokenized_payment.md) for detailed documentation.
    TokenizedPayment {
        /// Payment method token encapsulating payment details and user information
        token: Token,
    },
}

// SAFETY: The trait is safely implemented as:
// 1. it uses exposed versions of all inner types,
// 2. it uses `Debug` implementations of its values, that mask sensitive data by themselves.
unsafe impl Exposed for PaymentSource {
    type Output<'a> = ExposedPaymentSource<'a>;
    const TYPE_WRAPPER: &'static str = "PaymentSource";

    fn expose<'a>(&'a self) -> Self::Output<'a> {
        match self {
            Self::BankAccount {
                full_name,
                account_number,
                routing_number,
                account_type,
                account_holder_type,
                metadata,
            } => Self::Output::BankAccount {
                full_name: full_name.expose(),
                account_number: account_number.expose(),
                routing_number: routing_number.expose(),
                account_type: *account_type,
                account_holder_type: account_holder_type.clone(),
                metadata: metadata.as_ref().map(Exposed::expose),
            },

            Self::BNPL {
                billing_address,
                email,
                full_name,
                customer_category,
                date_of_birth,
                national_id,
                phone,
                metadata,
            } => Self::Output::BNPL {
                billing_address: billing_address.expose(),
                email: email.expose(),
                full_name: full_name.expose(),
                customer_category: customer_category.clone(),
                date_of_birth: date_of_birth.as_ref().map(Exposed::expose),
                national_id: national_id.as_ref().map(Exposed::expose),
                phone: phone.as_ref().map(Exposed::expose),
                metadata: metadata.as_ref().map(Exposed::expose),
            },

            Self::CashVoucher {
                full_name,
                billing_address,
                national_id,
                metadata,
            } => Self::Output::CashVoucher {
                full_name: full_name.expose(),
                billing_address: billing_address.as_ref().map(Exposed::expose),
                national_id: national_id.as_ref().map(Exposed::expose),
                metadata: metadata.as_ref().map(Exposed::expose),
            },

            Self::CryptoPayment { metadata } => Self::Output::CryptoPayment {
                metadata: metadata.as_ref().map(Exposed::expose),
            },

            Self::DirectCarrierBilling { phone, metadata } => Self::Output::DirectCarrierBilling {
                phone: phone.expose(),
                metadata: metadata.as_ref().map(Exposed::expose),
            },

            Self::InstantBankTransfer {
                email,
                full_name,
                account_number,
                bank_code,
                billing_address,
                customer_category,
                national_id,
                phone,
                virtual_payment_address,
                metadata,
            } => Self::Output::InstantBankTransfer {
                email: email.expose(),
                full_name: full_name.expose(),
                account_number: account_number.as_ref().map(Exposed::expose),
                bank_code: bank_code.as_ref().map(AsRef::as_ref),
                billing_address: billing_address.as_ref().map(Exposed::expose),
                customer_category: customer_category.clone(),
                national_id: national_id.as_ref().map(Exposed::expose),
                phone: phone.as_ref().map(Exposed::expose),
                virtual_payment_address: virtual_payment_address.as_ref().map(Exposed::expose),
                metadata: metadata.as_ref().map(Exposed::expose),
            },

            Self::PaymentCard {
                cvv,
                number,
                card_expiry,
                holder_name,
            } => Self::Output::PaymentCard {
                cvv: cvv.expose(),
                number: number.expose(),
                card_expiry: card_expiry.as_ref().map(Exposed::expose),
                holder_name: holder_name.as_ref().map(Exposed::expose),
            },

            Self::SEPATransfer {
                billing_address,
                email,
                full_name,
                iban,
            } => Self::Output::SEPATransfer {
                billing_address: billing_address.expose(),
                email: email.expose(),
                full_name: full_name.expose(),
                iban: iban.expose(),
            },

            Self::TokenizedPayment { token } => Self::Output::TokenizedPayment {
                token: token.expose(),
            },
        }
    }
}

#[derive(Clone)]
pub(crate) enum ExposedPaymentSource<'a> {
    BankAccount {
        account_number: &'a str,
        full_name: &'a str,
        routing_number: &'a str,
        account_type: Option<AccountType>,
        account_holder_type: Option<CustomerCategory>,
        metadata: Option<HashMap<&'static str, &'a str>>,
    },
    #[allow(clippy::upper_case_acronyms)]
    BNPL {
        billing_address: ExposedAddress<'a>,
        email: &'a str,
        full_name: &'a str,
        customer_category: Option<CustomerCategory>,
        date_of_birth: Option<ExposedBirthDate<'a>>,
        national_id: Option<&'a str>,
        phone: Option<&'a str>,
        metadata: Option<HashMap<&'static str, &'a str>>,
    },
    CashVoucher {
        full_name: &'a str,
        billing_address: Option<ExposedAddress<'a>>,
        national_id: Option<&'a str>,
        metadata: Option<HashMap<&'static str, &'a str>>,
    },
    CryptoPayment {
        metadata: Option<HashMap<&'static str, &'a str>>,
    },
    DirectCarrierBilling {
        phone: &'a str,
        metadata: Option<HashMap<&'static str, &'a str>>,
    },
    InstantBankTransfer {
        email: &'a str,
        full_name: &'a str,
        account_number: Option<&'a str>,
        bank_code: Option<&'a str>,
        billing_address: Option<ExposedAddress<'a>>,
        customer_category: Option<CustomerCategory>,
        national_id: Option<&'a str>,
        phone: Option<&'a str>,
        virtual_payment_address: Option<&'a str>,
        metadata: Option<HashMap<&'static str, &'a str>>,
    },
    PaymentCard {
        cvv: &'a str,
        number: &'a str,
        card_expiry: Option<ExposedCardExpiry<'a>>,
        holder_name: Option<&'a str>,
    },
    SEPATransfer {
        billing_address: ExposedAddress<'a>,
        email: &'a str,
        full_name: &'a str,
        iban: &'a str,
    },
    TokenizedPayment {
        token: &'a str,
    },
}
