use std::cmp::Ordering;
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

impl Ord for CardExpiry {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month).cmp(&(other.year, other.month))
    }
}

impl PartialOrd for CardExpiry {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
