#[cfg(feature = "evm")]
pub mod evm;
#[cfg(feature = "http")]
pub mod privy;
#[cfg(feature = "solana")]
pub mod solana;

#[cfg(feature = "evm")]
pub mod local_wallet;

use std::future::Future;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

#[cfg(feature = "hype")]
pub trait AsHypeSigner {
    fn as_hype_signer(&self)
        -> Arc<dyn hyperliquid_rust_sdk::signer::Signer>;
}

#[cfg(feature = "hype")]
impl AsHypeSigner for Arc<dyn TransactionSigner> {
    fn as_hype_signer(
        &self,
    ) -> Arc<dyn hyperliquid_rust_sdk::signer::Signer> {
        Arc::new(HypeSigner(self.clone()))
    }
}

#[cfg(feature = "hype")]
#[derive(Debug)]
struct HypeSigner(Arc<dyn TransactionSigner>);

#[cfg(feature = "hype")]
#[async_trait]
impl hyperliquid_rust_sdk::signer::Signer for HypeSigner {
    fn address(&self) -> ethers::types::Address {
        self.0
            .address()
            .expect("Address not available")
            .parse()
            .expect("Invalid address format")
    }

    async fn secp256k1_sign(
        &self,
        message: ethers::types::H256,
    ) -> std::result::Result<
        ethers::types::Signature,
        hyperliquid_rust_sdk::Error,
    > {
        self.0.secp256k1_sign(message).await
    }
}

#[cfg(feature = "evm")]
use self::evm::LocalEvmSigner;
#[cfg(feature = "http")]
use self::privy::PrivySigner;
#[cfg(feature = "solana")]
use self::solana::LocalSolanaSigner;

pub enum Transaction {
    #[cfg(feature = "solana")]
    Solana(solana_sdk::transaction::Transaction),
    #[cfg(feature = "evm")]
    Evm(),
}

pub enum SignerType {
    #[cfg(feature = "solana")]
    LocalSolana(LocalSolanaSigner),
    #[cfg(feature = "evm")]
    LocalEvm(LocalEvmSigner),
    #[cfg(feature = "http")]
    Privy(PrivySigner),
}

#[async_trait]
pub trait TransactionSigner: Send + Sync + std::fmt::Debug {
    fn locale(&self) -> String {
        "en".to_string()
    }

    fn user_id(&self) -> Option<String> {
        None
    }

    fn address(&self) -> Option<String> {
        None
    }

    fn pubkey(&self) -> Option<String> {
        None
    }

    fn evm_wallet_id(&self) -> Option<String> {
        None
    }

    #[cfg(feature = "solana")]
    async fn sign_and_send_solana_transaction(
        &self,
        _tx: &mut solana_sdk::transaction::VersionedTransaction,
    ) -> Result<String> {
        Err(anyhow::anyhow!(
            "Solana transactions not supported by this signer"
        ))
    }

    #[cfg(feature = "evm")]
    async fn sign_and_send_evm_transaction(
        &self,
        _tx: alloy::rpc::types::TransactionRequest,
    ) -> Result<String> {
        Err(anyhow::anyhow!(
            "EVM transactions not supported by this signer"
        ))
    }

    async fn sign_and_send_encoded_solana_transaction(
        &self,
        _tx: String,
    ) -> Result<String> {
        Err(anyhow::anyhow!(
            "Solana transactions not supported by this signer"
        ))
    }

    async fn sign_and_send_json_evm_transaction(
        &self,
        _tx: serde_json::Value,
        _caip2: Option<String>,
    ) -> Result<String> {
        Err(anyhow::anyhow!(
            "EVM transactions not supported by this signer"
        ))
    }

    #[cfg(feature = "hype")]
    async fn secp256k1_sign(
        &self,
        _message: ethers::types::H256,
    ) -> std::result::Result<
        ethers::types::Signature,
        hyperliquid_rust_sdk::Error,
    > {
        Err(hyperliquid_rust_sdk::Error::Wallet(
            "secp256k1_sign not supported by this signer".to_string(),
        ))
    }
}

tokio::task_local! {
    static CURRENT_SIGNER: Arc<dyn TransactionSigner>;
}

pub struct SignerContext;

impl SignerContext {
    pub async fn with_signer<T>(
        signer: Arc<dyn TransactionSigner>,
        f: impl Future<Output = Result<T>> + Send,
    ) -> Result<T> {
        CURRENT_SIGNER.scope(signer, f).await
    }

    pub async fn current() -> Arc<dyn TransactionSigner> {
        CURRENT_SIGNER.get().clone()
    }
}
