#![allow(deprecated)]
use std::cell::RefCell;

use erc_1155_setup::Erc1155Setup;
use multiversx_sc_scenario::{self, DebugApi, testing_framework::BlockchainStateWrapper, rust_biguint};

mod erc_1155_setup;

fn init_all<
    Erc1155ObjBuilder: 'static + Copy + Fn() -> erc1155::ContractObj<DebugApi>,
>(
    erc_builder: Erc1155ObjBuilder
) -> RefCell<BlockchainStateWrapper> {
    let mut b_mock = BlockchainStateWrapper::new();
    let owner = b_mock.create_user_account(&rust_biguint!(0));
    let b_mock_ref = RefCell::new(b_mock);
    let erc_sc = Erc1155Setup::new(
        b_mock_ref,
        erc_builder,
        &owner
    );

    b_mock_ref
        .borrow_mut()
        .execute_tx(
            &owner, 
            &erc_sc.erc_wrapper, 
            &rust_biguint!(0), 
            |sc| {}
        )
        .assert_ok();

    b_mock_ref
}

#[test]
fn init_test() {
    let _ = init_all(|| erc1155::contract_obj());
}

#[test]
fn deposit_ok() {
    let b_mock = init_all(erc1155::contract_obj);
}