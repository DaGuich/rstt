use std::net::TcpStream;

use anyhow::{anyhow, Result};

pub struct MqttClient {
    stream: TcpStream,
}

impl MqttClient {
    pub fn new(constr: &str) -> Result<MqttClient> {
        let stream = match TcpStream::connect(constr) {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow!("TCP Error: {}", e));
            }
        };
        Ok(MqttClient { stream })
    }
}
