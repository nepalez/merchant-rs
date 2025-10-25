use super::{City, CountryCode, PostalCode, RegionCode, StreetAddress};

/// Insecure representation of a postal address.
///
/// ```skip
/// let address = Address {
///     country: "PT",
///     region: Some("11"),
///     city: "Lisbon",
///     line: "Rua do Alecrim",
///     postal_code: Some("1200-014"),
/// }.try_into()?;
/// ```
pub struct Address<'a> {
    pub country: CountryCode<'a>,
    pub city: City<'a>,
    pub line: StreetAddress<'a>,
    pub postal_code: Option<PostalCode<'a>>,
    pub region: Option<RegionCode<'a>>,
}
