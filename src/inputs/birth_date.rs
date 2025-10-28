use zeroize_derive::ZeroizeOnDrop;

/// Insecure structure representing a person's birthdate (day, month, year).
///
/// ```skip
/// let birth_date = BirthDate {
///     day: 1,
///     month: 1,
///     year: 1970,
/// }.try_into()?;
/// ```
#[derive(ZeroizeOnDrop)]
pub struct BirthDate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}
