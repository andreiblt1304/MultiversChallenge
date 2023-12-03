#![allow(deprecated)]
use std::{cell::RefCell, rc::Rc};

pub const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] = multiversx_sc::hex_literal::hex!(
    "000000000000000000010000000000000000000000000000000000000002ffff"
);

use lottery::Lottery;
use multiversx_sc::types::{ManagedAddress, Address};
use multiversx_sc_scenario::{*, testing_framework::{BlockchainStateWrapper, ContractObjWrapper}};

pub struct LotterySetup<LotteryObjBuilder>
where
    LotteryObjBuilder: 'static + Copy + Fn() -> lottery::ContractObj<DebugApi>,
{
    pub b_mock: Rc<RefCell<BlockchainStateWrapper>>,
    pub owner_address: Address,
    pub lottery_wrapper: ContractObjWrapper<lottery::ContractObj<DebugApi>, LotteryObjBuilder>
}

impl<LotteryObjBuilder> LotterySetup<LotteryObjBuilder>
where
    LotteryObjBuilder: 'static + Copy + Fn() -> lottery::ContractObj<DebugApi>,
{
    pub fn new(
        b_mock: Rc<RefCell<BlockchainStateWrapper>>,
        builder: LotteryObjBuilder,
        owner_address: Address
    ) -> Self {
        let rust_zero = rust_biguint!(0);
        let lottery_wrapper = b_mock.borrow_mut().create_sc_account_fixed_address(
            &Address::from(ESDT_SYSTEM_SC_ADDRESS_ARRAY),
            &rust_zero,
            Some(&owner_address),
            builder,
            "../../lottery/output/lottery.wasm"
        );

        Self {
            b_mock,
            owner_address,
            lottery_wrapper,
        }
    }
}