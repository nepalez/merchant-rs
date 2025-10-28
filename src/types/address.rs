use std::convert::TryFrom;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::inputs::Address as Input;
use crate::types::{City, CountryCode, PostalCode, StreetAddress};

/// The address stored securely.
///
/// # Data Protection
/// The precise address is PII, deanonymizing the person behind it.
/// The necessary protection is implemented at the field level,
/// specifically in its most precise `line` and `postal_code`.
/// The rest (country-region-city) are not protected by design.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct Address {
    country_code: CountryCode,
    postal_code: PostalCode,
    city: City,
    line: StreetAddress,
}

impl Address {
    /// The country-specific code of the region (ISO 3166-2 alpha-2).
    pub fn country_code(&self) -> &CountryCode {
        &self.country_code
    }

    /// The country-specific postal code of the address.
    pub fn postal_code(&self) -> &PostalCode {
        &self.postal_code
    }

    /// The name of the city, town, village, or another locality.
    pub fn city(&self) -> &City {
        &self.city
    }

    /// The street address, P.O. box, company name, c/o, etc.
    pub fn line(&self) -> &StreetAddress {
        &self.line
    }
}

impl TryFrom<Input<'_>> for Address {
    type Error = Error;

    fn try_from(input: Input<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            country_code: input.country_code.try_into()?,
            postal_code: input.postal_code.try_into()?,
            city: input.city.try_into()?,
            line: input.line.try_into()?,
        })
    }
}
