use zeroize_derive::ZeroizeOnDrop;

use crate::types::{City, CountryCode, PostalCode, RegionCode, StreetAddress};

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
