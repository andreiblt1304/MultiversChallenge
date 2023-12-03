#![no_std]

multiversx_sc::imports!();

pub mod lottery_proxy;

#[multiversx_sc::contract]
pub trait Attacker:
    lottery_proxy::LotteryProxy
{
    #[init]
    fn init(
        &self,
        sc_address: ManagedAddress
    ) {
        require!(
            self.blockchain().is_smart_contract(&sc_address),
            "Invalid Lottery SC address"
        );

        self.lottery_sc_address().set(&sc_address);
    }
}
