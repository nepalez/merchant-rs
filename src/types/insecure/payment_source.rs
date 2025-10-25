use crate::types::{AccountType, CustomerCategory, insecure::*};

/// Payment source authorization data.
///
/// ```skip
/// let source = PaymentSource::BankAccount {
///     account_number: "1234-567890-77",
///     full_name: "João Cabral de Melo Neto",
///     routing_number: "7372 8979 9997 7749",
///     account_type: Some(AccountType::Savings),
///     account_holder_type: Some(CustomerCategory::Individual),
///     metadata: None,
/// }.try_into()?;
/// ```
pub enum PaymentSource<'a> {
    BankAccount {
        account_number: AccountNumber<'a>,
        full_name: FullName<'a>,
        routing_number: RoutingNumber<'a>,
        account_type: Option<AccountType>,
        account_holder_type: Option<CustomerCategory>,
        metadata: Option<Metadata<'a>>,
    },
    #[allow(clippy::upper_case_acronyms)]
    BNPL {
        billing_address: Address<'a>,
        email: EmailAddress<'a>,
        full_name: FullName<'a>,
        customer_category: Option<CustomerCategory>,
        date_of_birth: Option<BirthDate>,
        national_id: Option<NationalId<'a>>,
        phone: Option<PhoneNumber<'a>>,
        metadata: Option<Metadata<'a>>,
    },
    CashVoucher {
        full_name: FullName<'a>,
        billing_address: Option<Address<'a>>,
        national_id: Option<NationalId<'a>>,
        metadata: Option<Metadata<'a>>,
    },
    CryptoPayment {
        metadata: Option<Metadata<'a>>,
    },
    DirectCarrierBilling {
        phone: PhoneNumber<'a>,
        metadata: Option<Metadata<'a>>,
    },
    InstantBankTransfer {
        email: EmailAddress<'a>,
        full_name: FullName<'a>,
        account_number: Option<AccountNumber<'a>>,
        bank_code: Option<BankCode<'a>>,
        billing_address: Option<Address<'a>>,
        customer_category: Option<CustomerCategory>,
        national_id: Option<NationalId<'a>>,
        phone: Option<PhoneNumber<'a>>,
        virtual_payment_address: Option<VirtualPaymentAddress<'a>>,
        metadata: Option<Metadata<'a>>,
    },
    PaymentCard {
        cvv: CVV<'a>,
        number: PrimaryAccountNumber<'a>,
        card_expiry: Option<CardExpiry>,
        holder_name: Option<CardHolderName<'a>>,
    },
    SEPATransfer {
        billing_address: Address<'a>,
        email: EmailAddress<'a>,
        full_name: FullName<'a>,
        iban: IBAN<'a>,
    },
    TokenizedPayment {
        token: Token<'a>,
    },
}
