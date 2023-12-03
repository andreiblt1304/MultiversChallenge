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

    #[payable("EGLD")]
    #[endpoint(drawWinner)]
    fn draw_winner(&self) {
        let payment = self.call_value().egld_value().clone_value();
        require!(payment == ONE_EGLD, "Invalid payment");
        let mut rand_source = RandomnessSource::new();
        let rand_nr = rand_source.next_u64_in_range(1, MAX_NR);
        if rand_nr < 1000 {
            let caller = self.blockchain().get_caller();
            let prize: BigUint = BigUint::from(100u32) * ONE_EGLD;

            self.send().direct_egld(&caller, &BigUint::from(prize));
        }
    }
}
