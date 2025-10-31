use crate::inputs::Metadata;

pub struct CryptoAccount<'a> {
    /// Metadata associated with the crypto account
    pub metadata: Metadata<'a>,
}
