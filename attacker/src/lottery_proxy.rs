multiversx_sc::imports!();

use lottery::ProxyTrait as _;

#[multiversx_sc::module]
pub trait LotteryProxy<RequestResult>:
    //crate::common_storage::CommonStorageModule
{
    #[proxy]
    fn lottery_contract_proxy(&self, lottery_sc_address: ManagedAddress) -> lottery::Proxy<Self::Api>;

    #[payable("*EGLD")]
    #[endpoint(participate)]
    fn participate(
        &self,
        lottery_sc_address: ManagedAddress
    ) {
        self
            .send().direct_egld(&lottery_sc_address, &BigUint::from(1u64))
            //.lottery_contract_proxy(lottery_sc_address.clone())
            //.async_call()
            // .with_callback(
            //     self.callbacks()
            //         .draw_winner_callback()
            // )
            //.call_and_exit();
    }

    // #[callback]
    // fn draw_winner_callback(
    //     &self,
    //     #[call_result] result: ManagedAsyncCallResult<()>
    // ) { 
    //     let amount = self.call_value().egld_value();
    //     match result {
    //         ManagedAsyncCallResult::Ok(()) => {
                
    //         }
    //     }
    // }

    #[view(getLotteryScAddress)]
    #[storage_mapper("lotteryScAddress")]
    fn lottery_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}