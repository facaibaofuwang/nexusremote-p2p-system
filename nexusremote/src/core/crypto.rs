//! Cryptographic utilities for NexusRemote

use ed25519_dalek::{
    Signer, Verifier,
};
use ed25519_dalek::SigningKey as SecretKey;
use ed25519_dalek::VerifyingKey as PublicKey;
use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use rand::RngCore;
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use blake3::Hasher;
use crate::core::types::*;
use crate::Error;

/// Cryptographic keypair for node identity
#[derive(Clone)]
pub struct NodeKeypair {
    secret: SecretKey,
    public: PublicKey,
}

impl std::fmt::Debug for NodeKeypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeKeypair")
            .field("public_key", &hex::encode(self.public.as_bytes()))
            .field("node_id", &self.node_id())
            .finish()
    }
}

impl NodeKeypair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let mut secret_bytes = [0u8; 32];
        csprng.fill_bytes(&mut secret_bytes);
        let secret = SecretKey::from_bytes(&secret_bytes);
        let public = secret.verifying_key();
        Self { secret, public }
    }
    
    /// Create from secret key bytes
    pub fn from_secret_key(secret: &[u8; 32]) -> Result<Self, Error> {
        let secret_key = SecretKey::from_bytes(secret);
        let public_key = secret_key.verifying_key();
        Ok(Self { secret: secret_key, public: public_key })
    }
    
    /// Get the public key
    pub fn public_key(&self) -> &PublicKey {
        &self.public
    }
    
    /// Get the secret key
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret
    }
    
    /// Get the NodeID (hash of public key)
    pub fn node_id(&self) -> DeviceID {
        let mut hasher = Sha256::new();
        hasher.update(self.public.as_bytes());
        let hash = hasher.finalize();
        DeviceID::new(hash.into())
    }
    
    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.secret.sign(message).to_bytes().to_vec()
    }
    
    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        let sig = match Signature::from_slice(signature) {
            Ok(s) => s,
            Err(_) => return false,
        };
        self.public.verify(message, &sig).is_ok()
    }
    
    /// Export keypair to bytes (secret key first, then public key)
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(self.secret.as_bytes());
        bytes[32..].copy_from_slice(self.public.as_bytes());
        bytes
    }
}

/// Hash utility functions
pub mod hash {
    use super::*;
    
    /// Compute SHA-256 hash
    pub fn sha256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// Compute BLAKE3 hash
    pub fn blake3(data: &[u8]) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// Compute hash for DHT key derivation
    pub fn dht_key(key: &[u8]) -> [u8; 32] {
        blake3(key)
    }
}

/// Proof of Work for anti-Sybil protection
pub mod pow {
    use super::*;
    
    /// Mine a PoW with given difficulty
    pub async fn mine(seed: &[u8], difficulty: u32) -> Result<u64, Error> {
        let seed_hash = hash::sha256(seed);
        
        // Simple PoW: find nonce such that hash(seed + nonce) has difficulty leading zeros
        let mut nonce = 0u64;
        
        loop {
            let mut input = Vec::with_capacity(40);
            input.extend_from_slice(&seed_hash);
            input.extend_from_slice(&nonce.to_be_bytes());
            
            let hash = hash::sha256(&input);
            
            // Check leading zeros in bits
            let leading_zeros = count_leading_zeros(&hash);
            
            if leading_zeros >= difficulty {
                return Ok(nonce);
            }
            
            nonce += 1;
            
            // Yield occasionally to not block the event loop
            if nonce % 100_000 == 0 {
                tokio::task::yield_now().await;
            }
        }
    }
    
    /// Verify a PoW solution
    pub fn verify(seed: &[u8], nonce: u64, difficulty: u32) -> bool {
        let seed_hash = hash::sha256(seed);
        
        let mut input = Vec::with_capacity(40);
        input.extend_from_slice(&seed_hash);
        input.extend_from_slice(&nonce.to_be_bytes());
        
        let hash = hash::sha256(&input);
        count_leading_zeros(&hash) >= difficulty
    }
    
    /// Count leading zero bits in a hash
    fn count_leading_zeros(hash: &[u8; 32]) -> u32 {
        let mut count = 0u32;
        
        for &byte in hash {
            if byte == 0 {
                count += 8;
            } else {
                count += byte.leading_zeros() as u32;
                break;
            }
        }
        
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keypair_generation() {
        let keypair = NodeKeypair::generate();
        let node_id = keypair.node_id();
        assert_ne!(node_id.as_bytes(), &[0u8; 32]);
    }
    
    #[test]
    fn test_sign_and_verify() {
        let keypair = NodeKeypair::generate();
        let message = b"Hello, NexusRemote!";
        
        let signature = keypair.sign(message);
        assert!(keypair.verify(message, &signature));
        assert!(!keypair.verify(b"Wrong message", &signature));
    }
    
    #[test]
    fn test_hash_consistency() {
        let data = b"test data";
        let hash1 = hash::sha256(data);
        let hash2 = hash::sha256(data);
        assert_eq!(hash1, hash2);
    }
    
    #[tokio::test]
    async fn test_pow() {
        let seed = b"test seed";
        let difficulty = 8; // Low difficulty for test
        
        let nonce = pow::mine(seed, difficulty).await.unwrap();
        assert!(pow::verify(seed, nonce, difficulty));
        // Note: difficulty + 1 might still pass due to hash properties
        // This is acceptable for the test
    }
}
