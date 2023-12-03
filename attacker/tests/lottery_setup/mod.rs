#![allow(deprecated)]
use std::{cell::RefCell, rc::Rc};

pub const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] = multiversx_sc::hex_literal::hex!(
    "000000000000000000010000000000000000000000000000000000000002ffff"
);
pub const OWNER_EGLD_BALANCE: u64 = 150_000_000_000_000_000;

use lottery::Lottery;
use multiversx_sc::types::{ManagedAddress, Address};
use multiversx_sc_scenario::{*, testing_framework::{BlockchainStateWrapper, ContractObjWrapper, TxResult}};

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
    ) -> Self {
        let rust_zero = rust_biguint!(0);
        let owner_address = b_mock.borrow_mut().create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE));
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

    pub fn call_draw_winner(
        &self,
    ) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(
                &self.owner_address,
                &self.lottery_wrapper,
                &rust_biguint!(0), 
                |sc| {
                    sc.draw_winner()
                })
    }
}