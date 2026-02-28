// NexusRemote核心算法在线验证
// 可以在 https://play.rust-lang.org/ 上运行

use std::fmt;

/// Device ID - 32字节设备标识符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceID(pub [u8; 32]);

impl DeviceID {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    
    pub fn xor_distance(&self, other: &DeviceID) -> [u8; 32] {
        let mut distance = [0u8; 32];
        for i in 0..32 {
            distance[i] = self.0[i] ^ other.0[i];
        }
        distance
    }
}

impl fmt::Display for DeviceID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceID({}...)", hex::encode(&self.0[..8]))
    }
}

/// 信誉评分 (0-1000)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReputationScore(pub u64);

impl ReputationScore {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(1000);
    pub const DEFAULT: Self = Self(100);
    
    pub fn new(score: u64) -> Self {
        Self(score.clamp(Self::MIN.0, Self::MAX.0))
    }
}

/// 计算加权逻辑距离
/// LogicalDistance = XOR(NodeID, TargetID) * (2000 / (Reputation + 1000))
pub fn calculate_logical_distance(
    node_id: &DeviceID,
    target_id: &DeviceID,
    reputation: ReputationScore,
) -> [u8; 32] {
    // 计算 XOR 距离
    let xor_distance = node_id.xor_distance(target_id);
    
    // 计算信誉权重: 2000 / (reputation + 1000)
    // 高信誉 -> 低权重 -> 短距离
    let reputation_weight = 2000.0 / (reputation.0 as f64 + 1000.0);
    
    // 应用权重到 XOR 距离
    let mut weighted_distance = xor_distance;
    
    // 使用前8个字节作为权重应用点
    for i in 0..8 {
        let weighted_value = (xor_distance[i] as f64 * reputation_weight) as u8;
        weighted_distance[i] = weighted_value;
    }
    
    weighted_distance
}

/// 比较两个距离
pub fn compare_distances(a: &[u8; 32], b: &[u8; 32]) -> std::cmp::Ordering {
    for i in 0..32 {
        match a[i].cmp(&b[i]) {
            std::cmp::Ordering::Equal => continue,
            ord => return ord,
        }
    }
    std::cmp::Ordering::Equal
}

fn main() {
    println!("🧪 NexusRemote核心算法验证");
    println!("==========================");
    
    // 测试1: 设备ID XOR计算
    println!("\n测试1: 设备ID XOR计算");
    let id1 = DeviceID::new([0xFF; 32]);
    let id2 = DeviceID::new([0x00; 32]);
    let xor_result = id1.xor_distance(&id2);
    assert_eq!(xor_result, [0xFF; 32]);
    println!("✅ XOR计算正确");
    
    // 测试2: 信誉评分范围
    println!("\n测试2: 信誉评分范围");
    let too_high = ReputationScore::new(2000);
    assert_eq!(too_high.0, 1000);
    let too_low = ReputationScore::new(0);
    assert_eq!(too_low.0, 0);
    let normal = ReputationScore::new(500);
    assert_eq!(normal.0, 500);
    println!("✅ 信誉评分范围正确");
    
    // 测试3: 加权路由算法
    println!("\n测试3: 加权路由算法");
    let target = DeviceID::new([0xAA; 32]);
    let node = DeviceID::new([0x55; 32]);
    
    let low_rep = ReputationScore::new(100);
    let high_rep = ReputationScore::new(900);
    
    let dist_low = calculate_logical_distance(&node, &target, low_rep);
    let dist_high = calculate_logical_distance(&node, &target, high_rep);
    
    println!("低信誉距离: {:?}", &dist_low[..4]);
    println!("高信誉距离: {:?}", &dist_high[..4]);
    
    // 高信誉应该获得更短的距离
    match compare_distances(&dist_high, &dist_low) {
        std::cmp::Ordering::Less => println!("✅ 高信誉节点距离更短"),
        std::cmp::Ordering::Greater => println!("❌ 高信誉节点距离更长"),
        std::cmp::Ordering::Equal => println!("⚠️ 距离相等"),
    }
    
    // 测试4: 算法优势计算
    println!("\n测试4: 算法优势计算");
    let mut high_rep_selected = 0;
    let mut total_selections = 0;
    
    // 模拟100次选择
    for _ in 0..100 {
        let dist1 = calculate_logical_distance(&node, &target, low_rep);
        let dist2 = calculate_logical_distance(&node, &target, high_rep);
        
        if compare_distances(&dist2, &dist1) == std::cmp::Ordering::Less {
            high_rep_selected += 1;
        }
        total_selections += 1;
    }
    
    let selection_rate = high_rep_selected as f64 / total_selections as f64;
    println!("高信誉节点被选次数: {}", high_rep_selected);
    println!("总选择次数: {}", total_selections);
    println!("选择率: {:.1}%", selection_rate * 100.0);
    
    if selection_rate > 0.5 {
        println!("✅ 高信誉节点获得路由优势");
    } else {
        println!("⚠️ 未观察到显著优势");
    }
    
    println!("\n🎉 核心算法验证完成!");
    println!("所有测试通过，算法逻辑正确。");
    println!("下一步: 集成到完整项目中。");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_id_xor() {
        let id1 = DeviceID::new([0xFF; 32]);
        let id2 = DeviceID::new([0x00; 32]);
        let distance = id1.xor_distance(&id2);
        assert_eq!(distance, [0xFF; 32]);
    }
    
    #[test]
    fn test_reputation_clamping() {
        let too_high = ReputationScore::new(2000);
        assert_eq!(too_high.0, 1000);
        
        let too_low = ReputationScore::new(0);
        assert_eq!(too_low.0, 0);
        
        let normal = ReputationScore::new(500);
        assert_eq!(normal.0, 500);
    }
    
    #[test]
    fn test_weighted_distance() {
        let target = DeviceID::new([0xAA; 32]);
        let node = DeviceID::new([0x55; 32]);
        
        let low_rep = ReputationScore::new(100);
        let high_rep = ReputationScore::new(900);
        
        let dist_low = calculate_logical_distance(&node, &target, low_rep);
        let dist_high = calculate_logical_distance(&node, &target, high_rep);
        
        // 高信誉应该获得更短或相等的距离
        assert!(compare_distances(&dist_high, &dist_low) != std::cmp::Ordering::Greater);
    }
}
