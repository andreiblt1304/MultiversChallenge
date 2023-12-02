#![no_std]

pub mod payments;
pub mod service;
pub mod pair_actions;

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Subscription:
    payments::payments::PaymentsModule
    + service::ServiceModule
    + pair_actions::PairActionsModule
    + payments::substract_payments::SubstractPaymentsModule
    //+ crate::ongoing_operation::OngoingOperationModule
{
    #[init]
    fn init(
        &self,
        price_query_address: ManagedAddress<Self::Api>,
        accepted_tokens: MultiValueEncoded<Self::Api, EgldOrEsdtTokenIdentifier<Self::Api>>
    ) {
        require!(
            self.blockchain().is_smart_contract(&price_query_address),
            "Invalid price query address"
        );

        self.price_query_address().set(price_query_address);
        self.add_accepted_payment_tokens(accepted_tokens);
    }
}
