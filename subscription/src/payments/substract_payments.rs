use core::hint::unreachable_unchecked;

use crate::service::SubscriptionType;

use super::unique_payments::UniquePayments;

pub type Epoch = u64;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const DAILY_EPOCH: Epoch = 1;
pub const WEEKLY_EPOCH: Epoch = 7;
pub const MONTHLY_EPOCH: Epoch = 30;

#[must_use]
#[derive(Debug, PartialEq, Eq, Clone, TopDecode, TopEncode, TypeAbi)]
pub enum CustomScResult<
    T: NestedEncode + NestedDecode + TypeAbi,
    E: NestedEncode + NestedDecode + TypeAbi
> {
    Ok(T),
    Err(E)
}

impl<T: NestedEncode + NestedDecode + TypeAbi, E: NestedEncode + NestedDecode + TypeAbi>
    CustomScResult<T, E>
{
    pub fn is_err(&self) -> bool {
        matches!(*self, CustomScResult::Err(_))
    }

    pub unsafe fn unwrap_unchecked(self) -> T {
        match self {
            CustomScResult::Ok(t) => t,
            CustomScResult::Err(_) => unreachable_unchecked(),
        }
    }
}

#[multiversx_sc::module]
pub trait SubstractPaymentsModule:
    crate::payments::payments::PaymentsModule
    + crate::service::ServiceModule
    //+ multiversx_sc_modules::ongoing_operation::OngoingOperationModule
{
    #[endpoint(substractPayment)]
    fn substract_payment(
        &self,
        service_index: usize,
        user_id: AddressId
    ) -> CustomScResult<EgldOrEsdtTokenPayment, ()> {
        let caller = self.blockchain().get_caller();
        let service_id = self.service_id().get_id_non_zero(&caller);
        let current_epoch = self.blockchain().get_block_epoch();

        let last_action_mapper = self.user_last_action_epoch(user_id, service_id, service_index);
        let last_action_epoch = last_action_mapper.get();

        if last_action_epoch > 0 {
            let next_substract_epoch = last_action_epoch + MONTHLY_EPOCH;
            
            require!(next_substract_epoch <= current_epoch, "Cannot substract the payment yet");
        }
        
        let opt_user_adddress = self.user_id().get_address(user_id);

        let opt_user_adddress = self.user_id().get_address(user_id);
        
        if opt_user_adddress.is_none() {
            return CustomScResult::Err(());
        }

        let subscription_type = self.subscription_type(user_id, service_id, service_index).get();

        let multiplier = match subscription_type {
            SubscriptionType::Daily => MONTHLY_EPOCH / DAILY_EPOCH,
            SubscriptionType::Weekly => MONTHLY_EPOCH / WEEKLY_EPOCH,
            SubscriptionType::Monthly => 1,
            SubscriptionType::None => return CustomScResult::Err(())
        };

        let service_info = self.service_info(service_id).get().get(service_index);
        let substract_result = match service_info.opt_payment_token {
            Some(token_id) => {
                self.substract_specific_token(user_id, token_id, service_info.amount * multiplier)
            }
            None => self.substract_any_token(user_id, service_info.amount * multiplier)
        };

        if let CustomScResult::Ok(payment) = &substract_result {
            self.send().direct(
                &caller,
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount
            );
        }

        last_action_mapper.set(current_epoch);

        substract_result
    }

    fn substract_specific_token(
        &self,
        user_id: AddressId,
        token_id: EgldOrEsdtTokenIdentifier,
        amount: BigUint
    ) -> CustomScResult<EgldOrEsdtTokenPayment, ()> {
        if token_id.is_egld() {
            return self.user_deposited_egld(user_id).update(|value| {
                if *value < amount {
                    return CustomScResult::Err(());
                }

                *value -= &amount;

                CustomScResult::Ok(EgldOrEsdtTokenPayment::new(
                    EgldOrEsdtTokenIdentifier::egld(),
                    0,
                    amount
                ))
            });
        }

        let payment = EsdtTokenPayment::new(token_id.unwrap_esdt(), 0, amount);
        let raw_result = self
            .user_deposited_payments(user_id)
            .update(|user_payments| user_payments.withdraw_payments(&payment));

        match raw_result {
            Result::Ok(()) => CustomScResult::Ok(payment.into()),
            Result::Err(()) => CustomScResult::Err(()),
        }
    }

    fn substract_any_token(
        &self,
        user_id: AddressId,
        amount: BigUint
    ) -> CustomScResult<EgldOrEsdtTokenPayment, ()> {
        let tokens_mapper = self.user_deposited_payments(user_id);
        
        if tokens_mapper.is_empty() {
            return CustomScResult::Err(());
        }

        let mut user_tokens = tokens_mapper.get().into_payments();

        for (index, payment) in user_tokens.iter().enumerate() {
            //let mut payment = user_tokens.get(index);
            let query_result = self.get_price(payment.token_identifier.clone());

            if query_result.is_err() {
                continue;
            }

            let price = unsafe { query_result.unwrap_unchecked() };

            if price > payment.amount {
                continue;
            }

            payment.amount -= &price;

            let _ = user_tokens.set(index, &payment);
            tokens_mapper.set(UniquePayments::new_from_unique_payments(user_tokens));

            return CustomScResult::Ok(
                EgldOrEsdtTokenPayment::new(
                    EgldOrEsdtTokenIdentifier::esdt(payment.token_identifier),
                    0,
                    price
                )
            );
        }

        CustomScResult::Err(())
    }

    #[storage_mapper("userLastActionEpoch")]
    fn user_last_action_epoch(
        &self,
        user_id: AddressId,
        service_id: AddressId,
        service_index: usize,
    ) -> SingleValueMapper<Epoch>;
}
