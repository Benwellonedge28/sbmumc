//! Virtual Consul Module (627)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualConsul {
    pub vc_id: String,
    pub region: String,
    pub service_rating: f64,
}

impl VirtualConsul {
    pub fn new() -> Self {
        Self {
            vc_id: String::from("virtual_consul_v1"),
            region: String::from("Virtual Europe"),
            service_rating: 0.95,
        }
    }
}

impl Default for VirtualConsul {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_consul() {
        let c = VirtualConsul::new();
        assert!(c.service_rating > 0.9);
    }
}
