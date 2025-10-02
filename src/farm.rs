use constants::{TOKEN_ISSUANCE_COST, WAD_DECIMALS};
use errors::{
    ERROR_ALREADY_ENDED, ERROR_ALREADY_STARTED, ERROR_FARM_ALREADY_CREATED,
    ERROR_INSUFFICIENT_RESERVE, ERROR_INVALID_REWARD_TOKEN, ERROR_WRONG_ISSUANCE_AMOUNT,
    ERROR_WRONG_REWARD_VALUES, ERROR_WRONG_START_TS, ERROR_WRONG_TIMEFRAME, ERROR_ZERO_AMOUNT,
};
use multiversx_sc::imports::*;
use structs::Farm;

#[multiversx_sc::module]
pub trait FarmModule: crate::admins::AdminsModule {
    // === Endpoints ===

    #[only_owner]
    #[payable]
    #[endpoint(createFarm)]
    fn create_farm(
        &self,
        staked_token: EgldOrEsdtTokenIdentifier,
        reward_token: EgldOrEsdtTokenIdentifier,
        start_ts_ms: u64,
        end_ts_ms: u64,
        reward_per_sec: BigUint,
        reward_per_share: BigUint,
        share_token_display_name: ManagedBuffer,
        share_token_ticker: ManagedBuffer,
    ) {
        require!(
            self.staked_token().is_empty()
                && self.reward_token().is_empty()
                && self.start_ts_ms().is_empty()
                && self.end_ts_ms().is_empty()
                && self.reward_per_sec().is_empty()
                && self.reward_per_share().is_empty()
                && self.share_token().is_empty(),
            ERROR_FARM_ALREADY_CREATED
        );
        require!(start_ts_ms < end_ts_ms, ERROR_WRONG_TIMEFRAME);
        require!(
            start_ts_ms > self.blockchain().get_block_timestamp_ms(),
            ERROR_WRONG_START_TS
        );
        require!(
            &reward_per_sec * &reward_per_share > BigUint::zero(),
            ERROR_WRONG_REWARD_VALUES
        );

        let egld_transferred = self.call_value().egld();
        require!(
            egld_transferred.clone_value() == TOKEN_ISSUANCE_COST,
            ERROR_WRONG_ISSUANCE_AMOUNT
        );

        self.share_token().issue_and_set_all_roles(
            EsdtTokenType::DynamicMeta,
            BigUint::from(TOKEN_ISSUANCE_COST),
            share_token_display_name,
            share_token_ticker,
            WAD_DECIMALS,
            Option::Some(self.callbacks().share_token_issuance_callback(
                &self.blockchain().get_owner_address(),
                &staked_token,
                &reward_token,
                start_ts_ms,
                end_ts_ms,
                &reward_per_sec,
                &reward_per_share,
            )),
        );
    }

    #[endpoint(modifyStartTs)]
    fn modify_start_ts(&self, new_start_ts_ms: u64) {
        self.require_is_admin(&self.blockchain().get_caller());

        let current_ts_ms = self.blockchain().get_block_timestamp_ms();
        require!(
            self.start_ts_ms().get() > current_ts_ms,
            ERROR_ALREADY_STARTED
        );
        require!(new_start_ts_ms > current_ts_ms, ERROR_WRONG_START_TS);

        self.start_ts_ms().set(new_start_ts_ms);

        self.event_start_ts_modified(new_start_ts_ms);
    }

    #[endpoint(modifyEndTs)]
    fn modify_end_ts(&self, new_end_ts_ms: u64) {
        self.require_is_admin(&self.blockchain().get_caller());

        let current_ts_ms = self.blockchain().get_block_timestamp_ms();
        require!(self.end_ts_ms().get() > current_ts_ms, ERROR_ALREADY_ENDED);
        require!(new_end_ts_ms > current_ts_ms, ERROR_WRONG_TIMEFRAME);

        self.end_ts_ms().set(new_end_ts_ms);

        self.event_end_ts_modified(new_end_ts_ms);
    }

    #[endpoint(modifyRewards)]
    fn modify_rewards(&self, new_reward_per_sec: BigUint, new_reward_per_share: BigUint) {
        self.require_is_admin(&self.blockchain().get_caller());

        let current_ts_ms = self.blockchain().get_block_timestamp_ms();
        require!(self.end_ts_ms().get() > current_ts_ms, ERROR_ALREADY_ENDED);
        require!(
            &new_reward_per_sec * &new_reward_per_share > BigUint::zero(),
            ERROR_WRONG_REWARD_VALUES
        );

        self.reward_per_sec().set(&new_reward_per_sec);
        self.reward_per_share().set(&new_reward_per_share);

        self.event_rewards_modified(&new_reward_per_sec, &new_reward_per_share);
    }

    #[payable]
    #[endpoint(depositRewards)]
    fn deposit_rewards(&self) {
        self.require_is_admin(&self.blockchain().get_caller());

        let (token, amount) = self.call_value().egld_or_single_fungible_esdt();
        require!(
            token == self.reward_token().get(),
            ERROR_INVALID_REWARD_TOKEN
        );
        require!(amount > BigUint::zero(), ERROR_ZERO_AMOUNT);

        self.rewards_reserve().update(|c| *c += &amount);

        self.event_rewards_deposited(&amount);
    }

    #[endpoint(withdrawRewards)]
    fn withdraw_rewards(&self, amount: BigUint) {
        self.require_is_admin(&self.blockchain().get_caller());

        require!(amount > BigUint::zero(), ERROR_ZERO_AMOUNT);

        let current_reserve = self.rewards_reserve().get();
        require!(amount <= current_reserve, ERROR_INSUFFICIENT_RESERVE);

        self.rewards_reserve().set(&current_reserve - &amount);

        self.send().direct(
            &self.blockchain().get_owner_address(),
            &self.reward_token().get(),
            0,
            &amount,
        );

        self.event_rewards_withdrawn(&amount);
    }

    // === Views ===

    #[view(getFarmInfo)]
    fn get_farm_info(&self) -> Farm<Self::Api> {
        Farm {
            staked_token: self.staked_token().get(),
            share_token: self.share_token().get_token_id(),
            reward_token: self.reward_token().get(),
            rewards_reserve: self.rewards_reserve().get(),
            start_ts_ms: self.start_ts_ms().get(),
            end_ts_ms: self.end_ts_ms().get(),
            reward_per_sec: self.reward_per_sec().get(),
            reward_per_share: self.reward_per_share().get(),
        }
    }

    // === Storage ===

    #[storage_mapper("staked_token")]
    fn staked_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[storage_mapper("reward_token")]
    fn reward_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[storage_mapper("start_ts_ms")]
    fn start_ts_ms(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("end_ts_ms")]
    fn end_ts_ms(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("reward_per_sec")]
    fn reward_per_sec(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("reward_per_share")]
    fn reward_per_share(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("rewards_reserve")]
    fn rewards_reserve(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("share_token")]
    fn share_token(&self) -> NonFungibleTokenMapper;

    // === Events ===

    #[event("farmCreated")]
    fn event_farm_created(
        &self,
        #[indexed] staked_token: &EgldOrEsdtTokenIdentifier,
        #[indexed] reward_token: &EgldOrEsdtTokenIdentifier,
        #[indexed] start_ts_ms: u64,
        #[indexed] end_ts_ms: u64,
        #[indexed] reward_per_sec: &BigUint,
        #[indexed] reward_per_share: &BigUint,
        #[indexed] share_token: &TokenIdentifier,
    );

    #[event("startTsModified")]
    fn event_start_ts_modified(&self, #[indexed] new_start_ts_ms: u64);

    #[event("endTsModified")]
    fn event_end_ts_modified(&self, #[indexed] new_end_ts_ms: u64);

    #[event("rewardsModified")]
    fn event_rewards_modified(
        &self,
        #[indexed] new_reward_per_sec: &BigUint,
        #[indexed] new_reward_per_share: &BigUint,
    );

    #[event("rewardsDeposited")]
    fn event_rewards_deposited(&self, #[indexed] amount: &BigUint);

    #[event("rewardsWithdrawn")]
    fn event_rewards_withdrawn(&self, #[indexed] amount: &BigUint);

    // === Callbacks ===

    #[callback]
    fn share_token_issuance_callback(
        &self,
        caller: &ManagedAddress,
        staked_token: &EgldOrEsdtTokenIdentifier,
        reward_token: &EgldOrEsdtTokenIdentifier,
        start_ts_ms: u64,
        end_ts_ms: u64,
        reward_per_sec: &BigUint,
        reward_per_share: &BigUint,

        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(share_token_identifier) => {
                self.staked_token().set(staked_token);
                self.reward_token().set(reward_token);
                self.start_ts_ms().set(start_ts_ms);
                self.end_ts_ms().set(end_ts_ms);
                self.reward_per_sec().set(reward_per_sec);
                self.reward_per_share().set(reward_per_share);

                self.event_farm_created(
                    staked_token,
                    reward_token,
                    start_ts_ms,
                    end_ts_ms,
                    reward_per_sec,
                    reward_per_share,
                    &share_token_identifier,
                );

                self.share_token().set_token_id(share_token_identifier);
            }
            ManagedAsyncCallResult::Err(_) => {
                let (token, amount) = self.call_value().egld_or_single_fungible_esdt();
                if token.is_egld() && amount > 0 {
                    self.tx().to(caller).egld(&amount).transfer();
                }
                self.share_token().clear();
            }
        }
    }
}
