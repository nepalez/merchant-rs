use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::Exposed;
use crate::types::{
    insecure,
    secure::{City, CountryCode, PostalCode, RegionCode, StreetAddress},
};

/// The address stored securely.
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

impl TryFrom<insecure::Address<'_>> for Address {
    type Error = Error;

    fn try_from(input: insecure::Address<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            country: CountryCode::from_str(input.country)?,
            city: City::from_str(input.city)?,
            line: StreetAddress::from_str(input.line)?,
            postal_code: input.postal_code.map(FromStr::from_str).transpose()?,
            region: input.region.map(FromStr::from_str).transpose()?,
        })
    }
}

// --- Sealed traits (not parts of the public API) ---

// SAFETY: The trait is safe to implement because:
// 1. Its output does not own any data;
// 2. It does not expose any part of its data via `first_chars` or `last_chars`.
unsafe impl Exposed for Address {
    type Output<'a> = insecure::Address<'a>;
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
