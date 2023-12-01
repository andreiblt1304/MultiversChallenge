multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const NULL_ID: AddressId = 0;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct ServiceInfo<M: ManagedTypeApi> {
    pub sc_address: ManagedAddress<M>,
    pub opt_payment_token: Option<EgldOrEsdtTokenIdentifier<M>>,
    pub amount: BigUint<M>,
}

#[derive(TypeAbi, TopDecode, TopEncode)]
pub enum Periodicity {
    None,
    Daily,
    Weekly,
    Monthly
}

#[multiversx_sc::module]
pub trait ServiceModule: crate::payments::payments::PaymentsModule {
    #[endpoint(registerService)]
    fn register_service(
        &self,
        args: MultiValueEncoded<
            MultiValue3<ManagedAddress, Option<EgldOrEsdtTokenIdentifier>, BigUint>
        >
    ) {
        require!(!args.is_empty(), "No arguments provided");

        let service_address = self.blockchain().get_caller();
        let existing_service_id = self.service_id().get_id(&service_address);
        require!(existing_service_id == NULL_ID, "Service already registered");

        let mut services = ManagedVec::<Self::Api, ServiceInfo<Self::Api>>::new();

        for arg in args {
            let (
                sc_address, 
                opt_payment_token, 
                amount
            ) = arg.into_tuple();

            require!(
                self.blockchain().is_smart_contract(&sc_address) && 
                !sc_address.is_zero(),
                "Invalid SC address"
            );

            if let Some(token_id) = &opt_payment_token {
                require!(
                    self.accepted_payment_tokens().contains(token_id),
                    "Invalid token ID"
                );
            }

            services.push(
                ServiceInfo {
                    sc_address,
                    opt_payment_token,
                    amount,
                }
            );
        }
    }

    #[storage_mapper("serviceId")]
    fn service_id(&self) -> AddressToIdMapper<Self::Api>;
}


