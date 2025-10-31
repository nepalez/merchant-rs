use std::convert::TryFrom;

use crate::Error;
use crate::inputs::CryptoPayment as Input;
use crate::types::{ExternalPaymentMethod, Metadata, PaymentMethod};

// Marker implementations

impl PaymentMethod for CryptoPayment {}
impl ExternalPaymentMethod for CryptoPayment {}

// Converters

#[derive(Debug, Clone)]
pub struct CryptoPayment {
    metadata: Metadata,
}

impl CryptoPayment {
    /// Carrier-specific extensions
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl<'a> TryFrom<Input<'a>> for CryptoPayment {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: input.metadata.try_into()?,
        })
    }
}
