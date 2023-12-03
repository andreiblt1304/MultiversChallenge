#![allow(deprecated)]

use std::{cell::RefCell, rc::Rc};

use attacker::{Attacker, lottery_proxy::LotteryProxy};
use lottery::Lottery;
use multiversx_sc::types::Address;
use multiversx_sc_scenario::{testing_framework::{BlockchainStateWrapper, ContractObjWrapper, TxResult}, DebugApi, rust_biguint};

use crate::lottery_setup::*;

pub struct AttackerSetup<AttackerObjBuilder>
where
    AttackerObjBuilder: 'static + Copy + Fn() -> attacker::ContractObj<DebugApi>,
{
    pub b_mock: Rc<RefCell<BlockchainStateWrapper>>,
    pub owner_address: Address,
    pub lottery_address: Address,
    pub attacker_wrapper: ContractObjWrapper<attacker::ContractObj<DebugApi>, AttackerObjBuilder>,
}

impl<AttackerObjBuilder> AttackerSetup<AttackerObjBuilder>
where
    AttackerObjBuilder: 'static + Copy + Fn() -> attacker::ContractObj<DebugApi>,
{
    pub fn new<LotteryObjBuilder>(
        b_mock: Rc<RefCell<BlockchainStateWrapper>>,
        attacker_builder: AttackerObjBuilder,
        lottery_builder: LotteryObjBuilder,
        owner_address: &Address,
    ) -> Self 
        where
            LotteryObjBuilder: 'static + Copy + Fn() -> lottery::ContractObj<DebugApi>
    {
        let rust_zero = rust_biguint!(0);
        let attacker_wrapper = b_mock.borrow_mut().create_sc_account(
            &rust_zero,
            Some(&owner_address),
            attacker_builder,
            "../output/attacker.wasm"
        );

        let lottery_wrapper = 
            b_mock
                .borrow_mut()
                .create_sc_account(
                    &rust_zero,
                    Some(&owner_address),
                    lottery_builder,
                    "lottery path"
                );

        b_mock
            .borrow_mut()
            .execute_tx(
                &owner_address, 
                &attacker_wrapper,
                &rust_zero,
                |sc| {
                    sc.init(attacker_wrapper.address_ref().into())
                }
            )
            .assert_ok();

        Self {
            b_mock,
            owner_address: owner_address.clone(),
            lottery_address: lottery_wrapper.address_ref().clone(),
            attacker_wrapper,
        }
    }

    pub fn participate(
        &self,
        caller: &Address,
        lottery_sc_address: &Address
    ) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(
                caller, 
                &self.attacker_wrapper, 
                &rust_biguint!(1), 
                |sc| {
                    sc.participate(lottery_sc_address.clone().into());
                })
    }
}