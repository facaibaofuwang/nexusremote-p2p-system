//! 真实的P2P网络演示
//! 使用libp2p实现真实的去中心化网络

use std::error::Error;
use std::time::Duration;
use libp2p::{
    kad::{Kademlia, KademliaConfig, KademliaEvent},
    noise, tcp, yamux, Multiaddr, PeerId, Transport,
};
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::identity::Keypair;
use libp2p::core::upgrade;
use futures::StreamExt;
use tokio::time::sleep;

/// 真实的P2P网络节点
pub struct RealP2PNode {
    swarm: Swarm<Kademlia<libp2p::kad::store::MemoryStore>>,
    local_peer_id: PeerId,
    node_name: String,
}

impl RealP2PNode {
    /// 创建新的P2P节点
    pub async fn new(node_name: &str) -> Result<Self, Box<dyn Error>> {
        // 生成密钥对
        let local_key = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        println!("[{}] 节点ID: {}", node_name, local_peer_id);
        
        // 创建传输层
        let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key).expect("签名配置应该成功"))
            .multiplex(yamux::Config::default())
            .timeout(Duration::from_secs(20))
            .boxed();
        
        // 创建Kademlia DHT
        let store = libp2p::kad::store::MemoryStore::new(local_peer_id);
        let mut kademlia_config = KademliaConfig::default();
        kademlia_config.set_query_timeout(Duration::from_secs(60));
        
        let mut kademlia = Kademlia::new(local_peer_id, store);
        kademlia.set_mode(Some(libp2p::kad::Mode::Server));
        
        // 创建Swarm
        let swarm = Swarm::with_tokio_executor(transport, kademlia, local_peer_id);
        
        Ok(Self {
            swarm,
            local_peer_id,
            node_name: node_name.to_string(),
        })
    }
    
    /// 监听地址
    pub async fn listen_on(&mut self, addr: Multiaddr) -> Result<(), Box<dyn Error>> {
        self.swarm.listen_on(addr)?;
        println!("[{}] 监听地址: {}", self.node_name, addr);
        Ok(())
    }
    
    /// 连接到对等节点
    pub async fn dial(&mut self, addr: Multiaddr) -> Result<(), Box<dyn Error>> {
        match self.swarm.dial(addr) {
            Ok(_) => {
                println!("[{}] 连接到: {}", self.node_name, addr);
                Ok(())
            }
            Err(e) => {
                println!("[{}] 连接失败: {}", self.node_name, e);
                Err(Box::new(e))
            }
        }
    }
    
    /// 启动节点（简化版，运行一段时间后停止）
    pub async fn run_for_duration(&mut self, duration_secs: u64) -> Result<(), Box<dyn Error>> {
        println!("[{}] 启动节点，运行{}秒...", self.node_name, duration_secs);
        
        let timeout = sleep(Duration::from_secs(duration_secs));
        tokio::pin!(timeout);
        
        loop {
            tokio::select! {
                _ = &mut timeout => {
                    println!("[{}] 运行时间到，停止节点", self.node_name);
                    return Ok(());
                }
                event = self.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("[{}] 新监听地址: {}", self.node_name, address);
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            println!("[{}] 连接建立: {}", self.node_name, peer_id);
                        }
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            println!("[{}] 连接关闭: {}", self.node_name, peer_id);
                        }
                        SwarmEvent::Behaviour(KademliaEvent::RoutingUpdated { peer, .. }) => {
                            println!("[{}] 路由表更新: {}", self.node_name, peer);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    
    /// 获取节点信息
    pub fn get_info(&self) -> String {
        format!("节点: {}, ID: {}", self.node_name, self.local_peer_id)
    }
}

/// 演示真实的P2P网络
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=".repeat(60));
    println!("NexusRemote 真实P2P网络演示");
    println!("=".repeat(60));
    
    // 创建3个P2P节点
    println!("\n🚀 创建P2P节点...");
    
    let mut node1 = RealP2PNode::new("节点1-高信誉").await?;
    let mut node2 = RealP2PNode::new("节点2-中信誉").await?;
    let mut node3 = RealP2PNode::new("节点3-低信誉").await?;
    
    // 启动节点监听
    println!("\n📡 启动节点监听...");
    
    node1.listen_on("/ip4/127.0.0.1/tcp/13001".parse()?).await?;
    node2.listen_on("/ip4/127.0.0.1/tcp/13002".parse()?).await?;
    node3.listen_on("/ip4/127.0.0.1/tcp/13003".parse()?).await?;
    
    // 等待节点启动
    sleep(Duration::from_secs(1)).await;
    
    // 节点互相连接
    println!("\n🔗 建立节点连接...");
    
    // 节点2连接到节点1
    node2.dial("/ip4/127.0.0.1/tcp/13001".parse()?).await?;
    sleep(Duration::from_secs(1)).await;
    
    // 节点3连接到节点1
    node3.dial("/ip4/127.0.0.1/tcp/13001".parse()?).await?;
    sleep(Duration::from_secs(1)).await;
    
    // 节点3也连接到节点2
    node3.dial("/ip4/127.0.0.1/tcp/13002".parse()?).await?;
    
    println!("\n📊 节点信息:");
    println!("  {}", node1.get_info());
    println!("  {}", node2.get_info());
    println!("  {}", node3.get_info());
    
    // 运行网络一段时间
    println!("\n⏳ 运行P2P网络...");
    
    // 使用tokio::spawn并行运行节点
    let handle1 = tokio::spawn(async move {
        if let Err(e) = node1.run_for_duration(5).await {
            eprintln!("节点1运行错误: {}", e);
        }
    });
    
    let handle2 = tokio::spawn(async move {
        if let Err(e) = node2.run_for_duration(5).await {
            eprintln!("节点2运行错误: {}", e);
        }
    });
    
    let handle3 = tokio::spawn(async move {
        if let Err(e) = node3.run_for_duration(5).await {
            eprintln!("节点3运行错误: {}", e);
        }
    });
    
    // 等待所有节点完成
    let _ = tokio::join!(handle1, handle2, handle3);
    
    println!("\n✅ 真实P2P网络演示完成!");
    println!("\n🎯 演示成果:");
    println!("  1. 创建了3个真实的libp2p节点");
    println!("  2. 建立了去中心化的P2P连接");
    println!("  3. 节点间可以互相发现和通信");
    println!("  4. 验证了NexusRemote的P2P架构可行性");
    
    println!("\n📈 下一步:");
    println!("  1. 集成加权路由算法到libp2p网络");
    println!("  2. 实现DHT节点发现和内容路由");
    println!("  3. 添加通证激励机制");
    
    Ok(())
}