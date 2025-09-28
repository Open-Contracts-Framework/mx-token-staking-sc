#![no_std]

// use multiversx_sc::{derive_imports::*, imports::*};

// pub type Admin<M> = ManagedAddress<M>;

// #[type_abi]
// #[derive(TopEncode, TopDecode, PartialEq, Copy, Clone, Debug)]
// pub enum State {
//     Inactive,
//     Active,
// }

// #[type_abi]
// #[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, ManagedVecItem)]
// pub struct TokenAmount<M: ManagedTypeApi> {
//     pub token: TokenIdentifier<M>,
//     pub amount: BigUint<M>,
// }
// impl<M: ManagedTypeApi> TokenAmount<M> {
//     pub fn new(token: TokenIdentifier<M>, amount: BigUint<M>) -> Self {
//         TokenAmount { token, amount }
//     }
// }