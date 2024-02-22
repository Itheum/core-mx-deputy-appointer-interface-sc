use crate::errors::ERR_TOKEN_ISSUED;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait Callbacks: crate::storage::StorageModule {}
