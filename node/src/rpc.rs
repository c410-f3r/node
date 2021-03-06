//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

pub use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_transaction_pool::TransactionPool;
use std::sync::Arc;
use valiu_node_runtime_types::{AccountId, Index, OpaqueBlock};

/// Full client dependencies.
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// Transaction pool instance.
    pub pool: Arc<P>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
    C: HeaderBackend<OpaqueBlock> + HeaderMetadata<OpaqueBlock, Error = BlockChainError> + 'static,
    C: ProvideRuntimeApi<OpaqueBlock>,
    C: Send + Sync + 'static,
    C::Api: BlockBuilder<OpaqueBlock>,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<OpaqueBlock, AccountId, Index>,
    P: TransactionPool + 'static,
{
    use substrate_frame_rpc_system::{FullSystem, SystemApi};

    let mut io = jsonrpc_core::IoHandler::default();

    let FullDeps {
        client,
        pool,
        deny_unsafe,
    } = deps;

    io.extend_with(SystemApi::to_delegate(FullSystem::new(
        client,
        pool,
        deny_unsafe,
    )));

    io
}
