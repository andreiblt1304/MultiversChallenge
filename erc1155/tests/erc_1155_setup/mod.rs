#![allow(deprecated)]

use std::{borrow::BorrowMut, cell::RefCell};

use multiversx_sc::types::Address;
use multiversx_sc_scenario::{testing_framework::{BlockchainStateWrapper, ContractObjWrapper}, DebugApi, rust_biguint};
use erc1155::Erc1155;

const ERC1155_WASM_PATH: &str = "../output/erc_1155.wasm";
const OWNER_BALANCE: u64 = 100000000;

pub struct Erc1155Setup<Erc1155ObjBuilder>
where
    Erc1155ObjBuilder: 'static + Copy + Fn() -> erc1155::ContractObj<DebugApi>,
{
    pub b_mock: RefCell<BlockchainStateWrapper>,
    pub owner_address: Address,
    pub erc_wrapper: ContractObjWrapper<erc1155::ContractObj<DebugApi>, Erc1155ObjBuilder>,
}

impl<Erc1155ObjBuilder> Erc1155Setup<Erc1155ObjBuilder>
where
    Erc1155ObjBuilder: 'static + Copy + Clone + Fn() -> erc1155::ContractObj<DebugApi>,
{
    pub fn new(
        b_mock: RefCell<BlockchainStateWrapper>,
        builder: Erc1155ObjBuilder,
        owner_address: &Address
    ) -> Self {
        let rust_zero = rust_biguint!(0);
        let erc_wrapper = b_mock.borrow_mut().create_sc_account(
            &rust_zero,
            Some(owner_address),
            builder,
            "Some path");

        b_mock
            .borrow_mut()
            .execute_tx(
            owner_address,
            &erc_wrapper, 
            &rust_zero, 
            |sc| {})
            .assert_ok();

        Self {
            b_mock: b_mock,
            owner_address: owner_address.clone(),
            erc_wrapper
        }
    }
    
    pub fn call_deposit(
        &mut self,
        erc1155_setup: &mut Erc1155Setup<Erc1155ObjBuilder>,
        amount: &num_bigint::BigUint,
    ) where
        Erc1155ObjBuilder: 'static + Copy + Fn() -> erc1155::ContractObj<DebugApi>
    {
        let b_wrapper = &mut erc1155_setup.b_mock;
        self.b_mock.borrow_mut().execute_esdt_transfer(
            &erc1155_setup.owner_address, 
        &erc1155_setup.erc_wrapper, 
            &[1u8], 
            0, 
            amount, 
            |sc| { sc.deposit(erc1155_setup.erc_wrapper.address_ref().clone()); }
        )
        .assert_ok();
    }
}

