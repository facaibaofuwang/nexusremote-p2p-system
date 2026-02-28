//! Core type definitions for NexusRemote

use serde::{Deserialize, Serialize};
use std::fmt;

/// Device ID - unique identifier for a device in the network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceID(pub [u8; 32]);

impl DeviceID {
    /// Create a new DeviceID from bytes
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    
    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    
    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
    
    /// Create from hex string
    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(s, &mut bytes)?;
        Ok(Self(bytes))
    }
}

impl fmt::Display for DeviceID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceID({})", &self.to_hex()[..16])
    }
}

/// Peer ID - libp2p compatible peer identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerID(pub String);

impl PeerID {
    /// Create a new PeerID
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl fmt::Display for PeerID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Token amount - represents NEXUS tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TokenAmount(pub u128);

impl TokenAmount {
    /// Zero tokens
    pub const ZERO: Self = Self(0);
    
    /// Create a new token amount
    pub fn new(amount: u128) -> Self {
        Self(amount)
    }
    
    /// Get the underlying value
    pub fn value(&self) -> u128 {
        self.0
    }
    
    /// Add two token amounts
    pub fn add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }
    
    /// Subtract two token amounts
    pub fn sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }
}

impl fmt::Display for TokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} NEXUS", self.0)
    }
}

/// Reputation score - measures node trustworthiness (0-1000)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ReputationScore(pub u64);

impl ReputationScore {
    /// Minimum possible reputation
    pub const MIN: Self = Self(0);
    /// Maximum possible reputation
    pub const MAX: Self = Self(1000);
    /// Default reputation for new nodes
    pub const DEFAULT: Self = Self(100);
    
    /// Create a new reputation score (clamped to 0-1000)
    pub fn new(score: u64) -> Self {
        Self(score.clamp(Self::MIN.0, Self::MAX.0))
    }
    
    /// Get the score value
    pub fn value(&self) -> u64 {
        self.0
    }
    
    /// Increase reputation
    pub fn increase(&mut self, delta: u64) {
        self.0 = self.0.saturating_add(delta).min(Self::MAX.0);
    }
    
    /// Decrease reputation
    pub fn decrease(&mut self, delta: u64) {
        self.0 = self.0.saturating_sub(delta).max(Self::MIN.0);
    }
}

impl fmt::Display for ReputationScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Node role in the network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeRole {
    /// Controlling another device
    Controller,
    /// Being controlled by another device
    Controlled,
    /// Relaying traffic for others
    Relay,
    /// Idle, can switch roles
    Idle,
}

impl fmt::Display for NodeRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeRole::Controller => write!(f, "Controller"),
            NodeRole::Controlled => write!(f, "Controlled"),
            NodeRole::Relay => write!(f, "Relay"),
            NodeRole::Idle => write!(f, "Idle"),
        }
    }
}

/// Connection quality preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityPreset {
    /// Low quality, low bandwidth
    Low,
    /// Medium quality, balanced
    Medium,
    /// High quality, high bandwidth
    High,
    /// Ultra quality, maximum bandwidth
    Ultra,
}

impl QualityPreset {
    /// Get target bitrate in bps
    pub fn target_bitrate(&self) -> u32 {
        match self {
            QualityPreset::Low => 500_000,
            QualityPreset::Medium => 2_000_000,
            QualityPreset::High => 5_000_000,
            QualityPreset::Ultra => 15_000_000,
        }
    }
}

/// Input event for remote control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEvent {
    /// Mouse movement
    MouseMove { x: i32, y: i32 },
    /// Mouse button press/release
    MouseButton { button: u8, pressed: bool },
    /// Mouse wheel
    MouseWheel { delta_x: i32, delta_y: i32 },
    /// Key press/release
    Key { key_code: u32, pressed: bool },
    /// Unicode character input
    Char(char),
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer ID
    pub peer_id: PeerID,
    /// Device ID
    pub device_id: DeviceID,
    /// Reputation score
    pub reputation: ReputationScore,
    /// Current role
    pub role: NodeRole,
    /// Network addresses
    pub addresses: Vec<String>,
    /// Available bandwidth (bps)
    pub available_bandwidth: u64,
}

/// Signed receipt for relay service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedReceipt {
    /// Relay session ID
    pub session_id: [u8; 32],
    /// Amount of data relayed (bytes)
    pub data_relayed: u64,
    /// Duration (seconds)
    pub duration: u64,
    /// Token amount earned
    pub amount: TokenAmount,
    /// Relay peer signature
    pub relay_signature: Vec<u8>,
    /// Client peer signature
    pub client_signature: Vec<u8>,
    /// Timestamp
    pub timestamp: u64,
}
