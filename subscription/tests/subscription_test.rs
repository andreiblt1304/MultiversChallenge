#![allow(deprecated)]

use std::{cell::RefCell, rc::Rc};

use multiversx_sc_scenario::{DebugApi, testing_framework::BlockchainStateWrapper, rust_biguint, managed_token_id, managed_address};
use pair_setup::PairSetup;
use subscription::{pair_actions::{self, PairActionsModule}, ContractObj, service::SubscriptionType};
use subscription_setup::SubscriptionSetup;

mod subscription_setup;
mod pair_setup;

static FIRST_TOKEN_ID: &[u8] = b"MYTOKEN-123456";
static USDC_TOKEN_ID: &[u8] = b"USDC-123456";
static LP_TOKEN_ID: &[u8] = b"LPTOK-123456";

fn init_all<
    PairObjBuilder: 'static + Copy + Fn() -> pair_actions::ContractObj<DebugApi>,
    SubscriptionObjBuilder: 'static + Copy + Fn() -> subscription::ContractObj<DebugApi>
> (
    pair_builder: PairObjBuilder,
    sub_builder: SubscriptionObjBuilder
) -> (
    Rc<RefCell<BlockchainStateWrapper>>,
    PairSetup<PairObjBuilder>,
    SubscriptionSetup<SubscriptionObjBuilder>
) {
    let mut b_mock = BlockchainStateWrapper::new();
    let owner = b_mock.create_user_account(&rust_biguint!(0));

    let b_mock_ref = RefCell::new(b_mock);
    let b_mock_rc = Rc::new(b_mock_ref);
    let pair_setup = PairSetup::new(
        b_mock_rc.clone(),
        pair_builder,
        &owner,
        FIRST_TOKEN_ID,
        USDC_TOKEN_ID,
        LP_TOKEN_ID,
        1000000000,
        2000000000
    );

    let sub_sc = SubscriptionSetup::new(
        b_mock_rc.clone(),
        sub_builder,
        &owner,
        pair_setup.pair_wrapper.address_ref(),
        vec![FIRST_TOKEN_ID.to_vec()]
    );

    b_mock_rc
        .borrow_mut()
        .execute_tx(
            &owner,
            &sub_sc.sub_wrapper,
            &rust_biguint!(0),
            |sc: ContractObj<DebugApi>| {
            sc.add_pair(
                managed_token_id!(FIRST_TOKEN_ID),
                managed_address!(pair_setup.pair_wrapper.address_ref()),
            );
        })
        .assert_ok();

    (b_mock_rc, pair_setup, sub_sc)
}

// fn service_register_setup() {
//     let (b_mock_rc, pair_setup, mut sub_sc) = 
//         init_all(|| pair_actions::contract_obj(), || subscription::contract_obj());
    
//     let rust_zero = rust_biguint!(0);

//     let rand_service = b_mock_rc.borrow_mut().create_user_account(&rust_zero)

//     sub_sc
//         .call_register_service(
//             &rand_service,
//             vec![(
//                 pair_setup.pair_wrapper.address_ref().clone(),
//                 Some(FIRST_TOKEN_ID.to_vec()),
//                 1000
//             )]
//         )
//         .assert_ok();
// }

#[test]
fn init_test() {
    let _ = init_all(|| pair_actions::contract_obj(), || subscription::contract_obj());
}

#[test]
fn register_service_test() {
    let (b_mock_rc, pair_setup, mut sub_sc) = 
        init_all(|| pair_actions::contract_obj(), || subscription::contract_obj());
    
    let rust_zero = rust_biguint!(0);

    let rand_service = b_mock_rc
        .borrow_mut()
        .create_user_account(&rust_zero);
    
    sub_sc
        .call_register_service(
            &rand_service,
            vec![(
                pair_setup.pair_wrapper.address_ref().clone(),
                Some(FIRST_TOKEN_ID.to_vec()),
                1000
            )]
        )
        .assert_ok();
}

#[test]
fn subscribe_before_deposit_test() {
    let (b_mock_rc, pair_setup, mut sub_sc) = 
        init_all(|| pair_actions::contract_obj(), || subscription::contract_obj());
    
    let rust_zero = rust_biguint!(0);

    let rand_service = b_mock_rc.borrow_mut().create_user_account(&rust_zero);

    sub_sc
        .call_register_service(
            &rand_service,
            vec![(
                pair_setup.pair_wrapper.address_ref().clone(),
                Some(FIRST_TOKEN_ID.to_vec()),
                1000
            )]
        )
        .assert_ok();

    let user = b_mock_rc.borrow_mut().create_user_account(&rust_zero);
    
    sub_sc
        .call_subscribe(
            &user,
            vec![(1, 0, SubscriptionType::Daily)])
        .assert_user_error("Unknown address");
}

#[test]
fn subscribe_ok_test() {
    let (b_mock_rc, pair_setup, mut sub_sc) = 
        init_all(|| pair_actions::contract_obj(), || subscription::contract_obj());

    let rust_zero = rust_biguint!(0);

    let rand_service = b_mock_rc.borrow_mut().create_user_account(&rust_zero);

    sub_sc
        .call_register_service(
            &rand_service,
            vec![(
                pair_setup.pair_wrapper.address_ref().clone(),
                Some(FIRST_TOKEN_ID.to_vec()),
                1000
            )]
        )
        .assert_ok();

    let user = b_mock_rc.borrow_mut().create_user_account(&rust_zero);

    b_mock_rc
        .borrow_mut()
        .set_esdt_balance(&user, FIRST_TOKEN_ID, &rust_biguint!(1000000));

    sub_sc
        .call_deposit(&user, FIRST_TOKEN_ID, 1000000)
        .assert_ok();

    sub_sc
        .call_subscribe(&user, vec![(1, 0, SubscriptionType::Daily)])
        .assert_ok();    
}

#[test]
fn substract_ok_test() {
    let (b_mock_rc, pair_setup, mut sub_sc) = 
        init_all(|| pair_actions::contract_obj(), || subscription::contract_obj());

    let rust_zero = rust_biguint!(0);

    let rand_service = b_mock_rc.borrow_mut().create_user_account(&rust_zero);

    sub_sc
        .call_register_service(
            &rand_service,
            vec![(
                pair_setup.pair_wrapper.address_ref().clone(),
                Some(FIRST_TOKEN_ID.to_vec()),
                1000
            )]
        )
        .assert_ok();

    let user = b_mock_rc.borrow_mut().create_user_account(&rust_zero);

    b_mock_rc
        .borrow_mut()
        .set_esdt_balance(&user, FIRST_TOKEN_ID, &rust_biguint!(1000000));

    sub_sc
        .call_deposit(&user, FIRST_TOKEN_ID, 1000000)
        .assert_ok();

    sub_sc
        .call_subscribe(&user, vec![(1, 0, SubscriptionType::Daily)])
        .assert_ok();    

    b_mock_rc.borrow_mut().set_block_epoch(10);

    sub_sc
        .call_substract_payment(&rand_service, 0, 1)
        .assert_ok();

    b_mock_rc.borrow().check_esdt_balance(
        &rand_service,
        FIRST_TOKEN_ID,
        &rust_biguint!(30* 1000)
    );
}

#[test]
fn try_subtract_twice_same_day() {
    let (b_mock_rc, pair_setup, mut sub_sc) = 
        init_all(|| pair_actions::contract_obj(), || subscription::contract_obj());

    let rust_zero = rust_biguint!(0);

    let rand_service = b_mock_rc.borrow_mut().create_user_account(&rust_zero);
    sub_sc
        .call_register_service(
            &rand_service,
            vec![(
                pair_setup.pair_wrapper.address_ref().clone(),
                Some(FIRST_TOKEN_ID.to_vec()),
                1_000,
            )],
        )
        .assert_ok();

    let user = b_mock_rc.borrow_mut().create_user_account(&rust_zero);

    b_mock_rc
        .borrow_mut()
        .set_esdt_balance(&user, FIRST_TOKEN_ID, &rust_biguint!(1000000));

    sub_sc
        .call_deposit(&user, FIRST_TOKEN_ID, 1000000)
        .assert_ok();

    sub_sc
        .call_subscribe(&user, vec![(1, 0, SubscriptionType::Daily)])
        .assert_ok();

    b_mock_rc.borrow_mut().set_block_epoch(10);

    sub_sc
        .call_substract_payment(&rand_service, 0, 1)
        .assert_ok();

    b_mock_rc.borrow().check_esdt_balance(
        &rand_service,
        FIRST_TOKEN_ID,
        &rust_biguint!(30 * 1_000),
    );

    sub_sc
        .call_substract_payment(&rand_service, 0, 1)
        .assert_user_error("Cannot substract the payment yet");

    b_mock_rc.borrow().check_esdt_balance(
        &rand_service,
        FIRST_TOKEN_ID,
        &rust_biguint!(30 * 1_000),
    );
}

#[test]
fn withdraw_tokens_test() {
    let (b_mock_rc, pair_setup, mut sub_sc) = 
        init_all(|| pair_actions::contract_obj(), || subscription::contract_obj());

    let rust_zero = rust_biguint!(0);

    let rand_service = b_mock_rc.borrow_mut().create_user_account(&rust_zero);
    sub_sc
        .call_register_service(
            &rand_service,
            vec![(
                pair_setup.pair_wrapper.address_ref().clone(),
                Some(FIRST_TOKEN_ID.to_vec()),
                1_000,
            )],
        )
        .assert_ok();

    let user = b_mock_rc.borrow_mut().create_user_account(&rust_zero);
    b_mock_rc
        .borrow_mut()
        .set_esdt_balance(&user, FIRST_TOKEN_ID, &rust_biguint!(1_000_000));

    sub_sc
        .call_deposit(&user, FIRST_TOKEN_ID, 1_000_000)
        .assert_ok();

    sub_sc
        .call_subscribe(&user, vec![(1, 0, SubscriptionType::Daily)])
        .assert_ok();

    b_mock_rc.borrow_mut().set_block_epoch(10);

    sub_sc
        .call_withdraw_funds(&user, vec![(FIRST_TOKEN_ID.to_vec(), 999_999)])
        .assert_ok();

    b_mock_rc
        .borrow()
        .check_esdt_balance(&user, FIRST_TOKEN_ID, &rust_biguint!(999_999));

    b_mock_rc
        .borrow()
        .check_esdt_balance(&rand_service, FIRST_TOKEN_ID, &rust_biguint!(0));
}