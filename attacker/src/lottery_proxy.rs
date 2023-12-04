multiversx_sc::imports!();
use multiversx_sc::types::heap::Address;
use lottery::ProxyTrait as _;
pub const ONE_EGLD: u64 = 1_000_000_000_000_000_000;

#[multiversx_sc::module]
pub trait LotteryProxy {
    #[proxy]
    fn lottery_contract_proxy(&self, lottery_sc_address: ManagedAddress) -> lottery::Proxy<Self::Api>;

    #[payable("*EGLD")]
    #[endpoint(participate)]
    fn participate(
        &self,
        lottery_sc_address: ManagedAddress
    ) {
        self
            .send().direct_egld(&lottery_sc_address, &BigUint::from(ONE_EGLD))
    }

    #[payable("EGLD")]
    #[endpoint(drawWinnerEndpoint)]
    fn draw_winner_endpoint(
        &self,
        lottery_sc_address: ManagedAddress,
        amount: BigUint
    ) {
        self
            .lottery_contract_proxy(lottery_sc_address)
            .draw_winner()
            .with_egld_transfer(amount)
            .execute_on_dest_context()
    }

    #[payable("EGLD")]
    #[endpoint]
    fn fund_lottery(
        &self,
        amount: BigUint,
        lottery_sc_address: ManagedAddress
    ) {
        self
            .send().direct_egld(&lottery_sc_address, &amount);
    }

    #[callback]
    fn draw_winner_endpoint_callback(
        &self,
        _caller: &Address,
        #[call_result] result: ManagedAsyncCallResult<()>
    ) -> CallbackClosure<Self::Api> {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                CallbackClosure::<Self::Api>::new("callback_name")
            }
            ManagedAsyncCallResult::Err(_) => {
                CallbackClosure::<Self::Api>::new("callback_name")
            }
        }
    }

    #[view(getLotteryScAddress)]
    #[storage_mapper("lotteryScAddress")]
    fn lottery_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}