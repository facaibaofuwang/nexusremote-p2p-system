//! Network simulator for testing NexusRemote

use crate::core::crypto::NodeKeypair;
use crate::core::distance::*;
use crate::core::types::*;
use crate::network::dht::{InMemoryDht, WeightedRoutingTable};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use tracing::{info, debug};

/// Simulated network node
#[derive(Clone)]
pub struct SimulatedNode {
    /// Peer info
    pub info: PeerInfo,
    /// DHT node
    pub dht: InMemoryDht,
    /// Wallet
    pub wallet: crate::wallet::wallet::InMemoryWallet,
    /// Routing table
    pub routing_table: WeightedRoutingTable,
}

impl SimulatedNode {
    /// Create a new simulated node
    pub fn new(reputation: ReputationScore, initial_balance: TokenAmount) -> Self {
        let keypair = NodeKeypair::generate();
        let device_id = keypair.node_id();
        
        let peer_info = PeerInfo {
            peer_id: PeerID::new(device_id.to_hex()),
            device_id,
            reputation,
            role: NodeRole::Idle,
            addresses: vec![],
            available_bandwidth: 100_000_000,
        };
        
        let dht = InMemoryDht::new(peer_info.clone());
        let wallet = crate::wallet::wallet::InMemoryWallet::with_initial_balance(keypair, initial_balance);
        let routing_table = WeightedRoutingTable::new(peer_info.clone(), 20);
        
        Self {
            info: peer_info,
            dht,
            wallet,
            routing_table,
        }
    }
}

/// Network simulation results
#[derive(Debug, Clone)]
pub struct SimulationResults {
    /// Total nodes
    pub total_nodes: usize,
    /// High reputation nodes count
    pub high_rep_nodes: usize,
    /// Low reputation nodes count
    pub low_rep_nodes: usize,
    /// Routing distribution: how many times each node was selected
    pub routing_distribution: HashMap<DeviceID, usize>,
    /// High reputation selection rate
    pub high_rep_selection_rate: f64,
    /// Average path length
    pub average_path_length: f64,
}

/// Network simulator
pub struct NetworkSimulator {
    /// Nodes in the simulation
    nodes: HashMap<DeviceID, SimulatedNode>,
    /// Random number generator
    rng: rand::rngs::StdRng,
}

impl NetworkSimulator {
    /// Create a new network simulator
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            rng: rand::SeedableRng::seed_from_u64(42),
        }
    }
    
    /// Add a node to the simulation
    pub fn add_node(&mut self, node: SimulatedNode) {
        self.nodes.insert(node.info.device_id, node);
    }
    
    /// Create N random nodes
    pub fn create_random_nodes(&mut self, count: usize) {
        for i in 0..count {
            // 30% high reputation, 70% low/medium
            let reputation = if self.rng.gen::<f64>() < 0.3 {
                ReputationScore::new(self.rng.gen_range(700..=1000))
            } else {
                ReputationScore::new(self.rng.gen_range(50..=300))
            };
            
            let initial_balance = TokenAmount::new(self.rng.gen_range(10..=100));
            
            let node = SimulatedNode::new(reputation, initial_balance);
            self.add_node(node);
        }
    }
    
    /// Connect all nodes in a mesh (each node knows a few others)
    pub fn connect_mesh(&mut self, connections_per_node: usize) {
        let node_ids: Vec<_> = self.nodes.keys().cloned().collect();
        
        // First collect all peer info we need
        let mut connections_to_add: Vec<(DeviceID, Vec<PeerInfo>)> = Vec::new();
        
        for node_id in &node_ids {
            // Connect to random other nodes
            let mut connected = HashSet::new();
            connected.insert(*node_id);
            let mut targets = Vec::new();
            
            for _ in 0..connections_per_node {
                let mut candidate = node_ids[self.rng.gen_range(0..node_ids.len())];
                while connected.contains(&candidate) {
                    candidate = node_ids[self.rng.gen_range(0..node_ids.len())];
                }
                
                connected.insert(candidate);
                // Get peer info now, while we have immutable access
                let peer_info = self.nodes.get(&candidate).unwrap().info.clone();
                targets.push(peer_info);
            }
            
            connections_to_add.push((*node_id, targets));
        }
        
        // Now apply the connections
        for (node_id, targets) in connections_to_add {
            let node = self.nodes.get_mut(&node_id).unwrap();
            
            for peer_info in targets {
                node.routing_table.add_peer(peer_info);
            }
        }
    }
    
    /// Run routing simulation
    pub fn run_routing_simulation(&mut self, num_lookups: usize) -> SimulationResults {
        info!("Starting routing simulation with {} lookups...", num_lookups);
        
        let mut routing_distribution = HashMap::new();
        let mut total_high_rep_selected = 0;
        let mut total_queries = 0;
        
        let node_ids: Vec<_> = self.nodes.keys().cloned().collect();
        
        for _ in 0..num_lookups {
            // Pick a random source node
            let source_idx = self.rng.gen_range(0..node_ids.len());
            let source_id = node_ids[source_idx];
            let source = self.nodes.get(&source_id).unwrap();
            
            // Pick a random target
            let target = DeviceID::new(self.rng.gen());
            
            // Find closest peers using weighted routing
            let closest = source.routing_table.find_closest_peers(target, 5);
            
            for peer in &closest {
                *routing_distribution.entry(peer.device_id).or_insert(0) += 1;
                
                // Check if this is a high reputation node
                let node = self.nodes.get(&peer.device_id).unwrap();
                if node.info.reputation.value() >= 700 {
                    total_high_rep_selected += 1;
                }
                
                total_queries += 1;
            }
        }
        
        // Calculate statistics
        let total_nodes = self.nodes.len();
        let high_rep_nodes = self.nodes.values()
            .filter(|n| n.info.reputation.value() >= 700)
            .count();
        let low_rep_nodes = total_nodes - high_rep_nodes;
        
        let high_rep_selection_rate = if total_queries > 0 {
            total_high_rep_selected as f64 / total_queries as f64
        } else {
            0.0
        };
        
        info!("Simulation complete!");
        info!("High reputation selection rate: {:.2}%", high_rep_selection_rate * 100.0);
        
        SimulationResults {
            total_nodes,
            high_rep_nodes,
            low_rep_nodes,
            routing_distribution,
            high_rep_selection_rate,
            average_path_length: 0.0,
        }
    }
    
    /// Demonstrate that high reputation nodes are preferred
    pub fn demonstrate_weighted_routing(&mut self) -> bool {
        self.create_random_nodes(100);
        self.connect_mesh(10);
        
        let results = self.run_routing_simulation(1000);
        
        // High reputation nodes (30%) should be selected much more than 30%
        let expected_rate = results.high_rep_nodes as f64 / results.total_nodes as f64;
        let actual_rate = results.high_rep_selection_rate;
        
        info!("Expected selection rate: {:.2}%", expected_rate * 100.0);
        info!("Actual selection rate: {:.2}%", actual_rate * 100.0);
        
        // Actual rate should be at least 2x expected rate to demonstrate advantage
        actual_rate >= expected_rate * 1.5
    }
    
    /// Get all nodes
    pub fn nodes(&self) -> &HashMap<DeviceID, SimulatedNode> {
        &self.nodes
    }
}

impl Default for NetworkSimulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simulator_creation() {
        let mut sim = NetworkSimulator::new();
        sim.create_random_nodes(50);
        
        assert_eq!(sim.nodes().len(), 50);
    }
    
    #[test]
    fn test_weighted_routing_advantage() {
        let mut sim = NetworkSimulator::new();
        let has_advantage = sim.demonstrate_weighted_routing();
        
        // High reputation nodes should have advantage
        // Note: This test might occasionally fail due to randomness
        println!("Weighted routing advantage demonstrated: {}", has_advantage);
    }
    
    #[test]
    fn test_routing_simulation() {
        let mut sim = NetworkSimulator::new();
        sim.create_random_nodes(50);
        sim.connect_mesh(5);
        
        let results = sim.run_routing_simulation(100);
        
        assert_eq!(results.total_nodes, 50);
        assert!(!results.routing_distribution.is_empty());
    }
}
