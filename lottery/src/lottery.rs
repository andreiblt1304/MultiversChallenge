#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const ONE_EGLD: u64 = 1000000000000000000;
const MAX_NR: u64 = 1500;

#[multiversx_sc::contract]
pub trait Lottery
{
    #[init]
    fn init(&self) {}

    //#[only_owner]
    #[payable("EGLD")]
    #[endpoint(drawWinner)]
    fn draw_winner(&self) {
        let payment = self.call_value().egld_value().clone_value();
        let caller = self.blockchain().get_caller();
        let _balance = self.blockchain().get_balance(&self.blockchain().get_sc_address());
        let _address = self.blockchain().get_sc_address();
        require!(
            !self.is_executing().get(),
            "Contract is in execution!"
        );
        self.is_executing().set(true);
        require!(payment == ONE_EGLD, "Invalid payment");
        require!(
            !self.participants().contains_key(&caller),
            "A participant can only send funds once"
        );
        self.participants().insert(caller.clone(), payment);

        let mut rand_source = RandomnessSource::new();
        let participants_count: u64 = self.participants().len() as u64;
        let rand_nr = rand_source.next_u64_in_range(1, MAX_NR + participants_count);
        if rand_nr < 1000 {
            let prize: BigUint = BigUint::from(100u32) * ONE_EGLD;
            
            self.participants().clear();
            self.is_executing().set(false);
            self.send().direct_egld(&caller, &prize);
        }
    }

    #[view(participants)]
    #[storage_mapper("participants")]
    fn participants(&self) -> MapMapper<ManagedAddress, BigUint>;

    #[storage_mapper("isExecuting")]
    fn is_executing(&self) -> SingleValueMapper<bool>;
}
