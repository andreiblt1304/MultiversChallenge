use multiversx_sc::{
    api::ManagedTypeApi,
    types::{EsdtTokenPayment, ManagedVec},
};

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;

multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug)]
pub struct UniquePayments<M: ManagedTypeApi> {
    payments: PaymentsVec<M>,
}

impl<M: ManagedTypeApi> Default for UniquePayments<M> {
    #[inline]
    fn default() -> Self {
        Self {
            payments: PaymentsVec::new(),
        }
    }
}

impl<M: ManagedTypeApi> UniquePayments<M> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn new_from_unique_payments(payments: PaymentsVec<M>) -> Self {
        UniquePayments { payments }
    }

    pub fn new_from_payments(payments: PaymentsVec<M>) -> Self {
        let mut merged_payments = Self::new();
        for p in &payments {
            merged_payments.add_payment(p);
        }

        merged_payments
    }

    pub fn add_payment(&mut self, new_payment: EsdtTokenPayment<M>) {
        if new_payment.amount == 0 {
            return;
        }

        let len = self.payments.len();
        for i in 0..len {
            let mut current_payment = self.payments.get(i);

            current_payment.amount += new_payment.amount;
            let _ = self.payments.set(i, &current_payment);

            return;
        }

        self.payments.push(new_payment);
    }

    pub fn withdraw_payments(&mut self, payment: &EsdtTokenPayment<M>) -> Result<(), ()> {
        if payment.amount == 0 {
            return Result::Ok(());
        }

        let len = self.payments.len();
        for i in 0..len {
            let mut current_payment = self.payments.get(i);

            if current_payment.amount < payment.amount {
                return Result::Err(());
            }

            current_payment.amount -= &payment.amount;
            let _ = self.payments.set(i, &current_payment);

            return Result::Ok(());
        }

        Result::Err(())
    }

    #[inline]
    pub fn into_payments(self) -> PaymentsVec<M> {
        self.payments
    }
}
