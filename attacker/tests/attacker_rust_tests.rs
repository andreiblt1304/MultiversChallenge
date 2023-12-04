#![allow(deprecated)]

use std::{cell::RefCell, rc::Rc};

use attacker_setup::{AttackerSetup, TEN_EGLD, ONE_EGLD, THOUSAND_EGLD, TEN_THOUSAND_EGLD};
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

    let attacker_owner = b_mock_rc.borrow_mut().create_user_account(&rust_biguint!(0));
    let lottery_owner = b_mock_rc.borrow_mut().create_user_account(&rust_biguint!(0));

    let attacker_sc = AttackerSetup::new(
        b_mock_rc.clone(),
        attacker_builder,
        lottery_builder,
        &attacker_owner,
    );

    //attacker_sc.lot

    let lottery_sc = LotterySetup::new(
        b_mock_rc.clone(),
        lottery_builder,
        &attacker_sc.lottery_address,
        lottery_owner,
        
    );

    (b_mock_rc, attacker_sc, lottery_sc)
}

#[test]
fn init_test() {
    let _ = init_all(|| attacker::contract_obj(), || lottery::contract_obj());
}

#[test]
fn participate_test() {
    let (b_mock_rc, attacker_sc, lottery_sc) = 
        init_all(|| attacker::contract_obj(), || lottery::contract_obj());

    let caller_balance = rust_biguint!(10) * ONE_EGLD;
    let caller = b_mock_rc.borrow_mut().create_user_account(&caller_balance);
    let sc_balance_before_participating = b_mock_rc.borrow().get_egld_balance(&attacker_sc.lottery_address);

    attacker_sc.participate(&caller, &attacker_sc.lottery_address).assert_ok();

    let sc_balance_after_participating = b_mock_rc.borrow().get_egld_balance(&attacker_sc.lottery_address);

    b_mock_rc.borrow().check_egld_balance(&caller, &rust_biguint!(TEN_EGLD - ONE_EGLD));
    assert!(sc_balance_before_participating < sc_balance_after_participating);
    // b_mock_rc.borrow().check_egld_balance(&attacker_sc.lottery_address, &rust_biguint!(ONE_EGLD));
}

#[test]
fn draw_winner_test() {
    let (b_mock_rc, attacker_sc, lottery_sc) = 
        init_all(|| attacker::contract_obj(), || lottery::contract_obj());
    let caller_balance = rust_biguint!(1000) * ONE_EGLD;
    let caller = b_mock_rc.borrow_mut().create_user_account(&caller_balance);

    attacker_sc.participate(&caller, &attacker_sc.lottery_address).assert_ok();

    b_mock_rc.borrow_mut().set_egld_balance(&attacker_sc.lottery_address, &(rust_biguint!(THOUSAND_EGLD) * rust_biguint!(10)));

    attacker_sc.call_draw_winner(&caller, &attacker_sc.lottery_address, ONE_EGLD).assert_ok();
    
    b_mock_rc.borrow().check_egld_balance(&caller, &rust_biguint!(999999));
}