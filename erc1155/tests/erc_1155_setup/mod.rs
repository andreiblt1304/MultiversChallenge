#![allow(deprecated)]

use std::{cell::RefCell, rc::Rc};
use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::types::BigUint;
use multiversx_sc::types::Address;
use multiversx_sc::types::EgldOrEsdtTokenIdentifier;
use multiversx_sc::types::ManagedVec;
use multiversx_sc::types::MultiValueEncoded;
use multiversx_sc::types::TokenIdentifier;
use multiversx_sc_scenario::testing_framework::TxResult;
use multiversx_sc_scenario::{testing_framework::{BlockchainStateWrapper, ContractObjWrapper}, DebugApi, rust_biguint};
use erc1155::Erc1155;
const ERC1155_WASM_PATH: &str = "../output/erc_1155.wasm";
const OWNER_BALANCE: u64 = 100000000;

pub struct Erc1155Setup<Erc1155ObjBuilder>
where
    Erc1155ObjBuilder: 'static + Copy + Fn() -> erc1155::ContractObj<DebugApi>,
{
    pub b_mock: Rc<RefCell<BlockchainStateWrapper>>,
    pub owner_address: Address,
    pub erc_wrapper: ContractObjWrapper<erc1155::ContractObj<DebugApi>, Erc1155ObjBuilder>,
}

impl<Erc1155ObjBuilder> Erc1155Setup<Erc1155ObjBuilder>
where
    Erc1155ObjBuilder: 'static + Copy + Fn() -> erc1155::ContractObj<DebugApi>,
{
    pub fn new(
        b_mock: Rc<RefCell<BlockchainStateWrapper>>,
        builder: Erc1155ObjBuilder,
        owner_address: &Address,
        accepted_tokens: Vec<Vec<u8>>
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
            |sc| {
                let mut args = MultiValueEncoded::new();
                for arg in accepted_tokens {
                    if &arg == b"EGLD" {
                        let token_id = EgldOrEsdtTokenIdentifier::egld();
                        args.push(token_id);
                    } else {
                        let token_id = TokenIdentifier::from_esdt_bytes(arg);
                        args.push(EgldOrEsdtTokenIdentifier::esdt(token_id));
                    }
                }
            })
            .assert_ok();

        Self {
            b_mock,
            owner_address: owner_address.clone(),
            erc_wrapper
        }
    }
    
    pub fn call_deposit(
        &mut self,
        caller: &Address,
        token_id: &[u8],
        amount: u64,
    ) -> TxResult {
        //let b_wrapper = &self.b_mock;
        self.b_mock.borrow_mut().execute_esdt_transfer(
            caller, 
        &self.erc_wrapper, 
            token_id, 
            0, 
            &rust_biguint!(amount), 
            |sc| { sc.deposit(self.erc_wrapper.address_ref().clone()); }
        )
    }
}

