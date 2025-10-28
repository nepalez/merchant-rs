use zeroize_derive::ZeroizeOnDrop;

/// Card expiry (month, year)
///
/// ```skip
/// let expiry = CardExpiry {
///     month: 8,
///     year: 2030,
/// }.try_into()?;
/// ```
#[derive(Eq, PartialEq, ZeroizeOnDrop)]
pub struct CardExpiry {
    pub month: u8,
    pub year: u16,
}
