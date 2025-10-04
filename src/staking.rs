use errors::{
    ERROR_INVALID_SHARE_TOKEN, ERROR_ONLY_ONE_STAKING_TRANSFER_ALLOWED, ERROR_STAKED_TOKEN_MISSING,
    ERROR_UNSTAKE_AMOUNT_EXCEEDS, ERROR_ZERO_AMOUNT,
};
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait StakingModule:
    crate::admins::AdminsModule
    + crate::pause::PauseModule
    + crate::farm::FarmModule
    + crate::rewards::RewardsModule
{
    // === Endpoints ===

    #[payable]
    #[endpoint(stake)]
    fn stake(&self) {
        self.require_not_paused();

        let current_timestamp_ms = self.blockchain().get_block_timestamp_ms();
        let caller = self.blockchain().get_caller();
        let transfers = self.call_value().all_transfers();

        let staked_token = self.staked_token().get();
        let share_token = self.share_token().get_token_id();
        let (staking_transfer, share_transfers) =
            self.split_transfers(&transfers, &staked_token, &share_token);

        let mut new_staked_amount = staking_transfer.amount.clone();
        require!(new_staked_amount > BigUint::zero(), ERROR_ZERO_AMOUNT);

        let mut rewards_claimed = BigUint::zero();

        if !share_transfers.is_empty() {
            let token_merged_data =
                self.claim_rewards(&caller, current_timestamp_ms, &share_transfers);

            new_staked_amount += token_merged_data.token_supply;
            rewards_claimed = token_merged_data.reward_amount;
        }

        self.share_token().nft_create_and_send(
            &caller,
            new_staked_amount.clone(),
            &self.attributes_to_buffer(current_timestamp_ms),
        );

        self.event_staked(
            &caller,
            &staking_transfer.amount,
            &new_staked_amount,
            &rewards_claimed,
        );
    }

    #[payable]
    #[endpoint(unstake)]
    fn unstake(&self, opt_unstake_amount: OptionalValue<BigUint>) {
        self.require_not_paused();

        let current_timestamp_ms = self.blockchain().get_block_timestamp_ms();
        let caller = self.blockchain().get_caller();
        let transfers = self.call_value().all_esdt_transfers();

        let token_merged_data = self.claim_rewards(&caller, current_timestamp_ms, &transfers);

        let unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(amount) => {
                require!(
                    amount <= token_merged_data.token_supply,
                    ERROR_UNSTAKE_AMOUNT_EXCEEDS
                );
                amount
            }
            OptionalValue::None => token_merged_data.token_supply.clone(),
        };

        if token_merged_data.token_supply > unstake_amount {
            self.share_token().nft_create_and_send(
                &caller,
                &token_merged_data.token_supply - &unstake_amount,
                &self.attributes_to_buffer(current_timestamp_ms),
            );
        }

        self.send()
            .direct(&caller, &self.staked_token().get(), 0, &unstake_amount);

        self.event_unstaked(
            &caller,
            &unstake_amount,
            &token_merged_data.token_supply,
            &token_merged_data.reward_amount,
        );
    }

    // === Private ===

    fn split_transfers(
        &self,
        transfers: &ManagedVec<EgldOrEsdtTokenPayment>,
        staked_token: &EgldOrEsdtTokenIdentifier,
        share_token: &TokenIdentifier,
    ) -> (EgldOrEsdtTokenPayment, ManagedVec<EsdtTokenPayment>) {
        let mut staking_transfer: Option<EgldOrEsdtTokenPayment> = None;
        let mut share_transfers: ManagedVec<EsdtTokenPayment> = ManagedVec::new();

        for transfer in transfers.iter() {
            if &transfer.token_identifier == staked_token {
                require!(
                    staking_transfer.is_none(),
                    ERROR_ONLY_ONE_STAKING_TRANSFER_ALLOWED
                );
                staking_transfer = Some(transfer.clone());
            } else if &transfer.token_identifier == share_token {
                share_transfers.push(transfer.clone().unwrap_esdt());
            } else {
                sc_panic!(ERROR_INVALID_SHARE_TOKEN);
            }
        }

        (
            staking_transfer.expect(ERROR_STAKED_TOKEN_MISSING),
            share_transfers,
        )
    }

    // === Events ===

    #[event("staked")]
    fn event_staked(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] staked_amount: &BigUint,
        #[indexed] share_token_supply: &BigUint,
        #[indexed] reward_amount: &BigUint,
    );

    #[event("unstaked")]
    fn event_unstaked(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] unstaked_amount: &BigUint,
        #[indexed] share_token_supply: &BigUint,
        #[indexed] reward_amount: &BigUint,
    );
}
