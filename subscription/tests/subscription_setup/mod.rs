#![allow(deprecated)]

use subscription::{
    payments::payments::PaymentsModule,
    service::{ ServiceModule, SubscriptionType} 
};

pub struct SubscriptionSetup<SubscriptionObjBuilder>
where
    SubscriptionObjBuilder: 'static + Copy + Fn() -> subscription::ContractObj<DebugApi>,
{
    pub b_mock: Rc<RefCell<BlockchainStateWrapper>>,
    pub owner_address: Address,
    pub s_wrapper:
        ContractObjWrapper<subscription::ContractObj<DebugApi>, SubscriptionObjBuilder>,
}