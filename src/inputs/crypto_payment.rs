use crate::inputs::Metadata;

pub struct CryptoPayment<'a> {
    /// Crypto-specific extensions (currency, network, wallet address)
    pub metadata: Metadata<'a>,
}
