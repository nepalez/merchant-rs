use crate::inputs::{Address, Metadata};

pub struct CashVoucher<'a> {
    /// User full name
    pub full_name: &'a str,
    /// User billing address
    pub billing_address: Option<Address<'a>>,
    /// National identification number (CPF/CNPJ for Boleto)
    pub national_id: Option<&'a str>,
    /// Method-specific extensions
    pub metadata: Option<Metadata<'a>>,
}
