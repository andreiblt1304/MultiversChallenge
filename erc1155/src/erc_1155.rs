#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait Erc1155 {
    #[init]
    fn init(&self) {}

    #[endpoint(createToken)]
    fn create_token(
        &self,
        initial_supply: BigUint,
        is_fungible: bool,
    ) -> BigUint {
        let big_uint_one = BigUint::from(1u32);

        let creator = self.blockchain().get_caller();
        let type_id = &self.last_valid_type_id().get() + &big_uint_one;

        self.set_balance(&creator, &type_id, &initial_supply);
        self.token_type_creator(&type_id).set(&creator);
        self.is_fungible(&type_id).set(is_fungible);

        if !is_fungible {
            self.set_owner_for_tokens(&type_id, &big_uint_one, &initial_supply, &creator);
            self.last_valid_nft_type_id(&type_id)
                .set(&initial_supply);
        }

        self.last_valid_type_id().set(&type_id);

        type_id
    }

    //transfer ESDT type token
    #[endpoint]
    fn safe_transfer_from(
        &self, 
        from: ManagedAddress,
        to: ManagedAddress,
        type_id: BigUint,
        value: BigUint,
        data: &ManagedBuffer
    ) {
        let caller = self.blockchain().get_caller();

        require!(!to.is_zero(), "Tokens can't be trasfered to the zero address");
        require!(self.is_valid_type_id(&type_id), "Token id is invalid");

        if self.is_fungible(&type_id).get() {
            self.safe_transfer_from_fungible(from, to, type_id, value, data);
        } else {
            //self.safe_transfer_from_non_fungible(&from, &to, type_id, value, data);
        }
    }

    #[endpoint]
    fn safe_transfer_from_fungible(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_id: BigUint,
        amount: BigUint,
        data: &ManagedBuffer
    ) {
        self.try_balance_fungible(&from, &type_id, &amount);

        if self.blockchain().is_smart_contract(&to) {
            self.execute_async_call_single_transfer(from, to, type_id, amount, data);
        } else {
            self.increase_balance(&to, &type_id, &amount);
        }
    }

    // #[endpoint]
    // fn safe_transfer_from_non_fungible(
    //     &self,
    //     from: &ManagedAddress,
    //     to: &ManagedAddress,
    //     type_id: BigUint,
    //     value: BigUint,
    //     data: &ManagedBuffer
    // ) {

    // }

    #[endpoint]
    fn mint(&self, type_id: BigUint, amount: BigUint) {
        let creator = self.token_type_creator(&type_id).get();

        require!(
            self.blockchain().get_caller() == creator,
            "Only the token creator may mint more tokens"
        );

        self.increase_balance(&creator, &type_id, &amount);

        if !self.is_fungible(&type_id).get() {
            // assign NFT to user
        }
    }

    #[endpoint]
    fn burn(&self, type_id: BigUint, amount: BigUint) {
        require!(
            self.is_fungible(&type_id).get(),
            "Only fungible tokens can be burned"
        );

        let caller = self.blockchain().get_caller();
        let balance = self.get_balance_mapper(&caller)
            .get(&type_id)
            .unwrap_or_default();

        require!(balance >= amount, "Not enough tokens to burn");
        self.decrease_balance(&caller, &type_id, &amount);
    }

    fn execute_async_call_single_transfer(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_id: BigUint,
        amount: BigUint,
        data: &ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();

    }

    fn set_owner_for_tokens(
        &self,
        type_id: &BigUint,
        start: &BigUint,
        end: &BigUint,
        owner: &ManagedAddress
    ) {
        let big_uint_one = BigUint::from(1u32);
        let mut nft_id = start.clone();

        while &nft_id <= end {
            self.token_owner(type_id, &nft_id).set(owner);
            nft_id += &big_uint_one;
        }
    }

    fn increase_balance(&self, owner: &ManagedAddress, type_id: &BigUint, amount: &BigUint) {
        let mut balance = self.get_balance_mapper(owner)
            .get(type_id)
            .unwrap_or_default();
        balance += amount;
        self.set_balance(owner, type_id, &balance);
    }

    fn decrease_balance(&self, owner: &ManagedAddress, type_id: &BigUint, amount: &BigUint) {
        let mut balance = self.get_balance_mapper(owner)
                            .get(type_id)
                            .unwrap_or_default();

        balance -= amount;
        self.set_balance(owner, type_id, &balance);
    }

    fn set_balance(&self, owner: &ManagedAddress, type_id: &BigUint, value: &BigUint) {
        let mut balance_mapper = self.get_balance_mapper(owner);
        balance_mapper.insert(type_id.clone(), value.clone());
    }

    fn try_balance_fungible(
        &self,
        owner: &ManagedAddress, 
        type_id: &BigUint,
        amount: &BigUint 
    ) {
        let balance = 
            self.get_balance_mapper(owner)
                .get(type_id)
                .unwrap_or_default();

        require!(amount > &0u32, "Must transfer more than 0");
        require!(amount <= &balance, "Not enough balance for it");
    }

    fn is_valid_type_id(&self, type_id: &BigUint) -> bool {
        type_id > &0 && type_id < &self.last_valid_type_id().get()
    }

    #[view(getTokenOwner)]
    #[storage_mapper("tokenOwner")]
    fn token_owner(&self, type_id: &BigUint, nft_id: &BigUint) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("balanceOf")]
    fn get_balance_mapper(&self, owner: &ManagedAddress) -> MapMapper<BigUint, BigUint>;

    #[view(isFungible)]
    #[storage_mapper("isFungible")]
    fn is_fungible(&self, type_id: &BigUint) -> SingleValueMapper<bool>;

    #[view(tokenTypeCreator)]
    #[storage_mapper("tokenTypeCreator")]
    fn token_type_creator(&self, type_id: &BigUint) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("lastValidTypeId")]
    fn last_valid_type_id(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("lastValidNftTypeId")]
    fn last_valid_nft_type_id(&self, type_id: &BigUint) -> SingleValueMapper<BigUint>;
}