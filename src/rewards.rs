use errors::{ERROR_INVALID_SHARE_TOKEN, ERROR_NOT_ENOUGH_REWARDS, ERROR_NO_REWARDS_APPLICABLE};
use multiversx_sc::imports::*;
use structs::{
    ShareToken, ShareTokenAttributes, ShareTokenMergedData, ShareTokenMergedDataWithBurns,
    ShareTokenType,
};

#[multiversx_sc::module]
pub trait RewardsModule:
    crate::admins::AdminsModule + crate::pause::PauseModule + crate::farm::FarmModule
{
    // === Endpoints ===

    #[payable]
    #[endpoint(claimRewards)]
    fn claim_rewards_endpoint(&self) {
        self.require_not_paused();

        let current_timestamp_ms = self.blockchain().get_block_timestamp_ms();
        let caller = self.blockchain().get_caller();
        let transfers = self.call_value().all_esdt_transfers();

        let token_merged_data = self.claim_rewards(&caller, current_timestamp_ms, &transfers);

        self.share_token().nft_create_and_send(
            &caller,
            token_merged_data.token_supply.clone(),
            &self.attributes_to_buffer(current_timestamp_ms, token_merged_data.token_supply),
        );
    }

    // === Views ===

    #[view(getClaimableRewards)]
    fn get_claimable_rewards(
        &self,
        share_tokens: MultiValueEncoded<ShareTokenType<Self::Api>>,
    ) -> BigUint {
        let current_timestamp_ms = self.blockchain().get_block_timestamp_ms();

        let mut rewards = BigUint::zero();
        for share_token in share_tokens.into_iter() {
            let (nonce, amount) = share_token.into_tuple();
            rewards += self
                .calculate_reward(nonce, &amount, current_timestamp_ms)
                .reward_amount;
        }

        rewards
    }

    // === Private ===

    fn claim_rewards(
        &self,
        caller: &ManagedAddress,
        current_timestamp_ms: u64,
        transfers: &ManagedVec<EsdtTokenPayment>,
    ) -> ShareTokenMergedDataWithBurns<Self::Api> {
        let token_merged_data = self.prepare_rewards_and_burns(transfers, current_timestamp_ms);
        require!(
            token_merged_data.reward_amount > BigUint::zero(),
            ERROR_NO_REWARDS_APPLICABLE
        );
        require!(
            self.rewards_reserve().get() >= token_merged_data.reward_amount,
            ERROR_NOT_ENOUGH_REWARDS
        );

        self.rewards_reserve()
            .update(|current| *current -= &token_merged_data.reward_amount);

        self.burn_share_tokens(&token_merged_data.token_burns);

        self.send().direct(
            caller,
            &self.reward_token().get(),
            0,
            &token_merged_data.reward_amount,
        );

        self.event_rewards_claimed(
            &self.blockchain().get_caller(),
            &token_merged_data.reward_amount,
        );

        token_merged_data
    }

    #[view(test)]
    fn get_share_token_attributes(&self, nonce: u64) -> ShareTokenAttributes<Self::Api> {
        self.share_token().get_token_attributes(nonce)
    }

    fn attributes_to_buffer(&self, update_ts_ms: u64, token_supply: BigUint) -> ManagedBuffer {
        let share_token_attributes = ShareTokenAttributes {
            update_ts_ms,
            token_supply,
        };

        let mut attributes = ManagedBuffer::new();
        let _ = share_token_attributes.top_encode(&mut attributes);
        attributes
    }

    fn prepare_rewards_and_burns(
        &self,
        share_token_transfers: &ManagedVec<EsdtTokenPayment>,
        timestamp_ms: u64,
    ) -> ShareTokenMergedDataWithBurns<Self::Api> {
        let mut total_rewards_amount: BigUint = BigUint::zero();
        let mut total_token_supply: BigUint = BigUint::zero();
        let mut share_tokens: ManagedVec<ShareToken<Self::Api>> = ManagedVec::new();

        let share_token_id = self.share_token().get_token_id();
        for share_token_transfer in share_token_transfers.iter() {
            let (token, nonce, amount) = share_token_transfer.clone().into_tuple();

            require!(token == share_token_id, ERROR_INVALID_SHARE_TOKEN);

            let share_token_merged_data = self.calculate_reward(nonce, &amount, timestamp_ms);

            total_rewards_amount += share_token_merged_data.reward_amount;
            total_token_supply += share_token_merged_data.token_supply;
            share_tokens.push(ShareToken { nonce, amount });
        }

        ShareTokenMergedDataWithBurns {
            update_ts_ms: timestamp_ms,
            token_supply: total_token_supply,
            reward_amount: total_rewards_amount,
            token_burns: share_tokens,
        }
    }

    fn calculate_reward(
        &self,
        token_nonce: u64,
        amount: &BigUint,
        timestamp_ms: u64,
    ) -> ShareTokenMergedData<Self::Api> {
        let share_token_attributes = self.get_share_token_attributes(token_nonce);

        let rewards_per_sec = self.reward_per_sec().get();
        let rewards_per_share = self.reward_per_share().get();

        let rewarded_seconds = if timestamp_ms > share_token_attributes.update_ts_ms {
            (timestamp_ms - share_token_attributes.update_ts_ms) / 1000
        } else {
            0
        };

        let reward_amount = &rewards_per_sec * rewarded_seconds * amount / &rewards_per_share;

        ShareTokenMergedData {
            update_ts_ms: timestamp_ms,
            token_supply: amount.clone(),
            reward_amount,
        }
    }

    fn burn_share_tokens(&self, burns: &ManagedVec<ShareToken<Self::Api>>) {
        for burn in burns.iter() {
            self.burn_share_token(&burn);
        }
    }

    fn burn_share_token(&self, token_burn: &ShareToken<Self::Api>) {
        self.share_token()
            .nft_burn(token_burn.nonce, &token_burn.amount);
    }

    // === Events ===

    #[event("rewardsClaimed")]
    fn event_rewards_claimed(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] amount: &BigUint,
    );
}
