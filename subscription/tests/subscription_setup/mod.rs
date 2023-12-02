#![allow(deprecated)]

use std::{cell::RefCell, rc::Rc, fmt::Debug};

use multiversx_sc::{types::{MultiValueEncoded, EgldOrEsdtTokenIdentifier, Address, TokenIdentifier, ManagedAddress, BigUint}, codec::multi_types::MultiValue3};
use multiversx_sc_scenario::{testing_framework::{BlockchainStateWrapper, ContractObjWrapper, TxResult}, DebugApi, rust_biguint, managed_address, managed_token_id_wrapped, managed_biguint};
use subscription::{self, Subscription, service::ServiceModule};

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
            "some wasm path"
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
}