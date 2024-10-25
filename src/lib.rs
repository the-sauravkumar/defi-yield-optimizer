use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};
use near_sdk::serde::{Serialize, Deserialize};

/// Represents a yield farming strategy
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Strategy {
    name: String,
    protocol: String,
    apy: u64,  // APY in basis points (1/100th of a percent)
    tvl: Balance,
    min_deposit: Balance,
    is_active: bool,
    last_update: u64,
}

/// Represents a user's position in the yield optimizer
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserPosition {
    amount: Balance,
    strategy_id: u64,
    rewards_claimed: Balance,
    deposit_timestamp: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct YieldOptimizer {
    owner_id: AccountId,
    strategies: UnorderedMap<u64, Strategy>,
    user_positions: LookupMap<AccountId, Vec<UserPosition>>,
    total_tvl: Balance,
    strategy_count: u64,
    governance_token: AccountId,
    min_deposit_amount: Balance,
}

#[near_bindgen]
impl YieldOptimizer {
    #[init]
    pub fn new(owner_id: AccountId, governance_token: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            strategies: UnorderedMap::new(b"s"),
            user_positions: LookupMap::new(b"u"),
            total_tvl: 0,
            strategy_count: 0,
            governance_token,
            min_deposit_amount: 1_000_000_000_000_000_000_000, // 1 NEAR
        }
    }

    #[payable]
    pub fn add_strategy(
        &mut self,
        name: String,
        protocol: String,
        apy: u64,
        min_deposit: U128,
    ) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");
        
        let strategy = Strategy {
            name,
            protocol,
            apy,
            tvl: 0,
            min_deposit: min_deposit.into(),
            is_active: true,
            last_update: env::block_timestamp(),
        };

        self.strategies.insert(&self.strategy_count, &strategy);
        self.strategy_count += 1;
    }

    #[payable]
    pub fn deposit(&mut self, strategy_id: u64) {
        let deposit_amount = env::attached_deposit();
        assert!(deposit_amount >= self.min_deposit_amount, "Deposit too small");

        let mut strategy = self.strategies.get(&strategy_id).expect("Strategy not found");
        assert!(strategy.is_active, "Strategy is not active");
        assert!(deposit_amount >= strategy.min_deposit, "Below strategy minimum");

        let user_id = env::predecessor_account_id();
        let position = UserPosition {
            amount: deposit_amount,
            strategy_id,
            rewards_claimed: 0,
            deposit_timestamp: env::block_timestamp(),
        };

        // Update user positions
        let mut user_positions = self.user_positions
            .get(&user_id)
            .unwrap_or_else(|| Vec::new());
        user_positions.push(position);
        self.user_positions.insert(&user_id, &user_positions);

        // Update strategy TVL
        strategy.tvl += deposit_amount;
        self.strategies.insert(&strategy_id, &strategy);
        self.total_tvl += deposit_amount;
    }

    pub fn claim_rewards(&mut self, position_index: u64) -> Promise {
        let user_id = env::predecessor_account_id();
        let mut user_positions = self.user_positions.get(&user_id).expect("No positions found");
        assert!(position_index < user_positions.len() as u64, "Invalid position index");
        
        let position = &mut user_positions[position_index as usize];
        let strategy = self.strategies.get(&position.strategy_id).expect("Strategy not found");

        // Calculate rewards based on time elapsed and APY
        let time_elapsed = env::block_timestamp() - position.deposit_timestamp;
        let rewards = calculate_rewards(position.amount, strategy.apy, time_elapsed);

        // Update position
        position.rewards_claimed += rewards;
        self.user_positions.insert(&user_id, &user_positions);

        // Transfer rewards to user
        Promise::new(user_id).transfer(rewards)
    }

    pub fn update_strategy_apy(&mut self, strategy_id: u64, new_apy: u64) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");
        let mut strategy = self.strategies.get(&strategy_id).expect("Strategy not found");
        strategy.apy = new_apy;
        strategy.last_update = env::block_timestamp();
        self.strategies.insert(&strategy_id, &strategy);
    }

    // View methods
    pub fn get_strategy(&self, strategy_id: u64) -> Option<Strategy> {
        self.strategies.get(&strategy_id)
    }

    pub fn get_user_positions(&self, user_id: AccountId) -> Vec<UserPosition> {
        self.user_positions.get(&user_id).unwrap_or_else(Vec::new)
    }

    pub fn get_total_tvl(&self) -> U128 {
        U128(self.total_tvl)
    }
}

fn calculate_rewards(amount: Balance, apy: u64, time_elapsed: u64) -> Balance {
    let annual_reward = (amount as u128) * (apy as u128) / 10_000;  // APY in basis points
    let seconds_in_year = 31_536_000_u64;
    ((annual_reward * time_elapsed as u128) / seconds_in_year as u128) as Balance
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContext};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: accounts(0),
            signer_account_id: predecessor_account_id.clone(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 1,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context);
        let contract = YieldOptimizer::new(accounts(1), accounts(2));
        assert_eq!(contract.get_total_tvl(), U128(0));
    }
}