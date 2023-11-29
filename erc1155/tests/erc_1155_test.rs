#![allow(deprecated)]
use std::{cell::RefCell, rc::Rc};

use erc_1155_setup::Erc1155Setup;
use multiversx_sc_scenario::{self, DebugApi, testing_framework::BlockchainStateWrapper, rust_biguint};

mod erc_1155_setup;

fn init_all<
    Erc1155ObjBuilder: 'static + Copy + Fn() -> erc1155::ContractObj<DebugApi>,
>(
    erc_builder: Erc1155ObjBuilder
) -> Rc<RefCell<BlockchainStateWrapper>> {
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

    b_mock_rc
}

#[test]
fn init_test() {
    let _ = init_all(|| erc1155::contract_obj());
}

#[test]
fn deposit_ok() {
    let b_mock = init_all(erc1155::contract_obj);
}