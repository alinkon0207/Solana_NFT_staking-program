use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;

use constants::*;
use instructions::*;

declare_id!("9jxdsqoZXLp7m7eerXEnXaDmCak73DjGEaUXwQiubC61");

#[program]
pub mod rs_staking_program {
    use super::*;

    pub fn initialize_staking_pool(
        ctx: Context<InitializeStakingPool>,
        /*reward_policy_by_class: [u16; CLASS_TYPES]*/reward_per_week: u16,
        /*lock_day: u32,*/
    ) -> Result<()> {
        initialize::initialize_staking_pool(ctx, /*reward_policy_by_class*/reward_per_week/*, lock_day*/)
    }

    pub fn stake_nft(ctx: Context<StakeNft>/*, class_id: u32*/) -> Result<()> {
        stake::stake_nft(ctx/*, class_id*/)
    }

    pub fn unstake_nft(ctx: Context<WithdrawNft>) -> Result<()> {
        unstake::unstake_nft(ctx)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        claim_reward::claim_reward(ctx)
    }

    pub fn change_reward_mint(ctx: Context<ChangeRewardMint>, reward_mint: Pubkey) -> Result<()> {
        update_token_mint::handle(ctx, reward_mint)
    }

    pub fn change_pool_setting(
        ctx: Context<ChangePoolSetting>,
        /*reward_policy_by_class: [u16; CLASS_TYPES]*/reward_per_week: u16,
        /*lock_day: u32,*/
        paused: bool,
    ) -> Result<()> {
        update_config::handle(ctx, /*reward_policy_by_class*/reward_per_week, /*lock_day,*/ paused)
    }

    pub fn deposit_swrd(ctx: Context<DepositSwrd>, amount: u64) -> Result<()> {
        // Transfer reward tokens into the vault.
        deposit_reward::handle(ctx, amount)
    }

    pub fn withdraw_swrd(ctx: Context<WithdrawSwrd>) -> Result<()> {
        withdraw_reward::handle(ctx)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>, new_admin: Pubkey) -> Result<()> {
        transfer_ownership::handle(ctx, new_admin)
    }
}
