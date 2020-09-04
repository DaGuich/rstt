use std::collections::HashMap;
use std::net::TcpStream;

use anyhow::{anyhow, Result};

enum ClientState {
    /// The client is not connected to the server
    New,
    // The client has sent the connection packet but is not acknowledged yet
    // Connected,

    // The clients connection is acknowledged
    // ConnAcked,
}

/// The MQTT client
pub struct MqttClient {
    /// The tcp stream to the server
    ///
    /// TODO: check if this stream has to be reopened... or can be reopened
    stream: TcpStream,
}

impl MqttClient {
    /// Create a new client instance
    ///
    /// This method does not connect to the server
    pub fn new(constr: &str, options: HashMap<&str, &str>) -> Result<MqttClient> {
        let stream = match TcpStream::connect(constr) {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow!("TCP Error: {}", e));
            }
        };
        Ok(MqttClient { stream })
    }

    /// Connect to the server.
    pub fn connect(self: &mut Self) {}
}
