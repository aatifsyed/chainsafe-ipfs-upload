//! This library provides two top level functions:
//! - [upload_to_ipfs] for uploading binary data to an IPFS server
//! - [store_string] for storing an arbitrary string in a new contract
use std::borrow::Borrow;

use ethers::{
    abi::Tokenize,
    contract::{ContractError, ContractInstance},
    prelude::DeploymentTxFactory,
};

/// Returns the hash of the uploaded content
// TODO(aatifsyed): shouldn't return a String, instead some domain-specific type
#[tracing::instrument(level = "debug", skip(data), ret, err)]
pub async fn upload_to_ipfs(
    server_address: std::net::SocketAddr,
    data: impl std::io::Read + Send + Sync + Unpin + 'static,
) -> Result<String, IpfsUploadError> {
    use ipfs_api_backend_hyper::{IpfsApi as _, IpfsClient, TryFromUri as _};

    let client = IpfsClient::from_socket(
        // BUG?(aatifsyed): using HTTP panics:
        //     Client(hyper::Error(Connect, "invalid URL, scheme is not http"))
        http::uri::Scheme::HTTP,
        server_address,
    )
    .map_err(IpfsUploadError::ClientCreationError)?;

    let added = client
        .add(data)
        .await
        .map_err(IpfsUploadError::UploadError)?;

    Ok(added.hash)
}

#[derive(Debug, thiserror::Error, enum_as_inner::EnumAsInner)]
pub enum IpfsUploadError {
    #[error("failed to create ipfs client")]
    ClientCreationError(#[source] http::Error),
    #[error("failed to upload data")]
    UploadError(#[source] ipfs_api_backend_hyper::Error),
}

/// Returns the address of a newly created contract which contains the stored string.
/// The wallet (secret key holder) must have enough eth for gas to deploy the contract.
//
// We catch as many errors as possible in our contract at compile time - see build.rs
//
// BUG?(aatifsyed): _surely_ we can't store an unbounded string - how do we handle the error case? Will it be caught in DetokenizationError?
#[tracing::instrument(level = "debug", skip(secret_key), ret, err)]
pub async fn store_string(
    string: String,
    provider_url: url::Url,
    secret_key: ethers::core::k256::SecretKey,
    chain_id: u64,
) -> Result<ethers::types::H160, StoreStringError> {
    use ethers::{
        contract::ContractError::{
            AbiError, ConstructorError, ContractNotDeployed, DecodingError, DetokenizationError,
            MiddlewareError, ProviderError, Revert,
        },
        core::types::Bytes,
        middleware::SignerMiddleware,
        providers::{Http, Provider},
        signers::{LocalWallet, Signer as _},
    };

    let wallet = LocalWallet::from(secret_key);
    let client = SignerMiddleware::new(
        Provider::new(Http::new(provider_url)),
        wallet.with_chain_id(chain_id),
    );

    let factory = DeploymentTxFactory::new(
        // TODO(aatifsyed): eliminate the need for deser (and therefore the error case) using a crate like databake
        serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/abi.json"))).expect(
            "we've just compiled this abi in build.rs, and it should cross the serde boundary intact",
        ),
        Bytes::from_static(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/bytecode.bin"
        ))),
        client,
    );
    let contract = match flat_deploy(factory, string).await {
        Ok(deployer) => deployer,
        Err(AbiError(_) | DecodingError(_)) => unreachable!("newly compiled contract"),
        Err(MiddlewareError { e }) => return Err(StoreStringError::SigningError(e)),
        Err(Revert(_) | ContractNotDeployed | DetokenizationError(_)) => {
            unreachable!("not applicable")
        }
        Err(ConstructorError) => {
            panic!("code not up-to-date with contract requirements")
        }
        Err(ProviderError { e }) => return Err(StoreStringError::ProviderError(e)),
    };
    Ok(contract.address())
}

/// Utility for nesting deployment and sending results
async fn flat_deploy<ClientT, MiddlewareT>(
    factory: DeploymentTxFactory<ClientT, MiddlewareT>,
    constructor_args: impl Tokenize,
) -> Result<ContractInstance<ClientT, MiddlewareT>, ContractError<MiddlewareT>>
where
    MiddlewareT: ethers::providers::Middleware,
    ClientT: Borrow<MiddlewareT> + Clone,
{
    factory.deploy(constructor_args)?.send().await
}

#[derive(Debug, thiserror::Error, enum_as_inner::EnumAsInner)]
pub enum StoreStringError {
    #[error("error in signing middleware")]
    SigningError(
        ethers::middleware::signer::SignerMiddlewareError<
            ethers::providers::Provider<ethers::providers::Http>,
            ethers::signers::Wallet<ethers::core::k256::ecdsa::SigningKey>,
        >,
    ),
    #[error(transparent)]
    ProviderError(ethers::providers::ProviderError),
}
