use std::convert::TryFrom;

use crate::error::Error;
use crate::inputs::DirectCarrier as Input;
use crate::internal::{ExternalPaymentSource, PaymentSource, TokenizablePaymentSource};
use crate::types::{Metadata, PhoneNumber};

#[derive(Debug, Clone)]
pub struct DirectCarrier {
    phone: PhoneNumber,
    metadata: Option<Metadata>,
}

// Marker implementations

impl PaymentSource for DirectCarrier {}
impl ExternalPaymentSource for DirectCarrier {}
impl TokenizablePaymentSource for DirectCarrier {}

// Converters

impl DirectCarrier {
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

impl<'a> TryFrom<Input<'a>> for DirectCarrier {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            phone: input.phone.try_into()?,
            metadata: input.metadata.map(TryFrom::try_from).transpose()?,
        })
    }
}
