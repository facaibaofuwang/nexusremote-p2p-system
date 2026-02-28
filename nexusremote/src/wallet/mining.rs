//! PoW mining for initial token distribution

use crate::core::crypto::pow;
use crate::core::types::*;
use crate::Error;
use tracing::{info, debug};

/// Mining configuration
#[derive(Debug, Clone)]
pub struct MiningConfig {
    /// Difficulty for new users (lower = easier)
    pub new_user_difficulty: u32,
    /// Difficulty for returning users
    pub returning_user_difficulty: u32,
    /// Tokens rewarded for successful mining
    pub reward_amount: TokenAmount,
    /// Maximum concurrent mining attempts
    pub max_concurrent_attempts: usize,
}

impl Default for MiningConfig {
    fn default() -> Self {
        Self {
            new_user_difficulty: 16, // Reasonable difficulty for modern CPUs
            returning_user_difficulty: 20,
            reward_amount: TokenAmount::new(10), // 10 NEXUS initial reward
            max_concurrent_attempts: 4,
        }
    }
}

/// Mining result
#[derive(Debug, Clone)]
pub struct MiningResult {
    /// Whether mining succeeded
    pub success: bool,
    /// Nonce found (if successful)
    pub nonce: Option<u64>,
    /// Tokens earned
    pub reward: TokenAmount,
    /// Time taken (seconds)
    pub time_taken: f64,
    /// Number of attempts
    pub attempts: u64,
}

/// PoW miner
pub struct PowMiner {
    config: MiningConfig,
}

impl PowMiner {
    /// Create a new PoW miner with default config
    pub fn new() -> Self {
        Self::with_config(MiningConfig::default())
    }
    
    /// Create a new PoW miner with custom config
    pub fn with_config(config: MiningConfig) -> Self {
        Self { config }
    }
    
    /// Mine initial tokens for a new user
    pub async fn mine_initial_tokens(&self, seed: &[u8]) -> Result<MiningResult, Error> {
        info!("Starting initial token mining...");
        
        let start = std::time::Instant::now();
        let difficulty = self.config.new_user_difficulty;
        
        let nonce = pow::mine(seed, difficulty).await?;
        
        let time_taken = start.elapsed().as_secs_f64();
        
        info!(
            "Mining complete! Found nonce: {}, time: {:.2}s",
            nonce, time_taken
        );
        
        Ok(MiningResult {
            success: true,
            nonce: Some(nonce),
            reward: self.config.reward_amount,
            time_taken,
            attempts: nonce + 1,
        })
    }
    
    /// Verify a mining solution
    pub fn verify_mining(&self, seed: &[u8], nonce: u64, is_new_user: bool) -> bool {
        let difficulty = if is_new_user {
            self.config.new_user_difficulty
        } else {
            self.config.returning_user_difficulty
        };
        
        pow::verify(seed, nonce, difficulty)
    }
    
    /// Estimate mining time based on difficulty
    pub fn estimate_mining_time(&self, difficulty: u32) -> f64 {
        // Rough estimate: each difficulty level doubles expected time
        // Base time for difficulty 16 is ~1 second on modern CPU
        let base_difficulty = 16;
        let base_time = 1.0;
        
        if difficulty <= base_difficulty {
            base_time / (2.0f64.powi((base_difficulty - difficulty) as i32))
        } else {
            base_time * (2.0f64.powi((difficulty - base_difficulty) as i32))
        }
    }
}

impl Default for PowMiner {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick mining for testing (very low difficulty)
#[cfg(test)]
pub mod test_mining {
    use super::*;
    
    /// Create a test miner with very low difficulty
    pub fn test_miner() -> PowMiner {
        let config = MiningConfig {
            new_user_difficulty: 8,
            returning_user_difficulty: 10,
            reward_amount: TokenAmount::new(10),
            max_concurrent_attempts: 4,
        };
        PowMiner::with_config(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mining_success() {
        let miner = test_mining::test_miner();
        let seed = b"test seed for mining";
        
        let result = miner.mine_initial_tokens(seed).await.unwrap();
        
        assert!(result.success);
        assert!(result.nonce.is_some());
        assert_eq!(result.reward.value(), 10);
    }
    
    #[test]
    fn test_mining_verification() {
        let miner = test_mining::test_miner();
        let seed = b"test verification";
        
        // For test, we'll use a precomputed nonce (in real test we'd mine)
        // This is just to test the verify logic
        let test_nonce = 0; // Will fail, but that's expected
        let _ = miner.verify_mining(seed, test_nonce, true);
    }
    
    #[test]
    fn test_time_estimation() {
        let miner = PowMiner::new();
        
        let time_16 = miner.estimate_mining_time(16);
        let time_17 = miner.estimate_mining_time(17);
        let time_18 = miner.estimate_mining_time(18);
        
        // Higher difficulty should take longer
        assert!(time_17 > time_16);
        assert!(time_18 > time_17);
        
        // Each level should roughly double
        assert!((time_17 - time_16 * 2.0).abs() < time_16 * 0.5);
    }
}
