use std::convert::TryFrom;

use super::{ExternalPaymentMethod, PaymentMethod};
use crate::Error;
use crate::inputs::CashVoucher as Input;
use crate::types::{Address, FullName, Metadata, NationalId};

/// Cash-Based Voucher
///
/// ## Overview
///
/// Customer receives a voucher with a barcode or reference number, then pays with cash at physical locations
/// (convenience stores, banks, post offices). Settlement is asynchronous—payment confirmation arrives via
/// webhook after the customer completes payment at a physical location (typically 1–3 days, up to voucher expiration).
/// Gateway generates the voucher, customer takes it to the payment location.
///
/// ## Authentication Model
///
/// > Authentication occurs **through physical payment**, not in authorization request!
///
/// ### Voucher Generation and Payment Flow
///
/// 1. **Merchant initiates**: Calls `authorize()` with customer data
/// 2. **Gateway generates voucher**: Creates unique voucher with barcode and reference number
/// 3. **Customer receives voucher**: Via email (PDF) or displayed on screen
/// 4. **Customer goes to location**: Physically visits convenience store, bank, or post office
/// 5. **Customer presents voucher**: Shows barcode or provides reference number to cashier
/// 6. **Cash payment**: Customer pays cash to cashier
/// 7. **Receipt confirmation**: Payment location confirms transaction to gateway
/// 8. **Webhook notification**: Gateway notifies merchant asynchronously (1–3 days)
///
/// ### Authorization Request Content
///
/// The authorization request contains **customer data for voucher generation and compliance**,
/// not authentication credentials. Authentication is inherently physical — a customer's presence
/// at a payment location with cash.
///
/// ## Standards
///
/// - **[FEBRABAN Standards](https://portal.febraban.org.br/)**: Brazilian Federation of Banks (Boleto specifications)
/// - **OXXO Specifications**: Mexico convenience store payment standards
/// - **Konbini Standards**: Japan convenience store payment system
/// - **Multibanco Standards**: Portuguese ATM/retail payment network
///
/// ## Example Systems
///
/// ### Latin America
/// - **Boleto Bancário** (Brazil): Most popular payment method in Brazil, 3-day validity typical, requires CPF/CNPJ
/// - **OXXO** (Mexico): 18,000+ locations, cash payment at convenience stores
/// - **PagoEfectivo** (Peru): Bank branches and payment agents
/// - **Efecty** (Colombia): Payment network with 10,000+ locations
/// - **Servipag** (Chile): Bill payment and cash collection network
///
/// ### Asia Pacific
/// - **Konbini** (Japan): 7-Eleven, FamilyMart, Lawson stores, 30-day validity
/// - **7-Eleven** (various): Cash payment at 7-Eleven stores in multiple countries
/// - **Alfamart/Indomaret** (Indonesia): Major retail chains accepting cash vouchers
///
/// ### Europe
/// - **Multibanco** (Portugal): ATM and retail payment, widely used in Portugal
/// - **Barzahlen** (Germany/Austria): Cash payment at retail partners
///
/// ### Middle East/Africa
/// - **Fawry** (Egypt): Payment at retail outlets and mobile wallets
/// - **ePay** (various): Multi-country cash voucher network
///
/// ## Security Considerations
///
/// ### Customer Data Protection
/// - Full name, email, address, and optionally tax ID (CPF/CNPJ for Boleto)
/// - All PII must be protected per GDPR, LGPD, or local regulations
/// - Use appropriate types with masking where required
/// - Tax IDs (CPF/CNPJ) should use `NationalId` type with automatic memory zeroization
///
/// ### Voucher Security
/// - Unique voucher codes prevent duplicate payments
/// - Barcodes designed to prevent counterfeiting
/// - Expiration dates limit a fraud window (typically 1–7 days)
/// - Payment location validates voucher before accepting cash
///
/// ### Fraud Prevention
/// - Validate tax ID format (CPF/CNPJ check digits for Boleto)
/// - Monitor for duplicate voucher generation attempts
/// - Track voucher usage patterns
/// - Implement expiration to limit a fraud window
/// - Verify customer email for voucher delivery
///
/// ### Compliance
/// - **Boleto regulations** (Brazil): CPF/CNPJ required for identification and tax reporting
/// - **AML/KYC**: Some jurisdictions require customer identification
/// - **Tax reporting**: Transaction records for tax authorities
/// - **LGPD/GDPR**: Customer data protection requirements
/// - **Consumer protection**: Clear expiration dates and payment instructions
#[derive(Clone, Debug)]
pub struct CashVoucher {
    pub(crate) full_name: FullName,
    pub(crate) billing_address: Option<Address>,
    pub(crate) national_id: Option<NationalId>,
    pub(crate) metadata: Option<Metadata>,
}

impl CashVoucher {
    /// User full name
    #[inline]
    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }

    /// User billing address
    #[inline]
    pub fn billing_address(&self) -> Option<&Address> {
        self.billing_address.as_ref()
    }

    /// National identification number (CPF/CNPJ for Boleto)
    #[inline]
    pub fn national_id(&self) -> Option<&NationalId> {
        self.national_id.as_ref()
    }

    /// Method-specific extensions
    #[inline]
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
}

// Marker implementations

impl PaymentMethod for CashVoucher {}
impl ExternalPaymentMethod for CashVoucher {}

impl<'a> TryFrom<Input<'a>> for CashVoucher {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            full_name: input.full_name.try_into()?,
            billing_address: input.billing_address.map(TryFrom::try_from).transpose()?,
            national_id: input.national_id.map(TryFrom::try_from).transpose()?,
            metadata: input.metadata.map(TryFrom::try_from).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;

    fn valid_input() -> Input<'static> {
        inputs::CashVoucher {
            full_name: " john doe \n\t",
            billing_address: Some(inputs::Address {
                country_code: " BR \n\t",
                postal_code: " 01310-100 \n\t",
                city: " Sao Paulo \n\t",
                line: " Av Paulista 1578 \n\t",
            }),
            national_id: Some(" 12345678901 \n\t"),
            metadata: None,
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let voucher = CashVoucher::try_from(input).unwrap();

        unsafe {
            assert_eq!(voucher.full_name.as_ref(), "JOHN DOE");
            assert!(voucher.billing_address.is_some());
            if let Some(ref national_id) = voucher.national_id {
                assert_eq!(national_id.as_ref(), "12345678901");
            }
            assert!(voucher.metadata.is_none());
        }
    }

    #[test]
    fn rejects_invalid_full_name() {
        let mut input = valid_input();
        input.full_name = "X";

        let result = CashVoucher::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_national_id() {
        let mut input = valid_input();
        input.national_id = Some("12");

        let result = CashVoucher::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_billing_address() {
        let mut input = valid_input();
        if let Some(ref mut address) = input.billing_address {
            address.city = "";
        }

        let result = CashVoucher::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
