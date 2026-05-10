//! # SBMUMC Module 1321: Spatial Design
//!
//! Systems for architectural space planning.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpatialOrganization {
    Linear,
    Radial,
    Clustered,
    GridBased,
    Organic,
    Centralized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialDesignSystem {
    pub system_id: String,
    pub spatial_organization: SpatialOrganization,
    pub spatial_efficiency: f64,
    pub circulation_flow: f64,
    pub flexibility: f64,
    pub hierarchy_clarity: f64,
}

impl SpatialDesignSystem {
    pub fn new(spatial_organization: SpatialOrganization) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            spatial_organization,
            spatial_efficiency: 0.0,
            circulation_flow: 0.0,
            flexibility: 0.0,
            hierarchy_clarity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.spatial_organization {
            SpatialOrganization::Linear => {
                self.circulation_flow = 0.95 + rand_simple() * 0.05;
                self.spatial_efficiency = 0.90 + rand_simple() * 0.10;
                self.hierarchy_clarity = 0.85 + rand_simple() * 0.14;
            },
            SpatialOrganization::Radial => {
                self.hierarchy_clarity = 0.95 + rand_simple() * 0.05;
                self.circulation_flow = 0.90 + rand_simple() * 0.10;
                self.flexibility = 0.80 + rand_simple() * 0.18;
            },
            SpatialOrganization::Clustered => {
                self.flexibility = 0.95 + rand_simple() * 0.05;
                self.spatial_efficiency = 0.85 + rand_simple() * 0.14;
                self.circulation_flow = 0.75 + rand_simple() * 0.22;
            },
            SpatialOrganization::GridBased => {
                self.spatial_efficiency = 0.90 + rand_simple() * 0.10;
                self.hierarchy_clarity = 0.85 + rand_simple() * 0.14;
                self.circulation_flow = 0.85 + rand_simple() * 0.14;
            },
            SpatialOrganization::Organic => {
                self.flexibility = 0.90 + rand_simple() * 0.10;
                self.spatial_efficiency = 0.80 + rand_simple() * 0.18;
                self.circulation_flow = 0.70 + rand_simple() * 0.25;
            },
            SpatialOrganization::Centralized => {
                self.hierarchy_clarity = 0.90 + rand_simple() * 0.10;
                self.spatial_efficiency = 0.85 + rand_simple() * 0.14;
                self.circulation_flow = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.hierarchy_clarity == 0.0 {
            self.hierarchy_clarity = (self.spatial_efficiency + self.circulation_flow) / 2.0 * (0.6 + rand_simple() * 0.3);
        }
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let mut system = SpatialDesignSystem::new(SpatialOrganization::Linear);
        system.analyze_system().unwrap();
        assert!(system.circulation_flow > 0.8);
    }
}
