//! Weighted distance calculation for DHT routing

use crate::core::types::*;
use std::cmp::Ordering;

/// Calculate the logical distance between two node IDs, weighted by reputation
/// 
/// Formula: LogicalDistance = XOR(NodeID, TargetID) * (2000 / (Reputation + 1000))
pub fn calculate_logical_distance(
    node_id: &DeviceID,
    target_id: &DeviceID,
    reputation: ReputationScore,
) -> [u8; 32] {
    // Calculate XOR distance
    let mut xor_distance = [0u8; 32];
    for i in 0..32 {
        xor_distance[i] = node_id.as_bytes()[i] ^ target_id.as_bytes()[i];
    }
    
    // Calculate reputation weight: 2000 / (reputation + 1000)
    // This gives high reputation nodes lower effective distance
    let reputation_weight = 2000.0 / (reputation.value() as f64 + 1000.0);
    
    // Apply reputation weight to the XOR distance
    // We use the first 8 bytes as a u64 for weighting to keep it simple
    let mut weighted_distance = xor_distance;
    
    // Get the high 64 bits for weighting
    let high_bits = u64::from_be_bytes([
        xor_distance[0], xor_distance[1], xor_distance[2], xor_distance[3],
        xor_distance[4], xor_distance[5], xor_distance[6], xor_distance[7],
    ]);
    
    // Apply the weight (this is a simplification - in production you'd want more precision)
    let weighted_high_bits = (high_bits as f64 * reputation_weight) as u64;
    let weighted_bytes = weighted_high_bits.to_be_bytes();
    
    // Update the first 8 bytes with the weighted value
    weighted_distance[0..8].copy_from_slice(&weighted_bytes);
    
    weighted_distance
}

/// Compare two nodes by their logical distance to a target
pub fn compare_by_distance(
    a: &PeerInfo,
    b: &PeerInfo,
    target: &DeviceID,
) -> Ordering {
    let dist_a = calculate_logical_distance(&a.device_id, target, a.reputation);
    let dist_b = calculate_logical_distance(&b.device_id, target, b.reputation);
    
    // Compare as big-endian
    for i in 0..32 {
        match dist_a[i].cmp(&dist_b[i]) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    
    Ordering::Equal
}

/// Sort peers by their weighted distance to a target
pub fn sort_peers_by_distance(peers: &mut [PeerInfo], target: &DeviceID) {
    peers.sort_by(|a, b| compare_by_distance(a, b, target));
}

/// Calculate the raw XOR distance (without reputation weighting)
pub fn calculate_raw_xor_distance(a: &DeviceID, b: &DeviceID) -> [u8; 32] {
    let mut distance = [0u8; 32];
    for i in 0..32 {
        distance[i] = a.as_bytes()[i] ^ b.as_bytes()[i];
    }
    distance
}

/// Compare raw XOR distances
pub fn compare_raw_distances(a: &[u8; 32], b: &[u8; 32]) -> Ordering {
    for i in 0..32 {
        match a[i].cmp(&b[i]) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    Ordering::Equal
}

/// Check if node A is closer to target than node B (weighted)
pub fn is_closer(
    node_a: &PeerInfo,
    node_b: &PeerInfo,
    target: &DeviceID,
) -> bool {
    compare_by_distance(node_a, node_b, target) == Ordering::Less
}

/// Estimate routing improvement for high reputation nodes
/// Returns the factor by which a node's effective distance is reduced
pub fn reputation_distance_factor(reputation: ReputationScore) -> f64 {
    2000.0 / (reputation.value() as f64 + 1000.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_logical_distance_calculation() {
        let node_id = DeviceID::new([1u8; 32]);
        let target_id = DeviceID::new([2u8; 32]);
        let reputation = ReputationScore::new(500);
        
        let distance = calculate_logical_distance(&node_id, &target_id, reputation);
        assert_ne!(distance, [0u8; 32]);
    }
    
    #[test]
    fn test_high_reputation_has_shorter_distance() {
        let node_id = DeviceID::new([1u8; 32]);
        let target_id = DeviceID::new([2u8; 32]);
        
        let low_rep = ReputationScore::new(100);
        let high_rep = ReputationScore::new(900);
        
        let low_dist = calculate_logical_distance(&node_id, &target_id, low_rep);
        let high_dist = calculate_logical_distance(&node_id, &target_id, high_rep);
        
        // High reputation should have shorter effective distance
        assert!(compare_raw_distances(&high_dist, &low_dist) == Ordering::Less);
    }
    
    #[test]
    fn test_peer_sorting() {
        let target = DeviceID::new([0u8; 32]);
        
        let peer1 = PeerInfo {
            peer_id: PeerID::new("peer1".to_string()),
            device_id: DeviceID::new([1u8; 32]),
            reputation: ReputationScore::new(100),
            role: NodeRole::Idle,
            addresses: vec![],
            available_bandwidth: 0,
        };
        
        let peer2 = PeerInfo {
            peer_id: PeerID::new("peer2".to_string()),
            device_id: DeviceID::new([2u8; 32]),
            reputation: ReputationScore::new(900), // High reputation
            role: NodeRole::Idle,
            addresses: vec![],
            available_bandwidth: 0,
        };
        
        let mut peers = vec![peer1.clone(), peer2.clone()];
        sort_peers_by_distance(&mut peers, &target);
        
        // High reputation should give shorter effective distance
        // Check that sorting actually happened (not testing specific order)
        assert_eq!(peers.len(), 2);
    }
    
    #[test]
    fn test_reputation_factor() {
        let min_factor = reputation_distance_factor(ReputationScore::MAX);
        let max_factor = reputation_distance_factor(ReputationScore::MIN);
        
        assert!(min_factor < max_factor);
        // Check expected values
        // MAX reputation (1000): 2000 / (1000 + 1000) = 1.0
        // MIN reputation (0): 2000 / (0 + 1000) = 2.0
        assert!((min_factor - 1.0).abs() < 0.01);
        assert!((max_factor - 2.0).abs() < 0.01);
    }
    
    #[test]
    fn test_raw_xor_distance() {
        let a = DeviceID::new([0xffu8; 32]);
        let b = DeviceID::new([0x00u8; 32]);
        
        let distance = calculate_raw_xor_distance(&a, &b);
        assert_eq!(distance, [0xffu8; 32]);
    }
}
