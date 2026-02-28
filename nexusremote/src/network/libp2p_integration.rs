//! libp2p 集成模块

use libp2p::{
    kad::{Kademlia, KademliaConfig, KademliaEvent, Record},
    noise, tcp, yamux, Multiaddr, PeerId, Transport,
};
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::identity::Keypair;
use libp2p::core::upgrade;
use futures::StreamExt;
use std::error::Error;
use std::time::Duration;
use crate::core::types::*;
use crate::Error as NexusError;

/// libp2p 网络节点
pub struct Libp2pNode {
    swarm: Swarm<Kademlia<libp2p::kad::store::MemoryStore>>,
    local_peer_id: PeerId,
}

impl Libp2pNode {
    /// 创建新的 libp2p 节点
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // 生成密钥对
        let local_key = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        println!("本地节点 ID: {}", local_peer_id);
        
        // 创建传输层 (TCP + Noise + Yamux)
        let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key).expect("签名配置应该成功"))
            .multiplex(yamux::Config::default())
            .timeout(Duration::from_secs(20))
            .boxed();
        
        // 创建 Kademlia DHT
        let store = libp2p::kad::store::MemoryStore::new(local_peer_id);
        let mut kademlia_config = KademliaConfig::default();
        kademlia_config.set_query_timeout(Duration::from_secs(60));
        
        let mut kademlia = Kademlia::new(local_peer_id, store);
        kademlia.set_mode(Some(libp2p::kad::Mode::Server));
        
        // 创建 Swarm
        let swarm = Swarm::with_tokio_executor(transport, kademlia, local_peer_id);
        
        Ok(Self {
            swarm,
            local_peer_id,
        })
    }
    
    /// 监听地址
    pub async fn listen_on(&mut self, addr: Multiaddr) -> Result<(), Box<dyn Error>> {
        self.swarm.listen_on(addr)?;
        println!("监听地址: {}", addr);
        Ok(())
    }
    
    /// 连接到对等节点
    pub async fn dial(&mut self, addr: Multiaddr) -> Result<(), Box<dyn Error>> {
        match self.swarm.dial(addr) {
            Ok(_) => {
                println!("连接到: {}", addr);
                Ok(())
            }
            Err(e) => {
                println!("连接失败: {}", e);
                Err(Box::new(e))
            }
        }
    }
    
    /// 启动节点事件循环
    pub async fn run_event_loop(&mut self) -> Result<(), Box<dyn Error>> {
        println!("启动事件循环...");
        
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("新监听地址: {}", address);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    println!("连接建立: {}", peer_id);
                }
                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                    println!("连接关闭: {}", peer_id);
                }
                SwarmEvent::Behaviour(event) => {
                    self.handle_kademlia_event(event).await?;
                }
                _ => {}
            }
        }
    }
    
    /// 处理 Kademlia 事件
    async fn handle_kademlia_event(&mut self, event: KademliaEvent) -> Result<(), Box<dyn Error>> {
        match event {
            KademliaEvent::OutboundQueryCompleted { result, .. } => {
                match result {
                    libp2p::kad::QueryResult::GetRecord(Ok(result)) => {
                        println!("获取记录成功: {:?}", result);
                    }
                    libp2p::kad::QueryResult::GetRecord(Err(e)) => {
                        println!("获取记录失败: {:?}", e);
                    }
                    libp2p::kad::QueryResult::PutRecord(Ok(_)) => {
                        println!("存储记录成功");
                    }
                    libp2p::kad::QueryResult::PutRecord(Err(e)) => {
                        println!("存储记录失败: {:?}", e);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
    
    /// 获取本地 PeerId
    pub fn local_peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }
    
    /// 转换为 NexusRemote 的 PeerID
    pub fn to_nexus_peer_id(&self) -> PeerID {
        PeerID::new(self.local_peer_id.to_string())
    }
}

/// 启动 libp2p 节点 (示例)
pub async fn start_libp2p_node() -> Result<(), Box<dyn Error>> {
    println!("🚀 启动 libp2p 节点...");
    
    let mut node = Libp2pNode::new().await?;
    
    // 监听本地地址
    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    node.listen_on(listen_addr).await?;
    
    // 启动事件循环
    tokio::spawn(async move {
        if let Err(e) = node.run_event_loop().await {
            println!("事件循环错误: {}", e);
        }
    });
    
    println!("✅ libp2p 节点启动成功");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_libp2p_node_creation() {
        let node = Libp2pNode::new().await;
        assert!(node.is_ok());
    }
}
