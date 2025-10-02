#![no_std]

pub static ERROR_PAUSED: &[u8] = b"Paused";
pub static ERROR_NOT_PAUSED: &[u8] = b"Not paused";

pub static ERROR_NOT_ADMIN: &[u8] = b"Only admin allowed";

pub static ERROR_WRONG_TIMEFRAME: &[u8] = b"Wrong timeframe";
pub static ERROR_WRONG_START_TS: &[u8] = b"Wrong start timestamp";
pub static ERROR_WRONG_REWARD_VALUES: &[u8] = b"Wrong reward values";
pub static ERROR_FARM_ALREADY_CREATED: &[u8] = b"Farm already created";

pub static ERROR_ALREADY_STARTED: &[u8] = b"Already started";
pub static ERROR_ALREADY_ENDED: &[u8] = b"Already ended";

pub static ERROR_INVALID_REWARD_TOKEN: &[u8] = b"Invalid reward token";
pub static ERROR_ZERO_AMOUNT: &[u8] = b"Zero amount";
pub static ERROR_INSUFFICIENT_RESERVE: &[u8] = b"Insufficient reserve";

pub static ERROR_WRONG_ISSUANCE_AMOUNT: &[u8] = b"Wrong token issuance amount";

pub static ERROR_NO_REWARDS_APPLICABLE: &[u8] = b"No rewards applicable";
pub static ERROR_NOT_ENOUGH_REWARDS: &[u8] = b"Not enough rewards";

pub static ERROR_INVALID_SHARE_TOKEN: &[u8] = b"Invalid share token";

pub static ERROR_ONLY_ONE_STAKING_TRANSFER_ALLOWED: &[u8] = b"Only one staking transfer allowed";
pub static ERROR_STAKED_TOKEN_MISSING: &str = "Staked token transfer missing";

pub static ERROR_UNSTAKE_AMOUNT_EXCEEDS: &[u8] = b"Unstake amount exceeds stake";