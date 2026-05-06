//! Interplanetary Internet Module (687)
//!
//! Interplanetary communication networks, delay-tolerant networking, and deep space connectivity.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocol {
    DTN,
    TCP,
    UDP,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterplanetaryNetwork {
    pub network_name: String,
    pub protocol: NetworkProtocol,
    pub nodes: u32,
    pub coverage: String,
    pub average_latency: f64,          // minutes
    pub max_latency: f64,               // minutes
    pub bandwidth_mbps: f64,
    pub relay_satellites: u32,
    pub protocol_efficiency: f64,       // percent
}

impl InterplanetaryNetwork {
    pub fn new(network_name: String) -> Self {
        Self {
            network_name,
            protocol: NetworkProtocol::DTN,
            nodes: 0,
            coverage: "Mars".into(),
            average_latency: 0.0,
            max_latency: 0.0,
            bandwidth_mbps: 0.0,
            relay_satellites: 0,
            protocol_efficiency: 0.0,
        }
    }

    pub fn end_to_end_delay(&self, distance_au: f64) -> f64 {
        let speed_of_light = 299792458.0; // m/s
        let au_to_meters = 1.496e11;
        (distance_au * au_to_meters * 2.0 / speed_of_light) / 60.0 // minutes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interplanetary_network() {
        let network = InterplanetaryNetwork::new("Mars Network".into());
        assert_eq!(network.network_name, "Mars Network");
    }
}
