use std::fmt::Display;

/// Categories of account holders.
#[derive(Clone, Copy, Debug)]
pub enum AccountHolderType {
    Individual,
    Company,
}

impl Display for AccountHolderType {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
