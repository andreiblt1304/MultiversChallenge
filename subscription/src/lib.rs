#![no_std]

pub mod payments;

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Subscription:
{
    #[init]
    fn init(
        &self,
        price_query_address: ManagedAddress,
        accepted_tokens: MultiValueEncoded<EgldOrEsdtTokenIdentifier>
    ) {
        require!(
            self.blockchain().is_smart_contract(&price_query_address),
            "Invalid price query address"
        );

        // self.price_query_address().set(price_query_address);
        // self.add_accepted_payments_tokens(accepted_tokens);
    }
}
