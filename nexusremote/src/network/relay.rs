//! Relay node functionality

use crate::core::types::*;
use crate::core::state::NetworkStats;
use crate::Error;
use std::collections::HashMap;

/// Relay configuration
#[derive(Debug, Clone)]
pub struct RelayConfig {
    /// Maximum concurrent relay sessions
    pub max_sessions: usize,
    /// Maximum bandwidth per session (bps)
    pub max_bandwidth_per_session: u64,
    /// Minimum reputation for relaying
    pub min_reputation: ReputationScore,
    /// Token rate per MB
    pub tokens_per_mb: TokenAmount,
}

impl Default for RelayConfig {
    fn default() -> Self {
        Self {
            max_sessions: 10,
            max_bandwidth_per_session: 100_000_000, // 100 Mbps
            min_reputation: ReputationScore::new(100),
            tokens_per_mb: TokenAmount::new(1), // 1 NEXUS per MB
        }
    }
}

/// Relay session
#[derive(Debug, Clone)]
pub struct RelaySession {
    /// Session ID
    pub session_id: [u8; 32],
    /// Client peer
    pub client: PeerID,
    /// Target peer
    pub target: PeerID,
    /// Start time
    pub start_time: u64,
    /// Data relayed (bytes)
    pub data_relayed: u64,
    /// Current bandwidth usage (bps)
    pub current_bandwidth: u64,
    /// Token rate for this session
    pub token_rate: TokenAmount,
}

/// Relay manager
pub struct RelayManager {
    config: RelayConfig,
    sessions: HashMap<[u8; 32], RelaySession>,
    stats: NetworkStats,
}

impl RelayManager {
    /// Create a new relay manager
    pub fn new(config: RelayConfig) -> Self {
        Self {
            config,
            sessions: HashMap::new(),
            stats: NetworkStats::default(),
        }
    }
    
    /// Create a relay manager with default config
    pub fn default() -> Self {
        Self::new(RelayConfig::default())
    }
    
    /// Start a new relay session
    pub fn start_session(
        &mut self,
        client: PeerID,
        target: PeerID,
        reputation: ReputationScore,
    ) -> Result<RelaySession, Error> {
        if self.sessions.len() >= self.config.max_sessions {
            return Err(Error::Network("Max relay sessions reached".to_string()));
        }
        
        if reputation < self.config.min_reputation {
            return Err(Error::Token("Insufficient reputation for relay".to_string()));
        }
        
        let session_id: [u8; 32] = rand::random();
        let session = RelaySession {
            session_id,
            client,
            target,
            start_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data_relayed: 0,
            current_bandwidth: 0,
            token_rate: self.config.tokens_per_mb,
        };
        
        self.sessions.insert(session_id, session.clone());
        
        Ok(session)
    }
    
    /// Record data relayed for a session
    pub fn record_data(&mut self, session_id: &[u8; 32], bytes: u64) -> Result<TokenAmount, Error> {
        let session = self.sessions.get_mut(session_id)
            .ok_or_else(|| Error::Network("Session not found".to_string()))?;
        
        session.data_relayed += bytes;
        self.stats.bytes_sent += bytes;
        self.stats.bytes_received += bytes;
        
        // Calculate tokens earned
        let mb_relayed = (bytes as f64) / (1024.0 * 1024.0);
        let tokens_earned = TokenAmount::new((mb_relayed * self.config.tokens_per_mb.value() as f64) as u128);
        
        Ok(tokens_earned)
    }
    
    /// End a relay session and get final receipt
    pub fn end_session(&mut self, session_id: &[u8; 32]) -> Result<SignedReceipt, Error> {
        let session = self.sessions.remove(session_id)
            .ok_or_else(|| Error::Network("Session not found".to_string()))?;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let duration = now - session.start_time;
        
        // Calculate final amount
        let mb_relayed = (session.data_relayed as f64) / (1024.0 * 1024.0);
        let amount = TokenAmount::new((mb_relayed * session.token_rate.value() as f64) as u128);
        
        // Update stats
        self.stats.relay_sessions += 1;
        self.stats.total_relay_duration += duration;
        self.stats.total_data_relayed += session.data_relayed;
        
        // Create receipt (signatures will be added by the actual implementation)
        Ok(SignedReceipt {
            session_id: *session_id,
            data_relayed: session.data_relayed,
            duration,
            amount,
            relay_signature: vec![],
            client_signature: vec![],
            timestamp: now,
        })
    }
    
    /// Get active sessions
    pub fn active_sessions(&self) -> Vec<&RelaySession> {
        self.sessions.values().collect()
    }
    
    /// Get statistics
    pub fn stats(&self) -> &NetworkStats {
        &self.stats
    }
}
