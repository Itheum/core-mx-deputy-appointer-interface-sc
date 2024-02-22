multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Module that handles generic (commonly used, which are not specific to one function) requirements which should stop execution and rollback if not met
#[multiversx_sc::module]
pub trait RequirementsModule: crate::storage::StorageModule {}
