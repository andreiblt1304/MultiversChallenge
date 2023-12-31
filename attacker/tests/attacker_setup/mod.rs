//#![allow(deprecated)]

use std::{cell::RefCell, rc::Rc};

use attacker::{lottery_proxy::LotteryProxy, Attacker};
use multiversx_sc::types::{Address, BigUint, ManagedAddress};
use multiversx_sc_scenario::{
    rust_biguint,
    testing_framework::{BlockchainStateWrapper, ContractObjWrapper, TxResult},
    DebugApi,
};

pub const ONE_EGLD: u64 = 1_000_000_000_000_000_000;
pub const TEN_EGLD: u64 = 10_000_000_000_000_000_000;

pub struct AttackerSetup<AttackerObjBuilder>
where
    AttackerObjBuilder: 'static + Copy + Fn() -> attacker::ContractObj<DebugApi>,
{
    pub b_mock: Rc<RefCell<BlockchainStateWrapper>>,
    pub owner_address: Address,
    pub lottery_address: Address,
    pub attacker_wrapper: ContractObjWrapper<attacker::ContractObj<DebugApi>, AttackerObjBuilder>,
}

impl<AttackerObjBuilder> AttackerSetup<AttackerObjBuilder>
where
    AttackerObjBuilder: 'static + Copy + Fn() -> attacker::ContractObj<DebugApi>,
{
    pub fn new<LotteryObjBuilder>(
        b_mock: Rc<RefCell<BlockchainStateWrapper>>,
        attacker_builder: AttackerObjBuilder,
        lottery_builder: LotteryObjBuilder,
        owner_address: &Address,
    ) -> Self
    where
        LotteryObjBuilder: 'static + Copy + Fn() -> lottery::ContractObj<DebugApi>,
    {
        let rust_zero = rust_biguint!(0);
        let attacker_wrapper = b_mock.borrow_mut().create_sc_account(
            &(rust_biguint!(ONE_EGLD) * rust_biguint!(100)),
            Some(&owner_address),
            attacker_builder,
            "../output/attacker.wasm",
        );

        let lottery_wrapper = b_mock.borrow_mut().create_sc_account(
            &(rust_biguint!(ONE_EGLD) * rust_biguint!(100)),
            Some(&owner_address),
            lottery_builder,
            "../../lottery/output/lottery.wasm",
        );

        b_mock
            .borrow_mut()
            .execute_tx(&owner_address, &attacker_wrapper, &rust_zero, |sc| {
                sc.init(attacker_wrapper.address_ref().into())
            })
            .assert_ok();

        Self {
            b_mock,
            owner_address: owner_address.clone(),
            lottery_address: lottery_wrapper.address_ref().clone(),
            attacker_wrapper,
        }
    }

    pub fn call_participate(
        &self,
        participant: &Address,
        lottery_sc_address: &Address,
        amount: u64,
    ) -> TxResult {
        self.b_mock.borrow_mut().execute_tx(
            participant,
            &self.attacker_wrapper,
            &rust_biguint!(ONE_EGLD),
            |sc| {
                sc.participate(
                    participant.into(),
                    lottery_sc_address.clone().into(),
                    BigUint::from(amount),
                );
            },
        )
    }

    pub fn call_draw_winner(
        &self,
        owner_address: &Address,
        lottery_sc_address: &Address,
    ) -> TxResult {
        self.b_mock.borrow_mut().execute_tx(
            owner_address,
            &self.attacker_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.draw_winner(lottery_sc_address.clone().into());
            },
        )
    }

    pub fn call_redeem_prize(
        &self,
        participant: &Address,
        lottery_sc_address: &Address,
    ) -> TxResult {
        self.b_mock.borrow_mut().execute_tx(
            participant,
            &self.attacker_wrapper,
            &rust_biguint!(0),
            |sc| sc.redeem_prize(participant.into(), lottery_sc_address.clone().into()),
        )
    }

    pub fn call_attacker_async(
        &self,
        caller: &Address,
        lottery_sc_address: &Address,
        amount: u64,
    ) -> TxResult {
        self.b_mock.borrow_mut().execute_tx(
            caller,
            &self.attacker_wrapper,
            &rust_biguint!(amount),
            |sc| sc.attack_async(ManagedAddress::from(lottery_sc_address.clone())),
        )
    }
}
