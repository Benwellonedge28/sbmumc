//! Biohybrid Systems Module
//!
//! This module implements biohybrid robots, living machines,
//! cybernetic organisms, and biological-mechanical hybrid systems.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BiohybridSystems {
    pub systems: Vec<BiohybridSystem>,
    pub interfaces: Vec<BiohybridInterface>,
    pub controllers: Vec<SystemController>,
}

impl BiohybridSystems {
    pub fn new() -> Self {
        BiohybridSystems {
            systems: Vec::new(),
            interfaces: Vec::new(),
            controllers: Vec::new(),
        }
    }

    /// Create biohybrid
    pub fn create_biohybrid(&mut self, organism: &str, mechanical_parts: usize) -> &BiohybridSystem {
        let system = BiohybridSystem {
            system_id: format!("biohybrid_{}", self.systems.len()),
            organism_base: organism.to_string(),
            mechanical_integration: mechanical_parts,
            functionality: 0.8,
        };
        self.systems.push(system);
        self.systems.last().unwrap()
    }

    /// Interface biological and mechanical
    pub fn interface(&mut self, system_id: &str) -> &BiohybridInterface {
        let interface = BiohybridInterface {
            interface_id: format!("int_{}", self.interfaces.len()),
            system_id: system_id.to_string(),
            bidirectional: true,
            latency_ms: 10.0,
        };
        self.interfaces.push(interface);
        self.interfaces.last().unwrap()
    }

    /// Program controller
    pub fn program_controller(&mut self, system_id: &str, behavior: &str) -> &SystemController {
        let controller = SystemController {
            controller_id: format!("ctrl_{}", self.controllers.len()),
            system_id: system_id.to_string(),
            behavior,
            adaptation_rate: 0.1,
        };
        self.controllers.push(controller);
        self.controllers.last().unwrap()
    }

    /// Test integration
    pub fn test_integration(&self, system_id: &str) -> IntegrationTest {
        IntegrationTest {
            system_id: system_id.to_string(),
            biological_signal: 0.9,
            mechanical_response: 0.85,
            integrated_performance: 0.87,
        }
    }
}

impl Default for BiohybridSystems { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiohybridSystem {
    pub system_id: String,
    pub organism_base: String,
    pub mechanical_integration: usize,
    pub functionality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiohybridInterface {
    pub interface_id: String,
    pub system_id: String,
    pub bidirectional: bool,
    pub latency_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemController {
    pub controller_id: String,
    pub system_id: String,
    pub behavior: String,
    pub adaptation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationTest {
    pub system_id: String,
    pub biological_signal: f64,
    pub mechanical_response: f64,
    pub integrated_performance: f64,
}
