//! Wallet implementation for NexusRemote

use crate::core::crypto::NodeKeypair;
use crate::core::types::*;
use crate::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, debug};

/// Wallet state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletState {
    /// Current balance
    pub balance: TokenAmount,
    /// Reputation score
    pub reputation: ReputationScore,
    /// Overdraft limit (can go negative up to this amount)
    pub overdraft_limit: TokenAmount,
    /// Total tokens earned
    pub total_earned: TokenAmount,
    /// Total tokens spent
    pub total_spent: TokenAmount,
    /// Open payment channels
    pub channels: HashMap<PeerID, PaymentChannel>,
    /// Transaction history
    pub transactions: Vec<Transaction>,
}

impl WalletState {
    /// Create a new wallet state
    pub fn new() -> Self {
        Self {
            balance: TokenAmount::ZERO,
            reputation: ReputationScore::DEFAULT,
            overdraft_limit: TokenAmount::new(50), // Default 50 NEXUS
            total_earned: TokenAmount::ZERO,
            total_spent: TokenAmount::ZERO,
            channels: HashMap::new(),
            transactions: Vec::new(),
        }
    }
}

impl Default for WalletState {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID
    pub id: [u8; 32],
    /// Transaction type
    pub tx_type: TransactionType,
    /// Amount
    pub amount: TokenAmount,
    /// Counterparty (if any)
    pub counterparty: Option<PeerID>,
    /// Timestamp
    pub timestamp: u64,
    /// Description
    pub description: String,
}

/// Transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Tokens mined via PoW
    Mining,
    /// Tokens earned from relaying
    RelayEarnings,
    /// Tokens spent on relay
    RelayPayment,
    /// Tokens transferred
    Transfer,
    /// System reward
    Reward,
}

/// Payment channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentChannel {
    /// Channel ID
    pub channel_id: [u8; 32],
    /// Remote peer
    pub peer_id: PeerID,
    /// Total capacity
    pub capacity: TokenAmount,
    /// Current balance (our side)
    pub our_balance: TokenAmount,
    /// Current balance (their side)
    pub their_balance: TokenAmount,
    /// Last update timestamp
    pub last_update: u64,
    /// Channel status
    pub status: ChannelStatus,
}

/// Channel status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelStatus {
    /// Channel is open and active
    Open,
    /// Channel is being closed
    Closing,
    /// Channel is closed
    Closed,
}

/// Wallet engine trait
#[async_trait::async_trait]
pub trait WalletEngine: Send + Sync {
    /// Get wallet status
    fn get_status(&self) -> WalletState;
    
    /// Get current balance
    fn balance(&self) -> TokenAmount;
    
    /// Check if can pay (including overdraft)
    fn can_pay(&self, amount: TokenAmount) -> bool;
    
    /// Get effective balance (including overdraft)
    fn effective_balance(&self) -> TokenAmount;
    
    /// Add tokens to wallet
    fn add_tokens(&mut self, amount: TokenAmount, description: &str, tx_type: TransactionType);
    
    /// Spend tokens from wallet
    fn spend_tokens(&mut self, amount: TokenAmount, description: &str, counterparty: Option<&PeerID>) -> Result<(), Error>;
    
    /// Open a payment channel
    async fn open_channel(&mut self, peer_id: PeerID, capacity: TokenAmount) -> Result<PaymentChannel, Error>;
    
    /// Close a payment channel
    async fn close_channel(&mut self, peer_id: &PeerID) -> Result<(), Error>;
    
    /// Submit a relay proof for earnings
    async fn submit_proof_of_relay(&mut self, receipt: SignedReceipt) -> Result<(), Error>;
    
    /// Calculate dynamic overdraft limit based on reputation
    fn calculate_overdraft_limit(&self) -> TokenAmount;
}

/// In-memory wallet implementation
#[derive(Clone)]
pub struct InMemoryWallet {
    state: WalletState,
    keypair: NodeKeypair,
}

impl InMemoryWallet {
    /// Create a new in-memory wallet
    pub fn new(keypair: NodeKeypair) -> Self {
        Self {
            state: WalletState::new(),
            keypair,
        }
    }
    
    /// Create a wallet with initial balance
    pub fn with_initial_balance(keypair: NodeKeypair, initial_balance: TokenAmount) -> Self {
        let mut wallet = Self::new(keypair);
        wallet.state.balance = initial_balance;
        wallet
    }
}

#[async_trait::async_trait]
impl WalletEngine for InMemoryWallet {
    fn get_status(&self) -> WalletState {
        self.state.clone()
    }
    
    fn balance(&self) -> TokenAmount {
        self.state.balance
    }
    
    fn can_pay(&self, amount: TokenAmount) -> bool {
        let effective = self.effective_balance();
        effective.value() >= amount.value()
    }
    
    fn effective_balance(&self) -> TokenAmount {
        TokenAmount::new(
            self.state.balance.value().saturating_add(self.state.overdraft_limit.value())
        )
    }
    
    fn add_tokens(&mut self, amount: TokenAmount, description: &str, tx_type: TransactionType) {
        let tx = Transaction {
            id: rand::random(),
            tx_type,
            amount,
            counterparty: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            description: description.to_string(),
        };
        
        self.state.balance = self.state.balance.add(amount);
        self.state.total_earned = self.state.total_earned.add(amount);
        self.state.transactions.push(tx);
        
        info!("Added {}: {} - {}", amount, tx_type as u8, description);
    }
    
    fn spend_tokens(&mut self, amount: TokenAmount, description: &str, counterparty: Option<&PeerID>) -> Result<(), Error> {
        if !self.can_pay(amount) {
            return Err(Error::Token("Insufficient funds".to_string()));
        }
        
        let tx = Transaction {
            id: rand::random(),
            tx_type: TransactionType::Transfer,
            amount,
            counterparty: counterparty.cloned(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            description: description.to_string(),
        };
        
        self.state.balance = self.state.balance.sub(amount).unwrap_or(TokenAmount::ZERO);
        self.state.total_spent = self.state.total_spent.add(amount);
        self.state.transactions.push(tx);
        
        info!("Spent {} - {}", amount, description);
        
        Ok(())
    }
    
    async fn open_channel(&mut self, peer_id: PeerID, capacity: TokenAmount) -> Result<PaymentChannel, Error> {
        if !self.can_pay(capacity) {
            return Err(Error::Token("Insufficient funds for channel capacity".to_string()));
        }
        
        // Lock the capacity
        self.state.balance = self.state.balance.sub(capacity).unwrap();
        
        let channel = PaymentChannel {
            channel_id: rand::random(),
            peer_id: peer_id.clone(),
            capacity,
            our_balance: capacity,
            their_balance: TokenAmount::ZERO,
            last_update: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            status: ChannelStatus::Open,
        };
        
        self.state.channels.insert(peer_id, channel.clone());
        
        debug!("Opened payment channel with capacity: {}", capacity);
        
        Ok(channel)
    }
    
    async fn close_channel(&mut self, peer_id: &PeerID) -> Result<(), Error> {
        if let Some(channel) = self.state.channels.remove(peer_id) {
            // Return remaining balance
            self.state.balance = self.state.balance.add(channel.our_balance);
            debug!("Closed payment channel, returned: {}", channel.our_balance);
        }
        Ok(())
    }
    
    async fn submit_proof_of_relay(&mut self, receipt: SignedReceipt) -> Result<(), Error> {
        // In a real implementation, we would verify the signatures
        // For now, just add the earnings
        self.add_tokens(receipt.amount, "Relay earnings", TransactionType::RelayEarnings);
        self.state.reputation.increase(1);
        Ok(())
    }
    
    fn calculate_overdraft_limit(&self) -> TokenAmount {
        // Overdraft limit increases with reputation
        // Base 50 + (reputation / 10)
        let base = 50;
        let bonus = self.state.reputation.value() / 10;
        TokenAmount::new(base + bonus as u128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wallet_basics() {
        let keypair = crate::core::crypto::NodeKeypair::generate();
        let mut wallet = InMemoryWallet::new(keypair);
        
        assert_eq!(wallet.balance(), TokenAmount::ZERO);
        // Wallet has overdraft, so can_pay should return true for small amounts
        assert!(wallet.can_pay(TokenAmount::new(10)));
        
        wallet.add_tokens(TokenAmount::new(100), "Test mining", TransactionType::Mining);
        
        assert_eq!(wallet.balance().value(), 100);
        assert!(wallet.can_pay(TokenAmount::new(50)));
    }
    
    #[test]
    fn test_overdraft() {
        let keypair = crate::core::crypto::NodeKeypair::generate();
        let wallet = InMemoryWallet::new(keypair);
        
        // Should have overdraft limit
        assert!(wallet.effective_balance().value() >= 50);
        assert!(wallet.can_pay(TokenAmount::new(25)));
    }
    
    #[test]
    fn test_spend_tokens() {
        let keypair = crate::core::crypto::NodeKeypair::generate();
        let mut wallet = InMemoryWallet::with_initial_balance(keypair, TokenAmount::new(100));
        
        let peer = PeerID::new("test".to_string());
        
        wallet.spend_tokens(TokenAmount::new(50), "Test payment", Some(&peer)).unwrap();
        
        assert_eq!(wallet.balance().value(), 50);
    }
}
