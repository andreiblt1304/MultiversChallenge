multiversx_sc::imports!();

use lottery::ProxyTrait as _;

#[multiversx_sc::module]
pub trait LotteryProxy<RequestResult>:
    //crate::common_storage::CommonStorageModule
{
    #[proxy]
    fn lottery_contract_proxy(&self, lottery_sc_address: ManagedAddress) -> lottery::Proxy<Self::Api>;

    #[endpoint(participate)]
    fn draw_winner_endpoint(
        &self,
        lottery_sc_address: ManagedAddress
    ) {
        self.lottery_contract_proxy(lottery_sc_address)
            .draw_winner()
            .async_call()
            .call_and_exit();
    }

    #[view(getLotteryScAddress)]
    #[storage_mapper("lotteryScAddress")]
    fn lottery_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}