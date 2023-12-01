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
pub enum SubscriptionType {
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

        // may contain multiple services
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

    #[endpoint]
    fn subscribe(
        &self,
        services: MultiValueEncoded<MultiValue3<AddressId, usize, SubscriptionType>>
    ) {
        let caller = self.blockchain().get_caller();
        let caller_id = self.user_id().get_id_non_zero(&caller);
        
        for pair in services {
            let (
                service_id,
                service_index,
                subscription_type
            ) = pair.into_tuple();
            let service_options = self.service_info(service_id).get();

            require!(
                service_index < service_options.len(),
                "Invalid service index"
            );

            require!(
                !matches!(subscription_type, SubscriptionType::None),
                "Invalid subscription type"
            );

            self.subscription_type(caller_id, service_id, service_index)
                .set(subscription_type);

            let _ = self
                .subscribed_users(service_id, service_index)
                .swap_remove(&caller_id);
        }
    }

    #[storage_mapper("serviceId")]
    fn service_id(&self) -> AddressToIdMapper<Self::Api>;

    #[view(getServiceInfo)]
    #[storage_mapper("serviceInfo")]
    fn service_info(
        &self,
        service_id: AddressId,
    ) -> SingleValueMapper<ManagedVec<ServiceInfo<Self::Api>>>;

    #[view(getPendingServices)]
    #[storage_mapper("pendingServices")]
    fn pending_services(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getSubscribedUsers)]
    #[storage_mapper("subscribedUsers")]
    fn subscribed_users(
        &self,
        service_id: AddressId,
        service_index: usize,
    ) -> UnorderedSetMapper<AddressId>;

    #[storage_mapper("subscriptionType")]
    fn subscription_type(
        &self,
        user_id: AddressId,
        service_id: AddressId,
        service_index: usize,
    ) -> SingleValueMapper<SubscriptionType>;
}


