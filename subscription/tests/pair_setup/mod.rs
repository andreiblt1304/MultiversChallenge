#![allow(deprecated)]

use std::{cell::RefCell, rc::Rc};
extern crate subscription;
use subscription::pair_actions;
use multiversx_sc::types::{Address, EsdtLocalRole};
use multiversx_sc_scenario::{
    DebugApi, 
    testing_framework::{
        BlockchainStateWrapper, ContractObjWrapper
    }, 
    rust_biguint
};

pub struct PairSetup<PairObjBuilder>
where
    PairObjBuilder: 'static + Copy + Fn() -> pair_actions::ContractObj<DebugApi>,
{
    pub b_mock: Rc<RefCell<BlockchainStateWrapper>>,
    pub first_token_id: Vec<u8>,
    pub second_token_id: Vec<u8>,
    pub lp_token_id: Vec<u8>,
    pub pair_wrapper: ContractObjWrapper<pair_actions::ContractObj<DebugApi>, PairObjBuilder>
}

impl<PairObjBuilder> PairSetup<PairObjBuilder>
where
    PairObjBuilder: 'static + Copy + Fn() -> pair_actions::ContractObj<DebugApi>
{
    pub fn new(
        b_mock: Rc<RefCell<BlockchainStateWrapper>>,
        pair_builder: PairObjBuilder,
        owner: &Address,
        first_token_id: &[u8],
        second_token_id: &[u8],
        lp_token_id: &[u8],
        first_token_amount: u64,
        second_token_amount: u64
    ) -> Self {
        let rust_zero = rust_biguint!(0);
        let pair_wrapper = 
            b_mock
                .borrow_mut()
                .create_sc_account(&rust_zero, Some(owner), pair_builder, "pair");

            let lp_token_roles = [EsdtLocalRole::Mint, EsdtLocalRole::Burn];
            b_mock.borrow_mut().set_esdt_local_roles(
                pair_wrapper.address_ref(),
                lp_token_id,
                &lp_token_roles[..],
            );
    
            let pair_setup = PairSetup {
                b_mock: b_mock.clone(),
                first_token_id: first_token_id.to_vec(),
                second_token_id: second_token_id.to_vec(),
                lp_token_id: lp_token_id.to_vec(),
                pair_wrapper,
            };
    
            b_mock.borrow_mut().set_esdt_balance(
                owner,
                first_token_id,
                &rust_biguint!(first_token_amount),
            );
            b_mock.borrow_mut().set_esdt_balance(
                owner,
                second_token_id,
                &rust_biguint!(second_token_amount),
            );
            let block_round = 1;
            b_mock.borrow_mut().set_block_round(block_round);
    
        pair_setup
    }
}