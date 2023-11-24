// multiversx_sc::derive_imports!();

// use erc1155::*;
// use multiversx_sc::{
//     sc_error,
//     types::{Address, SCResult, BoxedBytes},
// };
// use multiversx_sc_scenario::{
//     managed_address, managed_biguint, managed_token_id, rust_biguint, whitebox::*,
//     DebugApi, testing_framework::BlockchainStateWrapper,
// };

// const WASM_PATH: &'static str = "../output/erc1155.wasm";

// struct Erc1155Setup<Erc1155ObjBuilder>
// where
//     Erc1155ObjBuilder:
//         'static + Copy + Fn() -> erc1155::ContractObj<DebugApi>
    
// {
//     pub blockchain_wrapper: BlockchainStateWrapper,
//     pub type_id: BigUint,
//     pub amount: BigUint,
//     pub initial_supply: BigUint,
//     pub is_fungible: bool,
//     pub uri: BoxedBytes,
//     pub owner_address: Address,
//     pub first_user_address: Address,
//     pub second_user_address: Address,
//     pub erc_wrapper:
//         ContractObj<erc1155::ContractObj<DebugApi>, Erc1155ObjBuilder>,
// }

// fn setup_erc1155<Erc1155ObjBuilder>(
//     erc_builder: Erc1155ObjBuilder
// ) -> Erc1155Setup<Erc1155ObjBuilder>
// where
//     Erc1155ObjBuilder: 'static + Copy + Fn() -> erc1155::ContractObj<DebugApi>
// {
//     let biguint_zero = rust_biguint!(0u64);
//     let mut blockchain_wrapper = BlocbkchainStateWrapper::new();
//     let owner_address = blockchain_wrapper.create_user_account(&biguint_zero);
//     //let first
// }