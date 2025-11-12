use std::convert::TryFrom;

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
#[derive(Clone, Debug)]
pub struct Address {
    pub(crate) country_code: CountryCode,
    pub(crate) postal_code: PostalCode,
    pub(crate) city: City,
    pub(crate) line: StreetAddress,
}

impl Address {
    /// The country-specific code of the region (ISO 3166-2 alpha-2).
    #[inline]
    pub fn country_code(&self) -> &CountryCode {
        &self.country_code
    }

    /// The country-specific postal code of the address.
    #[inline]
    pub fn postal_code(&self) -> &PostalCode {
        &self.postal_code
    }

    /// The name of the city, town, village, or another locality.
    #[inline]
    pub fn city(&self) -> &City {
        &self.city
    }

    /// The street address, P.O. box, company name, c/o, etc.
    #[inline]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;

    fn valid_input() -> Input<'static> {
        Input {
            country_code: " PT-11 \n\t",
            postal_code: " 1200-109 \n\t",
            city: " Lisbon \n\t",
            line: " Avenida Liberdade 14-3 \n\t",
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let address = Address::try_from(input).unwrap();

        unsafe {
            assert_eq!(address.country_code.as_ref(), "PT-11");
            assert_eq!(address.postal_code.as_ref(), "1200-109");
            assert_eq!(address.city.as_ref(), "Lisbon");
            assert_eq!(address.line.as_ref(), "Avenida Liberdade 14-3");
        }
    }

    #[test]
    fn rejects_invalid_country_code() {
        let mut input = valid_input();
        input.country_code = "X";

        let result = Address::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_postal_code() {
        let mut input = valid_input();
        input.postal_code = "12";

        let result = Address::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_city() {
        let mut input = valid_input();
        input.city = "";

        let result = Address::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_line() {
        let mut input = valid_input();
        input.line = "AB";

        let result = Address::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
