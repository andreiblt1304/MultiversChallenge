#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const ONE_EGLD: u64 = 1000000000000000000;

#[multiversx_sc::contract]
pub trait Lottery {
    #[init]
    fn init(&self) {}

    // #[only_owner]
    #[payable("EGLD")]
    #[endpoint(drawWinner)]
    fn draw_winner(&self) {
        require!(!self.is_executing().get(), "Contract is in execution!");
        self.is_executing().set(true);

        let mut rand_source = RandomnessSource::new();
        let participants_count: u64 = self.participants().len() as u64;
        if participants_count == 1 {
            self.is_executing().set(false);
            require!(
                participants_count > 1,
                "There should be more than 1 participants"
            );
        }
        let rand_index = rand_source.next_u64_in_range(0, participants_count);

        for (index, participant) in self.participants().iter().enumerate() {
            if index == rand_index.try_into().unwrap() {
                self.winner().set(participant.0);
                self.is_executing().set(false);
                break;
            }
        }
    }

    #[payable("EGLD")]
    #[endpoint(participate)]
    fn participate(&self, participant: ManagedAddress) -> bool {
        let call_value = self.call_value().egld_value().clone_value();
        require!(
            !self.is_executing().get(),
            "The lottery is in progress, you can't participate now"
        );

        require!(
            !(call_value < ONE_EGLD),
            "Minimum participation value is one EGLD"
        );

        require!(
            !self.participants().contains_key(&participant),
            "You are allowed to participate only once"
        );

        self.participants().insert(participant, call_value);

        return true;
    }

    #[payable("EGLD")]
    #[endpoint]
    fn redeem_prize(&self, participant: ManagedAddress) {
        require!(
            self.participants().is_empty() || !self.is_executing().get(),
            "The lottery is still in progress"
        );
        require!(self.get_winner() != None, "There was no winner nomitated");

        require!(
            self.is_winner(&participant),
            "You are not the winner of the lottery"
        );

        require!(
            self.participants().len() > 1,
            "There is only one participant!"
        );

        let prize = self.calculate_prize();

        self.participants().clear();
        self.is_executing().set(false);
        self.send().direct_egld(&participant, &prize);
    }

    fn calculate_prize(&self) -> BigUint {
        let mut prize: BigUint = BigUint::from(0u64);

        for participant in &self.participants() {
            prize += participant.1
        }

        return BigUint::from(prize);
    }

    fn is_winner(&self, caller: &ManagedAddress) -> bool {
        match self.get_winner() {
            Some(ref winner_address) => winner_address == caller,
            None => false,
        }
    }

    fn get_winner(&self) -> Option<ManagedAddress> {
        if self.winner().is_empty() {
            None
        } else {
            Some(self.winner().get())
        }
    }

    #[storage_mapper("winner")]
    fn winner(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(participants)]
    #[storage_mapper("participants")]
    fn participants(&self) -> MapMapper<ManagedAddress, BigUint>;

    #[storage_mapper("isExecuting")]
    fn is_executing(&self) -> SingleValueMapper<bool>;
}
