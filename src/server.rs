use ureq::Agent;
use log::{info, error};

pub struct ServerManager {
    agent: Agent,
    endpoint: String,
}

impl ServerManager {
    pub fn new(endpoint: String) -> Self {
        let agent = Agent::new();
        ServerManager { agent, endpoint }
    }

    pub fn send_data(&self, data: &serde_json::Value) {
        info!("Sending data to server: {:?}", data);
        match self.agent.post(&self.endpoint).send_json(data) {
            Ok(_) => info!("Data sent successfully"),
            Err(e) => error!("Failed to send data: {:?}", e),
        }
    }
}
