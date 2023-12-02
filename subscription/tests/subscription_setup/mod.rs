#![allow(deprecated)]

use std::{cell::RefCell, rc::Rc};

use multiversx_sc::{types::{MultiValueEncoded, EgldOrEsdtTokenIdentifier, Address, TokenIdentifier, ManagedAddress, BigUint}, codec::multi_types::{MultiValue3, MultiValue2}, storage::mappers::AddressId};
use multiversx_sc_scenario::{testing_framework::{BlockchainStateWrapper, ContractObjWrapper, TxResult}, DebugApi, rust_biguint, managed_address, managed_token_id_wrapped, managed_biguint};
use subscription::{self, Subscription, service::{ServiceModule, SubscriptionType}, payments::{payments::PaymentsModule, substract_payments::SubstractPaymentsModule}};

pub struct SubscriptionSetup<SubscriptionObjBuilder>
where
    SubscriptionObjBuilder: 'static + Copy + Fn() -> subscription::ContractObj<DebugApi>,
{
    pub b_mock: Rc<RefCell<BlockchainStateWrapper>>,
    pub owner_address: Address,
    pub sub_wrapper:
        ContractObjWrapper<subscription::ContractObj<DebugApi>, SubscriptionObjBuilder>,
}

impl<SubscriptionObjBuilder> SubscriptionSetup<SubscriptionObjBuilder>
where
    SubscriptionObjBuilder: 'static + Copy + Fn() -> subscription::ContractObj<DebugApi>
{
    pub fn new(
        b_mock: Rc<RefCell<BlockchainStateWrapper>>,
        builder: SubscriptionObjBuilder,
        owner_address: &Address,
        pair_address: &Address,
        accepted_tokens: Vec<Vec<u8>>
    ) -> Self {
        let rust_zero = rust_biguint!(0);
        let sub_wrapper = b_mock.borrow_mut().create_sc_account(
            &rust_zero,
            Some(owner_address),
            builder,
            "../output/subscription.wasm"
        );

        b_mock
            .borrow_mut()
            .execute_tx(
                owner_address,
                &sub_wrapper,
                &rust_zero,
                |sc| {
                    let mut args = MultiValueEncoded::new();

                    for arg in accepted_tokens {
                        if &arg == b"EGLD" {
                            let token_id = EgldOrEsdtTokenIdentifier::egld();
                            args.push(token_id);
                        } else {
                            let token_id = TokenIdentifier::from_esdt_bytes(arg);
                            args.push(EgldOrEsdtTokenIdentifier::esdt(token_id));
                        }
                    }

                    sc.init(managed_address!(pair_address), args);
                }
            )
            .assert_ok();

        Self {
            b_mock,
            owner_address: owner_address.clone(),
            sub_wrapper,
        }
    }

    pub fn call_register_service(
        &mut self,
        caller: &Address,
        args: Vec<(Address, Option<Vec<u8>>, u64)>,
    ) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(
                caller,
                &self.sub_wrapper,
                &rust_biguint!(0),
                |sc| {
                    let mut args_encoded = 
                        MultiValueEncoded::<
                        DebugApi, 
                        MultiValue3<ManagedAddress<DebugApi>, Option<EgldOrEsdtTokenIdentifier<DebugApi>>, BigUint<DebugApi>>
                        >::new();
                    for arg in args {
                        let (sc_address, opt_token_id, value) = arg;
                        args_encoded.push(
                            MultiValue3((
                                managed_address!(&sc_address),
                                opt_token_id.map(|token_id| managed_token_id_wrapped!(token_id)),
                                managed_biguint!(value)
                            ))
                        )
                    }

                    sc.register_service(args_encoded);
                }
            )
    }

    pub fn call_deposit(
        &self,
        caller: &Address,
        token_id: &[u8],
        amount: u64
    ) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_esdt_transfer(
                caller,
                &self.sub_wrapper,
                token_id,
                0,
                &rust_biguint!(amount),
                |sc| {
                    sc.deposit();
                }
            )
    }

    pub fn call_subscribe(
        &mut self,
        caller: &Address,
        args: Vec<(AddressId, usize, SubscriptionType)>
    ) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(
                caller,
                &self.sub_wrapper,
                &rust_biguint!(0),
                |sc| {
                    let mut managed_services = 
                        MultiValueEncoded::<DebugApi, MultiValue3<AddressId, usize, SubscriptionType>>::new();
                    for arg in args {
                        managed_services.push((arg.0, arg.1, arg.2).into())
                    }

                    sc.subscribe(managed_services)
                })
    }

    pub fn call_substract_payment(
        &mut self,
        caller: &Address,
        service_index: usize,
        user_id: AddressId
    ) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(
                caller,
                &self.sub_wrapper,
                &rust_biguint!(0),
            |sc| {
                let _ = sc.substract_payment(service_index, user_id);
            })
    }

    pub fn call_withdraw_funds(
        &mut self,
        caller: &Address,
        tokens: Vec<(Vec<u8>, u64)>
    ) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(
                caller,
                &self.sub_wrapper,
                &rust_biguint!(0),
                |sc| {
                    let mut tokens_to_withdraw = 
                        MultiValueEncoded::<DebugApi, MultiValue2<EgldOrEsdtTokenIdentifier<DebugApi>, BigUint<DebugApi>>>::new();
                    for token in tokens {
                        tokens_to_withdraw.push(
                            (
                                managed_token_id_wrapped!(token.0),
                                managed_biguint!(token.1)
                            )
                            .into()
                        )
                    }

                    let _ = sc.withdraw(tokens_to_withdraw);
                })
    }
}