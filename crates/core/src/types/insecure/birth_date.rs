use std::cmp::Ordering;
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
#[derive(Eq, PartialEq, ZeroizeOnDrop)]
pub struct BirthDate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Ord for BirthDate {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, self.day))
    }
}

impl PartialOrd for BirthDate {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
