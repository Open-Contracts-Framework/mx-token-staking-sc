#![no_std]

use multiversx_sc::{derive_imports::*, imports::*};

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Eq, Debug, ManagedVecItem)]
pub struct Farm<M: ManagedTypeApi> {
    pub staked_token: EgldOrEsdtTokenIdentifier<M>,
    pub share_token: TokenIdentifier<M>,
    pub reward_token: EgldOrEsdtTokenIdentifier<M>,
    pub rewards_reserve: BigUint<M>,
    pub start_ts_ms: u64,
    pub end_ts_ms: u64,
    pub reward_per_sec: BigUint<M>,
    pub reward_per_share: BigUint<M>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Eq, Debug, ManagedVecItem)]
pub struct ShareToken<M: ManagedTypeApi> {
    pub nonce: u64,
    pub amount: BigUint<M>,
}

pub type Nonce = u64;
pub type Amount<M> = BigUint<M>;
pub type ShareTokenType<M> = MultiValue2<Nonce, Amount<M>>;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Eq, Debug, ManagedVecItem)]
pub struct ShareTokenAttributes {
    pub update_ts_ms: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Eq, Debug, ManagedVecItem)]
pub struct ShareTokenMergedData<M: ManagedTypeApi> {
    pub update_ts_ms: u64,
    pub token_supply: BigUint<M>,
    pub reward_amount: BigUint<M>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Eq, Debug, ManagedVecItem)]
pub struct ShareTokenMergedDataWithBurns<M: ManagedTypeApi> {
    pub update_ts_ms: u64,
    pub token_supply: BigUint<M>,
    pub reward_amount: BigUint<M>,
    pub token_burns: ManagedVec<M, ShareToken<M>>,
}