#![no_std]

use multiversx_sc::imports::*;

mod admins;
mod farm;
mod pause;
mod rewards;
mod staking;

/// A Smart Contract that allows users to stake tokens and earn rewards over time.
/// - Users can stake a specific token and receive share tokens in return that represent their stake.
/// - Users can claim rewards based on the time they have staked their tokens and the fixed reward rate.
/// - Users can unstake their tokens by burning their share tokens.
/// - During staking or unstaking, any pending rewards are automatically claimed and sent to the user.
/// - Also, if multiple share tokens are sent, they are merged into a single share token to reduce NFT clutter.
/// - Farm Owner is responsible to fund the rewards reserve and set the farm parameters.
#[multiversx_sc::contract]
pub trait Template:
    admins::AdminsModule
    + pause::PauseModule
    + farm::FarmModule
    + staking::StakingModule
    + rewards::RewardsModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
