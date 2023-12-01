#![no_std]

pub mod payments;

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Subscription:
    payments::payments::PaymentsModule
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

        //self.price_query_address().set(price_query_address);
        self.add_accepted_payment_tokens(accepted_tokens);
    }
}
