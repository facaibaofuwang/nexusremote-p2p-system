//! Node state management

use crate::core::types::*;
use crate::core::crypto::NodeKeypair;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Node state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    /// Current node role
    pub role: NodeRole,
    /// Keypair (secret key is not serialized)
    #[serde(skip)]
    pub keypair: Option<NodeKeypair>,
    /// Device ID
    pub device_id: DeviceID,
    /// Reputation score
    pub reputation: ReputationScore,
    /// Known peers
    pub known_peers: HashMap<PeerID, PeerInfo>,
    /// Active sessions
    pub active_sessions: Vec<SessionInfo>,
    /// Last heartbeat timestamp
    pub last_heartbeat: u64,
    /// Network statistics
    pub stats: NetworkStats,
}

impl NodeState {
    /// Create a new node state
    pub fn new(keypair: NodeKeypair) -> Self {
        let device_id = keypair.node_id();
        
        Self {
            role: NodeRole::Idle,
            keypair: Some(keypair),
            device_id,
            reputation: ReputationScore::DEFAULT,
            known_peers: HashMap::new(),
            active_sessions: Vec::new(),
            last_heartbeat: 0,
            stats: NetworkStats::default(),
        }
    }
    
    /// Get the keypair
    pub fn keypair(&self) -> Option<&NodeKeypair> {
        self.keypair.as_ref()
    }
    
    /// Add a known peer
    pub fn add_peer(&mut self, peer: PeerInfo) {
        self.known_peers.insert(peer.peer_id.clone(), peer);
    }
    
    /// Remove a peer
    pub fn remove_peer(&mut self, peer_id: &PeerID) {
        self.known_peers.remove(peer_id);
    }
    
    /// Get peers sorted by reputation (highest first)
    pub fn get_peers_by_reputation(&self) -> Vec<&PeerInfo> {
        let mut peers: Vec<_> = self.known_peers.values().collect();
        peers.sort_by(|a, b| b.reputation.cmp(&a.reputation));
        peers
    }
    
    /// Get peers that can act as relays
    pub fn get_relay_candidates(&self, min_reputation: ReputationScore) -> Vec<&PeerInfo> {
        self.known_peers.values()
            .filter(|p| {
                p.role == NodeRole::Relay || p.role == NodeRole::Idle
                    && p.reputation >= min_reputation
                    && p.available_bandwidth > 0
            })
            .collect()
    }
    
    /// Update role
    pub fn set_role(&mut self, role: NodeRole) {
        self.role = role;
    }
    
    /// Increase reputation
    pub fn increase_reputation(&mut self, delta: u64) {
        self.reputation.increase(delta);
    }
    
    /// Decrease reputation
    pub fn decrease_reputation(&mut self, delta: u64) {
        self.reputation.decrease(delta);
    }
    
    /// Add an active session
    pub fn add_session(&mut self, session: SessionInfo) {
        self.active_sessions.push(session);
    }
    
    /// Remove a session
    pub fn remove_session(&mut self, session_id: &[u8; 32]) {
        self.active_sessions.retain(|s| s.session_id != *session_id);
    }
    
    /// Record data transfer
    pub fn record_data_transfer(&mut self, bytes_sent: u64, bytes_received: u64) {
        self.stats.bytes_sent += bytes_sent;
        self.stats.bytes_received += bytes_received;
    }
    
    /// Record a relay session
    pub fn record_relay_session(&mut self, duration: u64, data_relayed: u64) {
        self.stats.relay_sessions += 1;
        self.stats.total_relay_duration += duration;
        self.stats.total_data_relayed += data_relayed;
    }
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Session ID
    pub session_id: [u8; 32],
    /// Remote peer ID
    pub peer_id: PeerID,
    /// Session type
    pub session_type: SessionType,
    /// Start timestamp
    pub start_time: u64,
    /// Last activity timestamp
    pub last_activity: u64,
    /// Data transferred (bytes)
    pub data_transferred: u64,
}

/// Session type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
    /// Control session (we are controller)
    Control,
    /// Controlled session (we are being controlled)
    Controlled,
    /// Relay session (we are relaying)
    Relay,
}

/// Network statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Total bytes sent
    pub bytes_sent: u64,
    /// Total bytes received
    pub bytes_received: u64,
    /// Number of relay sessions
    pub relay_sessions: u64,
    /// Total relay duration (seconds)
    pub total_relay_duration: u64,
    /// Total data relayed (bytes)
    pub total_data_relayed: u64,
    /// Number of successful connections
    pub successful_connections: u64,
    /// Number of failed connections
    pub failed_connections: u64,
}

/// Connection strategy when funds are insufficient
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStrategy {
    /// Try to use overdraft
    Overdraft,
    /// Degrade to direct connection only
    DegradeDirect,
    /// Offer immediate task to earn tokens
    ImmediateTask,
    /// Disconnect
    Disconnect,
}
