multiversx_sc::imports!();
multiversx_sc::derive_imports!();

//Module that handles read-only endpoints (views) for the smart contract
#[multiversx_sc::module]
pub trait ViewsModule: crate::storage::StorageModule {}
