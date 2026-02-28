//! DHT (Distributed Hash Table) implementation with weighted routing

use crate::core::distance::*;
use crate::core::types::*;
use crate::core::state::NodeState;
use crate::Error;
use async_trait::async_trait;
use std::collections::{HashMap, VecDeque};
use tracing::{info, warn, debug};

/// Kademlia bucket configuration
const K: usize = 20;
const ALPHA: usize = 3;

/// DHT node trait
#[async_trait]
pub trait DhtNode: Send + Sync {
    /// Find peers close to a target ID
    async fn find_peer(&self, target: DeviceID) -> Result<Vec<PeerInfo>, crate::Error>;
    
    /// Store a value in the DHT
    async fn put_value(&mut self, key: [u8; 32], value: Vec<u8>) -> Result<(), crate::Error>;
    
    /// Retrieve a value from the DHT
    async fn get_value(&self, key: [u8; 32]) -> Result<Option<Vec<u8>>, crate::Error>;
    
    /// Add a peer to the routing table
    async fn add_peer(&mut self, peer: PeerInfo) -> Result<(), crate::Error>;
    
    /// Get local peer info
    fn local_peer(&self) -> PeerInfo;
}

/// Weighted Kademlia routing table
#[derive(Clone)]
pub struct WeightedRoutingTable {
    /// Local node info
    local_peer: PeerInfo,
    /// K-buckets, indexed by prefix length
    buckets: Vec<Bucket>,
    /// Maximum bucket size
    k: usize,
}

/// A single K-bucket
#[derive(Clone)]
struct Bucket {
    /// Peers in this bucket, sorted by last seen
    peers: VecDeque<PeerInfo>,
    /// Maximum size
    max_size: usize,
}

impl Bucket {
    fn new(max_size: usize) -> Self {
        Self {
            peers: VecDeque::new(),
            max_size,
        }
    }
    
    fn add_peer(&mut self, peer: PeerInfo) {
        // Remove if already present
        self.peers.retain(|p| p.device_id != peer.device_id);
        
        // Add to front (most recent)
        self.peers.push_front(peer);
        
        // Trim if over capacity
        while self.peers.len() > self.max_size {
            self.peers.pop_back();
        }
    }
    
    fn get_peers(&self) -> Vec<PeerInfo> {
        self.peers.iter().cloned().collect()
    }
    
    fn find_closest(&self, target: &DeviceID, count: usize) -> Vec<PeerInfo> {
        let mut peers: Vec<_> = self.peers.iter().cloned().collect();
        sort_peers_by_distance(&mut peers, target);
        peers.truncate(count);
        peers
    }
}

impl WeightedRoutingTable {
    /// Create a new weighted routing table
    pub fn new(local_peer: PeerInfo, k: usize) -> Self {
        let mut buckets = Vec::with_capacity(257);
        for _ in 0..257 {
            buckets.push(Bucket::new(k));
        }
        
        Self {
            local_peer,
            buckets,
            k,
        }
    }
    
    /// Calculate the bucket index for a given target
    fn bucket_index(&self, target: &DeviceID) -> usize {
        let distance = calculate_raw_xor_distance(&self.local_peer.device_id, target);
        
        // Count the number of leading zero bits
        let mut leading_zeros = 0;
        for &byte in &distance {
            if byte == 0 {
                leading_zeros += 8;
            } else {
                leading_zeros += byte.leading_zeros() as usize;
                break;
            }
        }
        
        leading_zeros.min(256)
    }
    
    /// Add a peer to the routing table
    pub fn add_peer(&mut self, peer: PeerInfo) {
        let bucket_idx = self.bucket_index(&peer.device_id);
        self.buckets[bucket_idx].add_peer(peer);
    }
    
    /// Find the K closest peers to a target
    pub fn find_closest_peers(&self, target: DeviceID, count: usize) -> Vec<PeerInfo> {
        let mut candidates = Vec::new();
        let bucket_idx = self.bucket_index(&target);
        
        // Check the target bucket first
        candidates.extend(self.buckets[bucket_idx].get_peers());
        
        // Check buckets outward from the target
        let mut i = 1;
        while candidates.len() < count && (bucket_idx >= i || bucket_idx + i < self.buckets.len()) {
            if bucket_idx >= i {
                candidates.extend(self.buckets[bucket_idx - i].get_peers());
            }
            if bucket_idx + i < self.buckets.len() {
                candidates.extend(self.buckets[bucket_idx + i].get_peers());
            }
            i += 1;
        }
        
        // Sort by weighted distance and take top K
        sort_peers_by_distance(&mut candidates, &target);
        candidates.truncate(count);
        candidates
    }
    
    /// Get all known peers
    pub fn all_peers(&self) -> Vec<PeerInfo> {
        self.buckets.iter()
            .flat_map(|bucket| bucket.get_peers())
            .collect()
    }
}

/// Simple in-memory DHT implementation for testing
#[derive(Clone)]
pub struct InMemoryDht {
    /// Routing table
    routing_table: WeightedRoutingTable,
    /// Local peer info
    local_peer: PeerInfo,
    /// Value store
    store: std::sync::Arc<std::sync::Mutex<HashMap<[u8; 32], Vec<u8>>>>,
    /// Other DHT nodes (for simulation)
    other_nodes: HashMap<DeviceID, InMemoryDhtHandle>,
}

/// Handle to another in-memory DHT node
#[derive(Clone)]
struct InMemoryDhtHandle {
    store: std::sync::Arc<std::sync::Mutex<HashMap<[u8; 32], Vec<u8>>>>,
    peers: std::sync::Arc<std::sync::Mutex<Vec<PeerInfo>>>,
    peer_info: PeerInfo,
}

impl InMemoryDht {
    /// Create a new in-memory DHT
    pub fn new(local_peer: PeerInfo) -> Self {
        Self {
            routing_table: WeightedRoutingTable::new(local_peer.clone(), K),
            local_peer,
            store: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
            other_nodes: HashMap::new(),
        }
    }
    
    /// Connect to another node
    pub fn connect_to(&mut self, other: &mut InMemoryDht) {
        let other_handle = InMemoryDhtHandle {
            store: other.store.clone(),
            peers: std::sync::Arc::new(std::sync::Mutex::new(other.routing_table.all_peers())),
            peer_info: other.local_peer.clone(),
        };
        
        self.other_nodes.insert(other.local_peer.device_id, other_handle);
        self.routing_table.add_peer(other.local_peer.clone());
        
        let self_handle = InMemoryDhtHandle {
            store: self.store.clone(),
            peers: std::sync::Arc::new(std::sync::Mutex::new(self.routing_table.all_peers())),
            peer_info: self.local_peer.clone(),
        };
        
        other.other_nodes.insert(self.local_peer.device_id, self_handle);
        other.routing_table.add_peer(self.local_peer.clone());
    }
}

#[async_trait]
impl DhtNode for InMemoryDht {
    async fn find_peer(&self, target: DeviceID) -> Result<Vec<PeerInfo>, crate::Error> {
        debug!("Finding peer close to: {}", target);
        
        // Get closest from our routing table
        let mut results = self.routing_table.find_closest_peers(target, K);
        
        // Also query other connected nodes (simulated)
        for handle in self.other_nodes.values() {
            let mut peer_list = handle.peers.lock().unwrap();
            sort_peers_by_distance(&mut peer_list, &target);
            results.extend(peer_list.iter().take(ALPHA).cloned());
        }
        
        // Deduplicate and sort
        results.sort_by(|a, b| compare_by_distance(a, b, &target));
        results.dedup_by(|a, b| a.device_id == b.device_id);
        results.truncate(K);
        
        Ok(results)
    }
    
    async fn put_value(&mut self, key: [u8; 32], value: Vec<u8>) -> Result<(), crate::Error> {
        debug!("Putting value with key: {:?}", &key[..8]);
        
        // Store locally
        let mut store = self.store.lock().unwrap();
        store.insert(key, value.clone());
        
        // Also store on connected nodes (simulated)
        for handle in self.other_nodes.values() {
            let mut other_store = handle.store.lock().unwrap();
            other_store.insert(key, value.clone());
        }
        
        Ok(())
    }
    
    async fn get_value(&self, key: [u8; 32]) -> Result<Option<Vec<u8>>, crate::Error> {
        debug!("Getting value with key: {:?}", &key[..8]);
        
        // Check local store first
        let store = self.store.lock().unwrap();
        if let Some(value) = store.get(&key) {
            return Ok(Some(value.clone()));
        }
        
        // Check connected nodes
        for handle in self.other_nodes.values() {
            let other_store = handle.store.lock().unwrap();
            if let Some(value) = other_store.get(&key) {
                return Ok(Some(value.clone()));
            }
        }
        
        Ok(None)
    }
    
    async fn add_peer(&mut self, peer: PeerInfo) -> Result<(), crate::Error> {
        debug!("Adding peer: {}", peer.device_id);
        self.routing_table.add_peer(peer);
        Ok(())
    }
    
    fn local_peer(&self) -> PeerInfo {
        self.local_peer.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_routing_table_add_and_find() {
        let local_keypair = crate::core::crypto::NodeKeypair::generate();
        let local_peer = PeerInfo {
            peer_id: PeerID::new("local".to_string()),
            device_id: local_keypair.node_id(),
            reputation: ReputationScore::new(500),
            role: NodeRole::Idle,
            addresses: vec![],
            available_bandwidth: 100_000_000,
        };
        
        let mut table = WeightedRoutingTable::new(local_peer.clone(), K);
        
        // Add some peers
        for i in 0..30 {
            let keypair = crate::core::crypto::NodeKeypair::generate();
            let peer = PeerInfo {
                peer_id: PeerID::new(format!("peer{}", i)),
                device_id: keypair.node_id(),
                reputation: ReputationScore::new(100 + i * 30),
                role: NodeRole::Idle,
                addresses: vec![],
                available_bandwidth: 100_000_000,
            };
            table.add_peer(peer);
        }
        
        // Find closest peers
        let target = DeviceID::new([0u8; 32]);
        let closest = table.find_closest_peers(target, 5);
        
        assert_eq!(closest.len(), 5);
    }
    
    #[tokio::test]
    async fn test_in_memory_dht() {
        let keypair1 = crate::core::crypto::NodeKeypair::generate();
        let peer1 = PeerInfo {
            peer_id: PeerID::new("peer1".to_string()),
            device_id: keypair1.node_id(),
            reputation: ReputationScore::new(500),
            role: NodeRole::Idle,
            addresses: vec![],
            available_bandwidth: 100_000_000,
        };
        
        let keypair2 = crate::core::crypto::NodeKeypair::generate();
        let peer2 = PeerInfo {
            peer_id: PeerID::new("peer2".to_string()),
            device_id: keypair2.node_id(),
            reputation: ReputationScore::new(500),
            role: NodeRole::Idle,
            addresses: vec![],
            available_bandwidth: 100_000_000,
        };
        
        let mut dht1 = InMemoryDht::new(peer1);
        let mut dht2 = InMemoryDht::new(peer2);
        
        dht1.connect_to(&mut dht2);
        
        // Test put and get
        let key = [1u8; 32];
        let value = b"test value".to_vec();
        
        // Put value in dht1
        dht1.put_value(key, value.clone()).await.unwrap();
        
        // Get value from dht2
        let retrieved = dht2.get_value(key).await.unwrap();
        
        // The value should be found because dht1 and dht2 are connected
        // and dht1's put_value stores in both its own store and dht2's handle
        assert_eq!(retrieved, Some(value));
    }
}
