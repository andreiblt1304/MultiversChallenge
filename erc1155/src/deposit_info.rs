multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone, ManagedVecItem)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub esdt_funds: ManagedVec<M, EsdtTokenPayment<M>>
}

impl<M> DepositInfo<M>
where
    M: ManagedTypeApi
{}

//impl<M> Copy for DepositInfo<M> {}