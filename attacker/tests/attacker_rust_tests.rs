#![allow(deprecated)]

use std::{cell::RefCell, rc::Rc};

use attacker_setup::{AttackerSetup, ONE_EGLD, TEN_EGLD};
use multiversx_sc_scenario::{rust_biguint, testing_framework::BlockchainStateWrapper, DebugApi};

pub mod attacker_setup;
pub mod lottery_setup;

fn init_all<
    AttackerObjBuilder: 'static + Copy + Fn() -> attacker::ContractObj<DebugApi>,
    LotteryObjBuilder: 'static + Copy + Fn() -> lottery::ContractObj<DebugApi>,
>(
    attacker_builder: AttackerObjBuilder,
    lottery_builder: LotteryObjBuilder,
) -> (
    Rc<RefCell<BlockchainStateWrapper>>,
    AttackerSetup<AttackerObjBuilder>,
) {
    let b_mock = BlockchainStateWrapper::new();
    let b_mock_ref = RefCell::new(b_mock);
    let b_mock_rc = Rc::new(b_mock_ref);

    let attacker_owner = b_mock_rc
        .borrow_mut()
        .create_user_account(&rust_biguint!(0));

    let attacker_sc = AttackerSetup::new(
        b_mock_rc.clone(),
        attacker_builder,
        lottery_builder,
        &attacker_owner,
    );

    (b_mock_rc, attacker_sc)
}

#[test]
fn init_test() {
    let _ = init_all(|| attacker::contract_obj(), || lottery::contract_obj());
}

#[test]
fn participating_tests_setup() {
    let (b_mock_rc, attacker_sc) =
        init_all(|| attacker::contract_obj(), || lottery::contract_obj());
    let caller_balance = rust_biguint!(10) * ONE_EGLD;
    let caller = b_mock_rc.borrow_mut().create_user_account(&caller_balance);
    let sc_balance_before_participating = b_mock_rc
        .borrow()
        .get_egld_balance(&attacker_sc.lottery_address);
    let amount_to_send = ONE_EGLD;

    attacker_sc
        .call_participate(&caller, &attacker_sc.lottery_address, amount_to_send)
        .assert_ok();

    let sc_balance_after_participating = b_mock_rc
        .borrow()
        .get_egld_balance(&attacker_sc.lottery_address);

    b_mock_rc
        .borrow()
        .check_egld_balance(&caller, &rust_biguint!(TEN_EGLD - ONE_EGLD));
    assert!(sc_balance_before_participating < sc_balance_after_participating);
}

#[test]
fn redeem_prize_one_participant() {
    let (b_mock_rc, attacker_sc) =
        init_all(|| attacker::contract_obj(), || lottery::contract_obj());

    let caller_balance = rust_biguint!(10) * ONE_EGLD;
    let caller = b_mock_rc.borrow_mut().create_user_account(&caller_balance);
    let amount_to_send = ONE_EGLD;

    attacker_sc
        .call_participate(&caller, &attacker_sc.lottery_address, amount_to_send)
        .assert_ok();

    attacker_sc
        .call_draw_winner(&attacker_sc.owner_address, &attacker_sc.lottery_address)
        .assert_error(4, "There should be more than 1 participants");
}

#[test]
fn redeem_prize_two_participants() {
    let (b_mock_rc, attacker_sc) =
        init_all(|| attacker::contract_obj(), || lottery::contract_obj());

    let first_caller_balance = rust_biguint!(10) * ONE_EGLD;
    let first_caller = b_mock_rc
        .borrow_mut()
        .create_user_account(&first_caller_balance);
    let first_amount_to_send = ONE_EGLD;

    let second_caller_balance = rust_biguint!(10) * ONE_EGLD;
    let second_caller = b_mock_rc
        .borrow_mut()
        .create_user_account(&second_caller_balance);
    let second_amount_to_send = ONE_EGLD;

    attacker_sc
        .call_participate(
            &first_caller,
            &attacker_sc.lottery_address,
            first_amount_to_send,
        )
        .assert_ok();

    b_mock_rc
        .borrow_mut()
        .set_egld_balance(&attacker_sc.owner_address, &(rust_biguint!(10) * ONE_EGLD));

    attacker_sc
        .call_participate(
            &second_caller,
            &attacker_sc.lottery_address,
            second_amount_to_send,
        )
        .assert_ok();

    let _ = attacker_sc.call_draw_winner(&attacker_sc.owner_address, &attacker_sc.lottery_address);

    let _ = attacker_sc.call_redeem_prize(&first_caller, &attacker_sc.lottery_address);

    let first_caller_balance_after_withrawall = b_mock_rc.borrow().get_egld_balance(&first_caller);

    let second_caller_balance_after_withrawall =
        b_mock_rc.borrow().get_egld_balance(&second_caller);
    assert_ne!(
        first_caller_balance_after_withrawall,
        second_caller_balance_after_withrawall
    );
}

#[test]
fn attack() {
    let (b_mock_rc, attacker_sc) =
        init_all(|| attacker::contract_obj(), || lottery::contract_obj());

    let first_caller_balance = rust_biguint!(10) * ONE_EGLD;
    let first_caller = b_mock_rc
        .borrow_mut()
        .create_user_account(&first_caller_balance);
    let first_amount_to_send = ONE_EGLD;

    let second_caller_balance = rust_biguint!(10) * ONE_EGLD;
    let second_caller = b_mock_rc
        .borrow_mut()
        .create_user_account(&second_caller_balance);
    let second_amount_to_send = ONE_EGLD;

    attacker_sc
        .call_participate(
            &first_caller,
            &attacker_sc.lottery_address,
            first_amount_to_send,
        )
        .assert_ok();

    b_mock_rc
        .borrow_mut()
        .set_egld_balance(&attacker_sc.owner_address, &(rust_biguint!(10) * ONE_EGLD));

    attacker_sc
        .call_participate(
            &second_caller,
            &attacker_sc.lottery_address,
            second_amount_to_send,
        )
        .assert_ok();

    attacker_sc
        .call_attacker_async(
            &first_caller,
            &attacker_sc.lottery_address,
            first_amount_to_send,
        )
        .assert_ok();

    let sc_balance_before_participating = b_mock_rc
        .borrow()
        .get_egld_balance(&attacker_sc.lottery_address);

    let sc_balance_after_participating = b_mock_rc
        .borrow()
        .get_egld_balance(&attacker_sc.lottery_address);

    b_mock_rc.borrow().check_egld_balance(
        &first_caller,
        &rust_biguint!(TEN_EGLD - ONE_EGLD - ONE_EGLD),
    );
    assert!(sc_balance_before_participating < sc_balance_after_participating);
}
