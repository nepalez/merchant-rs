use crate::internal::Exposed;
use crate::types::{City, CountryCode, PostalCode, RegionCode, StreetAddress};
use zeroize_derive::ZeroizeOnDrop;

/// Address of a user for authorization and post-processing
///
/// # Validation
/// No specific rules except for those applied to fields.
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

// --- Sealed traits (not parts of the public API) ---

unsafe impl Exposed for Address {
    type Output<'a> = ExposedAddress<'a>;
    const TYPE_WRAPPER: &'static str = "Address";

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

#[derive(Clone)]
pub(crate) struct ExposedAddress<'a> {
    pub country: &'a str,
    pub city: &'a str,
    pub line: &'a str,
    pub postal_code: Option<&'a str>,
    pub region: Option<&'a str>,
}
