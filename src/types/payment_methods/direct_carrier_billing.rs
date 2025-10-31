use std::convert::TryFrom;

use crate::error::Error;
use crate::inputs::DirectCarrier as Input;
use crate::types::{ExternalPaymentMethod, Metadata, PaymentMethod, PhoneNumber};

#[derive(Debug, Clone)]
pub struct DirectCarrierBilling {
    phone: PhoneNumber,
    metadata: Option<Metadata>,
}

// Marker implementations

impl PaymentMethod for DirectCarrierBilling {}
impl ExternalPaymentMethod for DirectCarrierBilling {}

// Converters

impl DirectCarrierBilling {
    /// User phone number (primary payment identifier)
    #[inline]
    pub fn phone(&self) -> &PhoneNumber {
        &self.phone
    }
    /// Carrier-specific extensions
    #[inline]
    pub fn metadata(&self) -> &Option<Metadata> {
        &self.metadata
    }
}

impl<'a> TryFrom<Input<'a>> for DirectCarrierBilling {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            phone: input.phone.try_into()?,
            metadata: input.metadata.map(TryFrom::try_from).transpose()?,
        })
    }
}
