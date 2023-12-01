use super::unique_payments::UniquePayments;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait PaymentsModule {

    // admin action: whitelist tokens
    #[only_owner]
    #[endpoint(addAcceptedPaymentTokens)]
    fn add_accepted_payment_tokens(
        &self,
        accepted_tokens: MultiValueEncoded<EgldOrEsdtTokenIdentifier>,
    ) {
        for token in accepted_tokens {
            require!(token.is_valid(), "Invalid token");

            let _ = self.accepted_payment_tokens().insert(token);
        }
    }

    // user deposit
    #[payable("*")]
    #[endpoint(deposit)]
    fn deposit(&self) {
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        require!(payment_amount > 0, "No payment amount provided");
        require!(
            self.accepted_payment_tokens().contains(&payment_token),
            "Invalid payment token provided"
        );

        let caller = self.blockchain().get_caller();
        let caller_id = self.user_id().get_id_or_insert(&caller);
        self.add_user_payment(
            caller_id,
            EgldOrEsdtTokenPayment::new(payment_token, 0, payment_amount),
            self.user_deposited_payments(caller_id)
        );
    }

    //user withdraw
    #[endpoint(withdraw)]
    fn withdraw(
        &self,
        tokens_to_withdraw: MultiValueEncoded<MultiValue2<EgldOrEsdtTokenIdentifier, BigUint>>
    ) ->MultiValue2<BigUint, ManagedVec<EsdtTokenPayment>> {
        let caller = self.blockchain().get_caller();
        let caller_id = self.user_id().get_id_non_zero(&caller);

        let user_payments = self.user_deposited_payments(caller_id);
        let mut all_user_tokens = user_payments.get().into_payments();
        let mut egld_amount = BigUint::zero();
        let mut output_payments = ManagedVec::new();

        for pair in tokens_to_withdraw {
            let (token_id, amount) = pair.into_tuple();

            if token_id.is_egld() {
                let egld_mapper = self.user_deposited_egld(caller_id);
                let user_egld_ammount = egld_mapper.get();
                
                if user_egld_ammount >= amount {
                    self.send().direct_egld(&caller, &amount);
                    egld_mapper.set(&user_egld_ammount - &amount);

                    egld_amount += amount;
                }

                continue;
            }

            let mut opt_found_index = None;
            for (index, user_payment) in all_user_tokens.iter().enumerate() {
                if user_payment.token_identifier == token_id && user_payment.amount >= amount {
                    output_payments.push(EsdtTokenPayment::new(
                        token_id.unwrap_esdt(),
                        0,
                        amount.clone()
                    ));
    
                    opt_found_index = Some(index);
                    break;
                }
            }

            if opt_found_index.is_none() {
                continue;
            }

            let token_index = unsafe { opt_found_index.unwrap_unchecked() };
            let token_info = all_user_tokens.get(token_index);

            if token_info.amount == amount {
                let _ = all_user_tokens.set(token_index, &token_info);
            }
        }

        if !output_payments.is_empty() {
            self.send().direct_multi(&caller, &output_payments);
        }

        user_payments.set(&UniquePayments::new_from_unique_payments(all_user_tokens));

        (egld_amount, output_payments).into()
    }

    fn add_user_payment(
        &self,
        caller_id: AddressId,
        payment: EgldOrEsdtTokenPayment,
        mapper: SingleValueMapper<UniquePayments<Self::Api>>
    ) {
        if payment.token_identifier.is_egld() {
            self.user_deposited_egld(caller_id)
                .update(|deposited_egld| *deposited_egld += payment.amount);

            return;
        }

        if mapper.is_empty() {
            let user_payments = UniquePayments::<Self::Api>::new_from_unique_payments(
                ManagedVec::from_single_item(payment.unwrap_esdt())
            );

            mapper.set(&user_payments);
        } else {
            mapper.update(|fees| {
                fees.add_payment(payment.unwrap_esdt());
            })
        }
    }

    #[view(getAcceptedFeesTokens)]
    #[storage_mapper("acceptedFeesTokens")]
    fn accepted_payment_tokens(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;

    #[storage_mapper("userId")]
    fn user_id(&self) -> AddressToIdMapper<Self::Api>;

    #[view(getUserDepositedPayments)]
    #[storage_mapper("userDepositedFees")]
    fn user_deposited_payments(
        &self,
        user: AddressId
    ) -> SingleValueMapper<UniquePayments<Self::Api>>;

    #[view(getUserDepositedEgld)]
    #[storage_mapper("userDepositedEgld")]
    fn user_deposited_egld(&self, user_id: AddressId) -> SingleValueMapper<BigUint>;
}