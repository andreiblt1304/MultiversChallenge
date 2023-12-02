


#[multiversx_sc::module]
pub trait LotteryProxy: proxy_common::ProxyCommonModule {
    #[proxy]
    fn lottery_contract_proxy(&self, to: Address)
}