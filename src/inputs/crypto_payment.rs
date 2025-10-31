use crate::inputs::Metadata;

pub struct CryptoPayment<'a> {
    /// Carrier-specific extensions
    pub metadata: Metadata<'a>,
}
