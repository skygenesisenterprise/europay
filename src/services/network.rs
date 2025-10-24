// Network service for inter-node communication

use reqwest::Client;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::core::network::{NetworkMessage, NetworkNode, NetworkProtocol};

pub struct HttpNetworkService {
    client: Client,
    nodes: Arc<Mutex<HashMap<Uuid, NetworkNode>>>,
}

impl HttpNetworkService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register_node(&self, node: NetworkNode) {
        let mut nodes = self.nodes.lock().await;
        nodes.insert(node.id, node);
    }

    pub async fn get_node(&self, id: &Uuid) -> Option<NetworkNode> {
        let nodes = self.nodes.lock().await;
        nodes.get(id).cloned()
    }
}

impl NetworkProtocol for HttpNetworkService {
    async fn send_message(&self, to: &NetworkNode, message: NetworkMessage) -> Result<NetworkMessage, String> {
        let url = format!("{}/network/message", to.address);
        let response = self.client
            .post(&url)
            .json(&message)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if response.status().is_success() {
            let response_message: NetworkMessage = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(response_message)
        } else {
            Err(format!("HTTP error: {}", response.status()))
        }
    }

    async fn broadcast(&self, message: NetworkMessage) -> Result<(), String> {
        let nodes = self.nodes.lock().await;
        let mut handles = vec![];

        for node in nodes.values() {
            let message = message.clone();
            let node = node.clone();
            let client = self.client.clone();

            let handle = tokio::spawn(async move {
                let url = format!("{}/network/message", node.address);
                client.post(&url).json(&message).send().await
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.map_err(|e| format!("Task join error: {}", e))?;
            result.map_err(|e| format!("HTTP request failed: {}", e))?;
        }

        Ok(())
    }
}