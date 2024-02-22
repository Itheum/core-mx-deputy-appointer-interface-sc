#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod events;
pub mod requirements;
pub mod storage;
pub mod views;

#[multiversx_sc::contract]
pub trait DeputyAppointerInterface:
    storage::StorageModule
    + events::EventsModule
    + requirements::RequirementsModule
    + views::ViewsModule
{
    #[init]
    fn init(&self) {
        self.is_paused().set(true);
        self.mint_pause_toggle_event(&true);
    }

    #[only_owner]
    #[endpoint(initializeContract)]
    fn initialize_contract(
        &self,
        deputy_address: ManagedAddress,
    ) {
        self.deputy_address().set(&deputy_address);
    }

    #[only_owner]
    #[endpoint(setDeputyAddress)]
    fn set_deputy_address(&self, address: ManagedAddress) {
        self.deputy_address_event(&address);
        self.deputy_address().set(&address);
    }

    #[only_owner]
    #[endpoint(setIsPaused)]
    fn set_is_paused(&self, is_paused: bool) {
        self.mint_pause_toggle_event(&is_paused);
        self.is_paused().set(is_paused);
    }
  }
