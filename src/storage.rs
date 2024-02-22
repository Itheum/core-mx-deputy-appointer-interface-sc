multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Module that handles the common storage of the smart contract
#[multiversx_sc::module]
pub trait StorageModule {
    // Stores the deputy address
    #[view(viewDeputyAddress)]
    #[storage_mapper("deputy_address")]
    fn deputy_address(&self) -> SingleValueMapper<ManagedAddress>;

    // Stores whether minting is paused or not
    #[view(getIsPaused)]
    #[storage_mapper("is_paused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;
}
