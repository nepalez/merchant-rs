use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::Exposed;
use crate::types::{City, CountryCode, PostalCode, RegionCode, StreetAddress};

/// Address of a user for authorization and post-processing
///
/// Use the builder pattern to create the Address structure:
/// ```skip
/// let address = Address::builder()
///     .country("PT")?
///     .region("11")?
///     .city("Lisbon")?
///     .line("Rua do Alecrim, 15")?
///     .postal_code("1200-018")?
///     .build()?;
/// ```
///
/// # Data Protection
/// The precise address is PII, deanonymizing the person behind it.
/// The necessary protection is implemented at the field level,
/// specifically in its most precise `line` and `postal_code`.
/// The rest (country-region-city) are not protected by design.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct Address {
    /// The country of the address (ISO 3166-1 alpha-2).
    pub country: CountryCode,
    /// The name of the city, town, village, or another locality.
    pub city: City,
    /// The street address, P.O. box, company name, c/o, etc.
    pub line: StreetAddress,

    /// The country-specific postal code of the address.
    pub postal_code: Option<PostalCode>,
    /// The country-specific code of the region (ISO 3166-2 alpha-2).
    pub region: Option<RegionCode>,
}

impl Address {
    /// Start the builder chain to create the Address structure.
    #[inline]
    pub fn builder() -> Builder {
        Builder::default()
    }
}

// --- Sealed traits (not parts of the public API) ---

// SAFETY: The trait is safe to implement because
//         it relies on the safety of the trait implementations
//         for the sensitive fields (postal_code, line).
//         The rest of its fields are not sensitive (country, region, city).
unsafe impl Exposed for Address {
    type Output<'a> = ExposedAddress<'a>;
    const TYPE_WRAPPER: &'static str = "Address";

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        Self::Output {
            country: self.country.as_ref(),
            city: self.city.as_ref(),
            line: self.line.expose(),
            postal_code: self.postal_code.as_ref().map(Exposed::expose),
            region: self.region.as_ref().map(AsRef::as_ref),
        }
    }
}

// --- Additional types ---

/// Represent the sensitive fields of the Address.
///
/// # SAFETY
/// This data should be used in a controlled environment only
/// (within the closure passed to the `expose_secrets` method of the request).
#[derive(Clone)]
pub struct ExposedAddress<'a> {
    pub country: &'a str,
    pub city: &'a str,
    pub line: &'a str,
    pub postal_code: Option<&'a str>,
    pub region: Option<&'a str>,
}

/// Intermediate structure to safely build the Address.
#[derive(Default)]
pub struct Builder {
    country: Option<CountryCode>,
    city: Option<City>,
    line: Option<StreetAddress>,
    postal_code: Option<PostalCode>,
    region: Option<RegionCode>,
}

impl Builder {
    /// Set the country of the address (required).
    #[inline]
    pub fn country(mut self, input: &str) -> Result<Self, Error> {
        self.country = Some(CountryCode::from_str(input)?);
        Ok(self)
    }

    /// Set the region of the address.
    #[inline]
    pub fn region(mut self, input: &str) -> Result<Self, Error> {
        self.region = Some(RegionCode::from_str(input)?);
        Ok(self)
    }

    /// Set the city of the address (required).
    #[inline]
    pub fn city(mut self, input: &str) -> Result<Self, Error> {
        self.city = Some(City::from_str(input)?);
        Ok(self)
    }

    /// Set the address line (street, house, etc.) of the address (required).
    #[inline]
    pub fn line(mut self, input: &str) -> Result<Self, Error> {
        self.line = Some(StreetAddress::from_str(input)?);
        Ok(self)
    }

    /// Set the postal code of the address.
    #[inline]
    pub fn postal_code(mut self, input: &str) -> Result<Self, Error> {
        self.postal_code = Some(PostalCode::from_str(input)?);
        Ok(self)
    }

    /// Finalize the address
    #[inline]
    pub fn build(self) -> Result<Address, Error> {
        let Some(country) = self.country else {
            Err(Error::validation_failed("country is missed".to_string()))?
        };
        let Some(city) = self.city else {
            Err(Error::validation_failed("city is missed".to_string()))?
        };
        let Some(line) = self.line else {
            Err(Error::validation_failed("line is missed".to_string()))?
        };

        Ok(Address {
            country,
            city,
            line,
            postal_code: self.postal_code,
            region: self.region,
        })
    }
}
