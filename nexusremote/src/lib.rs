//! NexusRemote - A decentralized P2P remote control system with token incentives
//! 
//! This library provides the core functionality for the NexusRemote system,
//! including P2P networking, token economics, and remote control capabilities.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod core;
pub mod network;
pub mod wallet;
pub mod ui;
pub mod simulator;

// Re-export commonly used types
pub use core::*;
pub use network::*;
pub use wallet::*;

/// Result type for NexusRemote operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for NexusRemote operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Network-related errors
    #[error("Network error: {0}")]
    Network(String),
    
    /// Wallet-related errors
    #[error("Wallet error: {0}")]
    Wallet(String),
    
    /// Token-related errors
    #[error("Token error: {0}")]
    Token(String),
    
    /// Encryption/decryption errors
    #[error("Crypto error: {0}")]
    Crypto(String),
    
    /// Input/output errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Other errors
    #[error("{0}")]
    Other(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}
