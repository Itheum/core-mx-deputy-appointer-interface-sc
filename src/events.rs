multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Module that handles event emitting for important smart contract events in order to facilitate logging, debugging and monitoring with ease
#[multiversx_sc::module]
pub trait EventsModule {
    // Emitted whenever minting pause changes value
    #[event("mintPauseToggle")]
    fn mint_pause_toggle_event(&self, #[indexed] pause_value: &bool);

    // Emitted whenever deputy address is set
    #[event("setDeputyAddress")]
    fn deputy_address_event(&self, #[indexed] deputy_address: &ManagedAddress);
}
