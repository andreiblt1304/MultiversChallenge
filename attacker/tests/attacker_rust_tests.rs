#![allow(deprecated)]
pub const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] = multiversx_sc::hex_literal::hex!(
    "000000000000000000010000000000000000000000000000000000000002ffff"
);
use std::{cell::RefCell, rc::Rc};

use attacker_setup::AttackerSetup;
use lottery_setup::LotterySetup;
use multiversx_sc_scenario::{DebugApi, testing_framework::BlockchainStateWrapper, rust_biguint};

pub mod lottery_setup;
pub mod attacker_setup;

fn init_all<
    AttackerObjBuilder: 'static + Copy + Fn() -> attacker::ContractObj<DebugApi>,
    LotteryObjBuilder: 'static + Copy + Fn() -> lottery::ContractObj<DebugApi>
> (
    attacker_builder: AttackerObjBuilder,
    lottery_builder: LotteryObjBuilder
) -> (
    Rc<RefCell<BlockchainStateWrapper>>,
    AttackerSetup<AttackerObjBuilder>,
    LotterySetup<LotteryObjBuilder>
) {
    let mut b_mock = BlockchainStateWrapper::new();
    let b_mock_ref = RefCell::new(b_mock);
    let b_mock_rc = Rc::new(b_mock_ref);

    let owner = b_mock_rc.borrow_mut().create_user_account(&rust_biguint!(0));

    let attacker_sc = AttackerSetup::new(
        b_mock_rc.clone(),
        attacker_builder,
        lottery_builder,
        &owner,
    );

    let lottery_sc = LotterySetup::new(
        b_mock_rc.clone(),
        lottery_builder
    );

    (b_mock_rc, attacker_sc, lottery_sc)
}

#[test]
fn init_test() {
    let _ = init_all(|| attacker::contract_obj(), || lottery::contract_obj());
}

#[test]
fn withdraw_winner_endpoint_test() {
    let (b_mock_rc, attacker_sc, lottery_sc) = init_all(|| attacker::contract_obj(), || lottery::contract_obj());

    let caller = b_mock_rc.borrow_mut().create_user_account(&rust_biguint!(1000000));

    attacker_sc.call_draw_winner_endpoint(&caller, &attacker_sc.lottery_address).assert_ok();

    b_mock_rc.borrow().check_egld_balance(&caller, &rust_biguint!(999999));
    b_mock_rc.borrow().check_egld_balance(&attacker_sc.lottery_address, &rust_biguint!(1));
}