// 最小化Rust测试 - 不使用标准库
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// 自定义panic处理
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// 入口点
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 简单测试：计算斐波那契数列
    let mut a = 0;
    let mut b = 1;
    
    // 计算前10个斐波那契数
    for _ in 0..10 {
        let c = a + b;
        a = b;
        b = c;
    }
    
    // 最终结果应该在变量中
    // 由于没有标准库，无法打印，但可以验证编译
    
    loop {}
}

// 测试基本类型和操作
#[test]
fn test_basic_operations() {
    // 测试整数运算
    assert_eq!(2 + 2, 4);
    assert_eq!(10 * 10, 100);
    
    // 测试位运算
    let x = 0b1010;
    let y = 0b0110;
    assert_eq!(x & y, 0b0010);
    assert_eq!(x | y, 0b1110);
    
    // 测试数组
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(arr[0], 1);
    assert_eq!(arr[4], 5);
    
    // 测试结构体
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 10, y: 20 };
    assert_eq!(p.x, 10);
    assert_eq!(p.y, 20);
}

// 测试NexusRemote核心类型
mod nexus_types {
    // 设备ID - 32字节
    pub struct DeviceID(pub [u8; 32]);
    
    impl DeviceID {
        pub fn xor_distance(&self, other: &DeviceID) -> [u8; 32] {
            let mut result = [0u8; 32];
            for i in 0..32 {
                result[i] = self.0[i] ^ other.0[i];
            }
            result
        }
    }
    
    // 信誉评分
    pub struct ReputationScore(pub u64);
    
    impl ReputationScore {
        pub fn calculate_weight(&self) -> f64 {
            2000.0 / (self.0 as f64 + 1000.0)
        }
    }
    
    #[test]
    fn test_device_id() {
        let id1 = DeviceID([0xFF; 32]);
        let id2 = DeviceID([0x00; 32]);
        let distance = id1.xor_distance(&id2);
        assert_eq!(distance, [0xFF; 32]);
    }
    
    #[test]
    fn test_reputation_weight() {
        let low_rep = ReputationScore(100);
        let high_rep = ReputationScore(900);
        
        let weight_low = low_rep.calculate_weight();
        let weight_high = high_rep.calculate_weight();
        
        // 高信誉应该获得更低的权重
        assert!(weight_high < weight_low);
        
        // 验证具体值
        assert!((weight_low - 1.818).abs() < 0.001);
        assert!((weight_high - 1.052).abs() < 0.001);
    }
}

// 主测试函数
#[cfg(test)]
mod tests {
    use super::nexus_types::*;
    
    #[test]
    fn test_all() {
        // 运行所有测试
        test_basic_operations();
        
        // 测试NexusRemote类型
        let id1 = DeviceID([0xAA; 32]);
        let id2 = DeviceID([0x55; 32]);
        let distance = id1.xor_distance(&id2);
        assert_eq!(distance, [0xFF; 32]);
        
        let rep = ReputationScore(500);
        let weight = rep.calculate_weight();
        assert!((weight - 1.333).abs() < 0.001);
    }
}

// 如果没有标准库，这个可能无法运行，但可以测试编译
fn main() {
    // 空的主函数，用于编译测试
}
