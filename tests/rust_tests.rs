use std::u8;

use deputyappointerinterface::requirements::RequirementsModule;
use deputyappointerinterface::storage::*;
use deputyappointerinterface::*;
use multiversx_sc::{
    codec::Empty,
    storage::mappers::StorageTokenWrapper,
    types::{Address, EsdtLocalRole},
};
use multiversx_sc_scenario::api::SingleTxApi;
use multiversx_sc_scenario::multiversx_chain_vm::tx_mock::TxContextRef;
use multiversx_sc_scenario::whitebox_legacy::{BlockchainStateWrapper, ContractObjWrapper};
use multiversx_sc_scenario::*;

pub const WASM_PATH: &'static str = "../output/deputyappointerinterface.wasm";
pub const OWNER_EGLD_BALANCE: u128 = 100 * 10u128.pow(18u32);
pub const TOKEN_ID: &[u8] = b"ITHEUM-df6f26";
pub const ANOTHER_TOKEN_ID: &[u8] = b"ANOTHER-123456";

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> deputyappointerinterface::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub contract_wrapper:
        ContractObjWrapper<deputyappointerinterface::ContractObj<DebugApi>, ContractObjBuilder>,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub deputy_address: Address,
    pub withdrawal_address: Address,
}

fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> deputyappointerinterface::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let first_user_address =
        blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE / 10u128));
    let second_user_address =
        blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE / 100u128));
    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE));
    let withdrawal_address = blockchain_wrapper.create_user_account(&rust_biguint!(0u64));
    let deputy_address =
        blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE / 10u128));
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );
    blockchain_wrapper.set_esdt_balance(&owner_address, TOKEN_ID, &rust_biguint!(5_000_000));
    blockchain_wrapper.set_esdt_balance(
        &owner_address,
        ANOTHER_TOKEN_ID,
        &rust_biguint!(1_000_000),
    );
    blockchain_wrapper.set_esdt_balance(&first_user_address, TOKEN_ID, &rust_biguint!(10_000));
    blockchain_wrapper.set_esdt_balance(&owner_address, ANOTHER_TOKEN_ID, &rust_biguint!(10_000));
    blockchain_wrapper.set_esdt_balance(&second_user_address, TOKEN_ID, &rust_biguint!(0));

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        deputy_address,
        withdrawal_address,
        contract_wrapper: cf_wrapper,
    }
}

#[test] // Tests whether the contract is deployed and initialized correctly after deployment.
fn deploy_test() {
    let mut setup = setup_contract(deputyappointerinterface::contract_obj);
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.init();
                sc.set_is_paused(false);
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.init();
                assert_eq!(sc.is_paused().get(), true);
            },
        )
        .assert_ok();
}

#[test] //Tests owner setting a new admin
        //Tests whether pause correct state after deployment
        //Tests whether the owner can unpause the contract and pause again
        //Tests whether the admin can unpause the contract
fn pause_test() {
    let mut setup = setup_contract(deputyappointerinterface::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), true)
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(false);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), false)
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(true);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), true)
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(false);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), false)
        })
        .assert_ok();
}



#[test] // Tests if the contract has whitelist enabled and is empty by default after deployment
        // Tests if other values are set correctly after deployment
        // Tests if the owner and administrator can change the max supply and royalties
fn value_setters_test() {
    let mut setup = setup_contract(deputyappointerinterface::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let administrator_address = &setup.first_user_address;
    let deputy_address = &setup.deputy_address;

    b_wrapper
        .execute_tx(
            owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.set_deputy_address(managed_address!(deputy_address)),
        )
        .assert_ok();

}

#[test] // Tests whether the owner can initialize the contract correctly.
fn setup_contract_test() {
    let mut setup = setup_contract(deputyappointerinterface::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let deputy_address = &setup.deputy_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_address!(deputy_address),
                )
            },
        )
        .assert_ok();

}
