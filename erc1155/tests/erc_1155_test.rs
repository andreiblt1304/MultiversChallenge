#![allow(deprecated)]
use std::{cell::RefCell, rc::Rc};

use erc_1155_setup::Erc1155Setup;
use multiversx_sc_scenario::{self, DebugApi, testing_framework::BlockchainStateWrapper, rust_biguint, scenario_model::BigUintValue};
static TOKEN_ID: &[u8] = b"MOCKTOKEN-123";

mod erc_1155_setup;

fn init_all<
    Erc1155ObjBuilder: 'static + Copy + Fn() -> erc1155::ContractObj<DebugApi>,
>(
    erc_builder: Erc1155ObjBuilder
) -> (
        Rc<RefCell<BlockchainStateWrapper>>,
        Erc1155Setup<Erc1155ObjBuilder>
    ) {
    let mut b_mock = BlockchainStateWrapper::new();
    let owner = b_mock.create_user_account(&rust_biguint!(0));
    let b_mock_ref = RefCell::new(b_mock);
    let b_mock_rc = Rc::new(b_mock_ref);
    let erc_sc = Erc1155Setup::new(
        b_mock_rc.clone(),
        erc_builder,
        &owner
    );

    b_mock_rc
        .borrow_mut()
        .execute_tx(
            &owner, 
            &erc_sc.erc_wrapper, 
            &rust_biguint!(0), 
            |sc| {}
        )
        .assert_ok();

    (b_mock_rc, erc_sc)
}

#[test]
fn init_test() {
    let _ = init_all(|| erc1155::contract_obj());
}

#[test]
fn deposit_ok() {
    let rust_zero = &rust_biguint!(0);
    let (b_mock_rc, mut erc_sc) = init_all(erc1155::contract_obj);
    let amount = 64;
    let user = b_mock_rc.borrow_mut().create_user_account(rust_zero);
    
    b_mock_rc
        .borrow_mut()
        .set_esdt_balance(&user, TOKEN_ID, &rust_biguint!(1000000));

    erc_sc.call_deposit(&user, TOKEN_ID, amount).assert_ok();

    b_mock_rc
        .borrow()
        .check_esdt_balance(erc_sc.erc_wrapper.address_ref(), TOKEN_ID, &rust_biguint!(1000000));
}