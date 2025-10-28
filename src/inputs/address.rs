/// Insecure representation of a postal address.
///
/// ```skip
/// let address = Address {
///     country_code: "PT-11",
///     postal_code: "1200-109",
///     city: "Lisbon",
///     line: "Av.Liberdade 14, 3ºDto",
/// }.try_into()?;
/// ```
pub struct Address<'a> {
    /// The country/region code as defined by ISO 3166-2.
    /// The code must contain a country part (like `PT`),
    /// and can contain a region part as well (like `PT-11`).
    pub country_code: &'a str,
    /// The postal code applicable to the address.
    pub postal_code: &'a str,
    /// The name of the city or town (like `Porto`).
    pub city: &'a str,
    /// The full address line within the city.
    pub line: &'a str,
}
