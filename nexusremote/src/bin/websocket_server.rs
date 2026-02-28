//! NexusRemote WebSocket Server
//! Provides real-time communication for remote control

use nexusremote::core::crypto::NodeKeypair;
use nexusremote::network::dht::{InMemoryDht, DhtNode};
use nexusremote::core::types::{PeerInfo, PeerID, DeviceID, ReputationScore, NodeRole};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn, error};

/// WebSocket server state
struct WebSocketServer {
    /// Local node keypair
    keypair: NodeKeypair,
    /// DHT for peer discovery
    dht: Arc<Mutex<InMemoryDht>>,
    /// Connected clients
    clients: Arc<Mutex<HashMap<String, ClientInfo>>>,
}

/// Client information
struct ClientInfo {
    /// Client ID
    id: String,
    /// WebSocket connection
    // Note: We'll handle connections separately
    /// Peer info
    peer_info: PeerInfo,
    /// Last activity timestamp
    last_active: std::time::Instant,
}

impl WebSocketServer {
    /// Create a new WebSocket server
    fn new() -> Self {
        let keypair = NodeKeypair::generate();
        let peer_info = PeerInfo {
            peer_id: PeerID::new("websocket-server".to_string()),
            device_id: keypair.node_id(),
            reputation: ReputationScore::new(500),
            role: NodeRole::Relay,
            addresses: vec!["localhost:8081".to_string()],
            available_bandwidth: 100_000_000,
        };
        
        let dht = InMemoryDht::new(peer_info.clone());
        
        Self {
            keypair,
            dht: Arc::new(Mutex::new(dht)),
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Start the WebSocket server
    async fn start(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        info!("WebSocket server listening on {}", addr);
        
        loop {
            let (stream, client_addr) = listener.accept().await?;
            info!("New connection from: {}", client_addr);
            
            let dht_clone = self.dht.clone();
            let clients_clone = self.clients.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, dht_clone, clients_clone).await {
                    error!("Connection error: {}", e);
                }
            });
        }
    }
    
    /// Handle a WebSocket connection
    async fn handle_connection(
        stream: TcpStream,
        dht: Arc<Mutex<InMemoryDht>>,
        clients: Arc<Mutex<HashMap<String, ClientInfo>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = tokio_tungstenite::accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        let client_id = uuid::Uuid::new_v4().to_string();
        
        // Send welcome message
        let welcome_msg = serde_json::json!({
            "type": "welcome",
            "client_id": client_id,
            "message": "Connected to NexusRemote WebSocket server",
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        
        ws_sender.send(Message::Text(welcome_msg.to_string())).await?;
        
        // Handle incoming messages
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("Received message: {}", text);
                    
                    // Parse JSON message
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                        Self::handle_json_message(&json, &mut ws_sender, &dht, &clients, &client_id).await?;
                    } else {
                        warn!("Failed to parse JSON: {}", text);
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("Client {} disconnected", client_id);
                    break;
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
                _ => {} // Ignore other message types
            }
        }
        
        // Remove client from list
        clients.lock().await.remove(&client_id);
        Ok(())
    }
    
    /// Handle JSON message from client
    async fn handle_json_message(
        json: &serde_json::Value,
        ws_sender: &mut futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>,
        dht: &Arc<Mutex<InMemoryDht>>,
        clients: &Arc<Mutex<HashMap<String, ClientInfo>>>,
        client_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let msg_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("unknown");
        
        match msg_type {
            "ping" => {
                let response = serde_json::json!({
                    "type": "pong",
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });
                ws_sender.send(Message::Text(response.to_string())).await?;
            }
            
            "get_peers" => {
                let target_id = json.get("target_id")
                    .and_then(|id| id.as_str())
                    .unwrap_or("");
                
                let dht_lock = dht.lock().await;
                let target = if target_id.is_empty() {
                    // Use random target if none provided
                    DeviceID::new([0u8; 32])
                } else {
                    // Parse hex string to DeviceID
                    DeviceID::from_hex(target_id).unwrap_or_else(|_| DeviceID::new([0u8; 32]))
                };
                
                let peers: Vec<PeerInfo> = dht_lock.find_peer(target).await.unwrap_or_default();
                
                let response = serde_json::json!({
                    "type": "peers",
                    "peers": peers.iter().map(|p| {
                        serde_json::json!({
                            "peer_id": p.peer_id.0,
                            "device_id": p.device_id.to_hex(),
                            "reputation": p.reputation.value(),
                            "role": format!("{}", p.role),
                            "addresses": p.addresses,
                        })
                    }).collect::<Vec<_>>(),
                });
                
                ws_sender.send(Message::Text(response.to_string())).await?;
            }
            
            "send_command" => {
                let command = json.get("command").and_then(|c| c.as_str()).unwrap_or("");
                let target = json.get("target").and_then(|t| t.as_str()).unwrap_or("");
                
                info!("Command received: {} for target: {}", command, target);
                
                let response = serde_json::json!({
                    "type": "command_result",
                    "command": command,
                    "target": target,
                    "status": "received",
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });
                
                ws_sender.send(Message::Text(response.to_string())).await?;
            }
            
            "get_routing_stats" => {
                // 获取本地节点信息
                let dht_lock = dht.lock().await;
                let local_peer = dht_lock.local_peer();
                
                // 模拟一些对等节点数据
                let total_peers = 50;
                let high_rep_count = 15; // 30% 高信誉节点
                let low_rep_count = 20;  // 40% 低信誉节点
                
                let response = serde_json::json!({
                    "type": "routing_stats",
                    "local_node": {
                        "device_id": local_peer.device_id.to_hex(),
                        "reputation": local_peer.reputation.value(),
                        "role": format!("{}", local_peer.role),
                    },
                    "total_peers": total_peers,
                    "high_reputation_peers": high_rep_count,
                    "low_reputation_peers": low_rep_count,
                    "weighted_routing_enabled": true,
                    "expected_advantage": 1.5,
                    "simulation_data": {
                        "high_rep_selection_rate": 0.45, // 45% 选择率（高于30%的随机率）
                        "advantage_ratio": 1.5,
                    }
                });
                
                ws_sender.send(Message::Text(response.to_string())).await?;
            }
            
            _ => {
                warn!("Unknown message type: {}", msg_type);
                let response = serde_json::json!({
                    "type": "error",
                    "message": format!("Unknown message type: {}", msg_type),
                });
                ws_sender.send(Message::Text(response.to_string())).await?;
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    info!("Starting NexusRemote WebSocket Server...");
    
    let server = WebSocketServer::new();
    
    // Start server on port 8081
    server.start("127.0.0.1:8081").await?;
    
    Ok(())
}