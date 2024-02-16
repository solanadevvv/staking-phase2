use crate::errors::StakeError;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PoolInfo {
    pub stake_bump: u8,
    pub demr_stake_bump: u8,
    pub admin: Pubkey,
    pub collection: [Pubkey; 2],
    pub demr_mint: Pubkey,
    pub energy_per_sec: [u64; 2],
    pub energy_per_box: u64,
    pub stake_start: [i64; 2],
    pub demr_stake_amount: u64,
    pub demr_per_box: [u64; 8],
    pub open_box_rate: [u64; 8],
}

impl PoolInfo {
    pub fn check_collection(&self, collection: &Pubkey) -> bool {
        collection == &self.collection[0] || collection == &self.collection[1]
    }

    pub fn get_collection_index(is_collection1: bool) -> usize {
        if is_collection1 {
            0_usize
        } else {
            1_usize
        }
    }

    pub fn check_stakeable(&self, collection_index: usize, cur: i64) -> Result<()> {
        if self.stake_start[collection_index] > 0 {
            require!(
                cur >= self.stake_start[collection_index],
                StakeError::StartError
            );
        }
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct StakeInfo {
    pub nft_mint: Pubkey,
    pub collection: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ClaimInfo {
    pub reward: u64,
}

#[account]
#[derive(InitSpace)]
pub struct NftInfo {
    pub bump: u8,
    pub staking: bool,
    pub collection: u8,
    pub stake_at: i64,
    pub unstake_at: i64,
    pub staker: Pubkey,
    pub nft_mint: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct UserInfo {
    pub staker: Pubkey,
    pub stake_id: u64,
    pub bump: u8,
    pub pending_box: u128,
    pub energy: u128,
    pub opened_box: u128,
    pub used_energy: u128,
    pub demr_reward: u64,
    pub last_calimed_time: i64,
    pub total_staked: [u64; 2],
}

impl UserInfo {
    pub fn claim_pending_energy(&mut self, cur: i64) {
        if cur > self.last_calimed_time {
            let delta_sec = cur - self.last_calimed_time;
            for collection_index in 0..2 {
                if self.total_staked[collection_index] > 0 {
                    let energy = (delta_sec as u64) * self.total_staked[collection_index];
                    self.energy += energy as u128;
                }
            }
            self.last_calimed_time = cur;
        }
    }
}