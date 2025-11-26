use std::convert::TryFrom;

use crate::Error;
use crate::types::{ExternalPaymentMethod, Metadata};

/// Cryptocurrency Payment via Blockchain Transfer
///
/// ## Overview
///
/// Customer sends cryptocurrency directly from their wallet to a gateway-generated deposit address.
/// Payment is settled on-chain through blockchain network confirmation.
/// Transaction is irreversible once confirmed on the blockchain (typically 1-6 block confirmations
/// depending on network and amount). Gateway monitors the blockchain for incoming transactions and
/// confirms payment asynchronously via webhook.
///
/// ## When to Use
///
/// - **Borderless payments**: No traditional banking infrastructure required
/// - **Crypto-native users**: Customers holding cryptocurrency who prefer to pay directly
/// - **Traditional rails unavailable**: Regions where card or bank transfer infrastructure is limited
/// - **High-value international transfers**: Lower fees than wire transfers for large amounts
/// - **Privacy-focused transactions**: Reduced KYC requirements compared to traditional payments
/// - **24/7 settlement**: No banking hours or weekend delays
///
/// ## Authentication Model
///
/// > Authentication occurs **on-chain through cryptographic signatures**, not in authorization request!
///
/// ### Payment Flow Steps
///
/// 1. **Merchant initiates**: Calls `authorize()` with crypto payment request
/// 2. **Gateway generates address**: Creates a unique deposit address for the transaction
/// 3. **Customer receives address**: Via QR code, direct address, or payment link
/// 4. **Customer initiates transfer**: Sends crypto from any wallet to the deposit address
/// 5. **Blockchain confirmation**: Transaction propagates through network, miners/validators confirm
/// 6. **Gateway monitors blockchain**: Watches for incoming transactions to the deposit address
/// 7. **Webhook notification**: Gateway notifies the merchant after sufficient confirmations (async, minutes to hours)
/// 8. **Settlement completion**: Funds available to merchant (it can be converted to fiat automatically)
///
/// ### Authorization Request Content
///
/// The authorization request contains **only metadata for currency/network selection**,
/// not authentication credentials. Authentication is inherent in the blockchain — only the holder
/// of private keys can sign valid transactions.
///
/// ## Standards
///
/// - **Bitcoin (BTC)**: [Bitcoin Core reference implementation](https://bitcoin.org/en/bitcoin-core/), BIP standards
/// - **Ethereum (ETH)**: [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf), EIP standards
/// - **[ERC-20 tokens](https://eips.ethereum.org/EIPS/eip-20)**: Standard for fungible tokens (USDT, USDC, DAI)
/// - **[BIP-21](https://github.com/bitcoin/bips/blob/master/bip-0021.mediawiki)**: Bitcoin URI scheme for payment requests
/// - **[EIP-681](https://eips.ethereum.org/EIPS/eip-681)**: Ethereum URI scheme for payment requests
/// - **[Lightning Network](https://github.com/lightning/bolts)**: BOLT specifications for instant Bitcoin payments
///
/// ## Example Systems
///
/// ### Payment Processors
/// - **BitPay**: BTC, BCH, ETH, stablecoins; invoice generation, auto-conversion
/// - **Coinbase Commerce**: BTC, ETH, LTC, BCH, USDC; merchant-hosted checkout
/// - **NOWPayments**: 200+ cryptocurrencies, auto-conversion, recurring payments
/// - **CoinGate**: 70+ cryptocurrencies, Lightning Network, fiat settlement
/// - **BTCPay Server**: Self-hosted, noncustodial, supports BTC and Lightning
///
/// ### Supported Networks
/// - **Bitcoin (BTC)**: Most established, 10-60 minute confirmation
/// - **Ethereum (ETH)**: Smart contract platform, ~15 second blocks
/// - **Lightning Network**: Instant Bitcoin micropayments, off-chain scaling
/// - **Stablecoins**: USDT, USDC, DAI on multiple networks
///
/// ## Security Considerations
///
/// ### Blockchain Security
/// - Customer authenticates via private key cryptographic signature
/// - Gateway never handles customer's private keys
/// - Transactions are irreversible once confirmed on-chain
/// - No chargebacks or payment reversals
///
/// ### Address Generation
/// - Use HD (Hierarchical Deterministic) wallets for address generation
/// - Each transaction should use a unique deposit address
/// - Never reuse addresses to prevent transaction correlation
/// - Validate an address format before displaying to the customer
///
/// ### Confirmation Requirements
/// - Higher value transactions require more confirmations
/// - Bitcoin: 1 confirmation (~10 min) for small amounts, 6 confirmations (~60 min) for large
/// - Ethereum: 12 confirmations (~3 min) typical, 35+ for high value
/// - Stablecoins: Follow underlying network confirmation rules
///
/// ### Price Volatility
/// - Cryptocurrency prices are highly volatile
/// - Lock exchange rate at time of address generation
/// - Set expiration time for payment requests (typically 15-30 minutes)
/// - Auto-convert to fiat immediately upon confirmation to reduce merchant risk
///
/// ### Compliance
/// - **AML/KYC**: Requirements vary by jurisdiction and transaction size
/// - **Travel Rule**: Some jurisdictions require customer information for large transfers
/// - **Tax reporting**: Cryptocurrency transactions may trigger reporting obligations
/// - **Sanctions screening**: Check addresses against OFAC and other sanctions lists
#[derive(Debug, Clone)]
pub struct CryptoPayment {
    pub(crate) metadata: Metadata,
}

impl CryptoPayment {
    /// Crypto-specific extensions (currency, network, wallet address)
    #[inline]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

// Marker implementations

impl ExternalPaymentMethod for CryptoPayment {}

impl<'a> TryFrom<crate::CryptoPayment<'a>> for CryptoPayment {
    type Error = Error;

    fn try_from(input: crate::CryptoPayment<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: input.metadata.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    use std::collections::HashMap;

    fn valid_input() -> crate::CryptoPayment<'static> {
        let mut meta = HashMap::new();
        meta.insert("currency", "BTC");
        meta.insert("network", "mainnet");

        inputs::CryptoPayment { metadata: meta }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let crypto = CryptoPayment::try_from(input).unwrap();

        // Metadata exists
        let debug = format!("{:?}", crypto.metadata);
        assert!(!debug.is_empty());
    }
}
