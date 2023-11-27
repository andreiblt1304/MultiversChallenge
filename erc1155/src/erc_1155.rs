//TODO

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
            self.set_owner_for_token_range(&type_id, &big_uint_one, &initial_supply, &creator);
            self.last_valid_nft_type_id(&type_id)
                .set(&initial_supply);
        }

        self.last_valid_type_id().set(&type_id);

        type_id
    }

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
        require!(
            caller == from || self.has_permission(&caller, &from).get(),
            "Caller has no permissions to transfer tokens from address"
        );

        if self.is_fungible(&type_id).get() {
            self.safe_transfer_from_fungible(from, to, type_id, value, data);
        } else {
            self.safe_transfer_from_non_fungible(from, to, type_id, value, data);
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
            // TODO maybe add some async call and callback
            self.execute_call_fungible_single_transfer(from, to, type_id, amount, data);
        } else {
            self.modify_balance_after_transfer(&from, &to, &type_id, &amount);
        }
    }

    #[endpoint]
    fn safe_transfer_from_non_fungible(
        &self, 
        from: ManagedAddress,
        to: ManagedAddress,
        type_id: BigUint,
        nft_id: BigUint,
        data: &ManagedBuffer
    ) {
        self.try_balance_nonfungible(&from, &type_id, &nft_id);

        //self.execute_call_nonfungible_single_transfer(&to, &type_id, &nft_id);
        self.increase_balance(&to, &type_id, &BigUint::from(1u32));
        self.token_owner(&type_id, &nft_id).set(&to);

    }

    #[endpoint]
    fn batch_transfer_from(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_ids: &ManagedVec<BigUint>,
        values: &ManagedVec<BigUint>,
        data: ManagedBuffer
    ) {
        let caller = self.blockchain().get_caller();

        require!(
            caller == from || self.has_permission(&caller, &from).get(),
            "Calles is not approved to transfer tokens from address"
        );

        require!(!to.is_zero(), "Tokens can't be sent to the zero address");
        require!(
            !type_ids.is_empty() && !values.is_empty(),
            "There were no type_ids and values provided"
        );
        require!(
            type_ids.len() == values.len(),
            "The length of type_ids and values don't match"
        );

        for (type_id, value) in type_ids.iter().zip(values.iter()) {
            if self.is_fungible(&type_id).get() {
                self.batch_transfer_from_fungible(
                    &from,
                    &to,
                    &type_id,
                    &value
                );
            } else {
                self.batch_transfer_from_nonfugible(
                    &from,
                    &to,
                    &type_id,
                    &value
                )
            }
        }
    }

    #[endpoint]
    fn mint(&self, type_id: BigUint, amount: BigUint) {
        let creator = self.token_type_creator(&type_id).get();

        require!(
            self.blockchain().get_caller() == creator,
            "Only the token creator may mint more tokens"
        );

        self.increase_balance(&creator, &type_id, &amount);

        if !self.is_fungible(&type_id).get() {
            let last_valid_id = self.last_valid_nft_type_id(&type_id).get();
            let new_nft_first_id = &last_valid_id + 1u32;
            let new_nft_last_id = last_valid_id + &amount;

            self.set_owner_for_token_range(&type_id, &new_nft_first_id, &new_nft_last_id, &creator);

            self.last_valid_nft_type_id(&type_id).set(&new_nft_last_id);
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

    fn mint_and_send_esdt_tokens(
        &self,
        to: &ManagedAddress,
        amount: BigUint
    ) -> EsdtTokenPayment<Self::Api> {

        self.create_token(amount, false);

        let type_id = self.last_valid_type_id().get();
        self.mint(type_id, &amount);

        EgldOrEsdtTokenPayment::new()
    }

    #[payable("*")]
    #[endpoint]
    fn deposit(&self) -> EgldOrEsdtTokenPayment<Self::Api> {
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();

        self.increase_esdt_balance(self.esdt_tokens_balance(), &payment_amount);

        let caller = self.blockchain().get_caller();

        let payment_result = self.mint_and_send_esdt_tokens(caller, payment_amount);

        payment_result
    }

    #[payable("*")]
    #[endpoint]
    fn withdraw(
        &self,
        type_ids: &ManagedVec<BigUint>,
        values: &ManagedRef<BigUint>
    ) -> EgldOrEsdtTokenPayment<Self::Api> {
        let (payment_token, payment_nonce, payment_amount) = 
            self.call_value().single_esdt().into_tuple();
        let caller = self.blockchain().get_caller();
        
        for (type_id, value) in type_ids.iter().zip(values.iter()) {
            //let token_owner = self.token_owner(type_id, nft_id);
            require!(
                type_id == payment_token,
                "The token type id provided is corresponding to the payment token"
            );

            self.decrease_balance(caller, type_id, value);
            self.send().direct(&caller, &type_id, 0, &value);
        };

        EgldOrEsdtTokenPayment::new()
    }

    #[callback]
    fn transfer_callback(
        &self,
        from: &ManagedAddress,
        to: &ManagedAddress,
        type_ids: &ManagedVec<BigUint>,
        values: &ManagedVec<BigUint>,
        #[call_result] result: ManagedAsyncCallResult<()>
    ) {
        let destination_addr = match result {
            ManagedAsyncCallResult::Ok(()) => to,
            ManagedAsyncCallResult::Err(_) => from,
        };

        for (type_id, value) in type_ids.iter().zip(values.iter()) {
            if self.is_fungible(&type_id).get() {
                self.increase_balance(&destination_addr, &type_id, &value);
            } else {
                self.increase_balance(&destination_addr, &type_id, &BigUint::from(1u32));
            }
        }
    }

    fn execute_call_fungible_single_transfer(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        type_id: BigUint,
        amount: BigUint,
        data: &ManagedBuffer,
    ) {
        self.modify_balance_after_transfer(&from, &to, &type_id, &amount);
    }

    fn execute_call_nonfungible_single_transfer(
        &self,
        to: &ManagedAddress,
        type_id: &BigUint,
        nft_id: &BigUint
    ) {
        self.increase_balance(to, type_id, &BigUint::from(1u32));
        self.token_owner(type_id, nft_id).set(to);
    }

    fn batch_transfer_from_fungible(
        &self,
        from: &ManagedAddress,
        to: &ManagedAddress,
        type_id: &ManagedRef<BigUint>,
        amount: &ManagedRef<BigUint>
    ) {
        self.try_balance_fungible(from, type_id, amount);
        self.increase_balance(to, type_id, amount);
    }

    fn batch_transfer_from_nonfugible(
        &self,
        to: &ManagedAddress,
        from: &ManagedAddress,
        type_id: &ManagedRef<BigUint>,
        nft_id: &ManagedRef<BigUint>
    ) {
        self.try_balance_nonfungible(from, type_id, nft_id);

        self.token_owner(type_id, nft_id).set(to);
    }

    fn modify_balance_after_transfer(
        &self,
        from: &ManagedAddress,
        to: &ManagedAddress,
        type_id: &BigUint,
        amount: &BigUint
    ) {
        self.increase_balance(to, type_id, amount);
        self.decrease_balance(from, type_id, amount);
    }

    fn set_owner_for_token_range(
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
        let mut balance = self.get_balance_for_type_id(owner, type_id);

        require!(
            &balance >= amount,
            "Balance has to be at greater or equal than the amount sent"
        );

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

    fn get_balance_for_type_id(
        &self,
        owner: &ManagedAddress,
        type_id: &BigUint
    ) -> BigUint {
        let balance = self.get_balance_mapper(owner)
            .get(type_id)
            .unwrap_or_else(|| {
                panic!("Error: Unable to retrieve balance for the given type ID")
            });

        balance
    }

    fn try_balance_nonfungible(
        &self,
        owner: &ManagedAddress,
        type_id: &BigUint,
        nft_id: &BigUint
    ) {
        require!(
            self.is_valid_nft_id(type_id, nft_id),
            "The Id for this NFT is not valid"
        );

        require!(
            &self.token_owner(type_id, nft_id).get() == owner,
            "Sender is not the owner of the NFT"
        );

        let amount = BigUint::from(1u32);
        self.decrease_balance(owner, type_id, &amount);
        self.token_owner(type_id, nft_id).set(&ManagedAddress::zero());
    }

    fn is_valid_type_id(&self, type_id: &BigUint) -> bool {
        type_id > &0 && type_id <= &self.last_valid_type_id().get()
    }

    fn is_valid_nft_id(&self, type_id: &BigUint, nft_id: &BigUint) -> bool {
        self.is_valid_type_id(type_id)
            && nft_id > &0
            && nft_id <= &self.last_valid_nft_type_id(type_id).get()
    }

    fn increase_esdt_balance(&self, mapper: SingleValueMapper<BigUint>, amount: &BigUint) {
        mapper.update(|b| *b += amount);
    }

    fn decrease_esdt_balance(&self, mapper: SingleValueMapper<BigUint>, amount: &BigUint) {
        mapper.update(|b| *b -= amount);
    }

    #[view(getDepositAmount)]
    fn get_deposit_amount(&self) -> BigUint {
        let caller = self.blockchain().get_caller();

        self.deposit(&caller).get()
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

    #[view(hasPermission)]
    #[storage_mapper("hasPermission")]
    fn has_permission(&self, operator: &ManagedAddress, owner: &ManagedAddress) -> SingleValueMapper<bool>;

    #[storage_mapper("lastValidTypeId")]
    fn last_valid_type_id(&self) -> SingleValueMapper<BigUint>;
    
    #[storage_mapper("lastValidNftTypeId")]
    fn last_valid_nft_type_id(&self, type_id: &BigUint) -> SingleValueMapper<BigUint>;

    #[storage_mapper("esdtTokensBalance")]
    fn esdt_tokens_balance(&self) -> SingleValueMapper<BigUint>;
}