//! Solar PV Module (753)
//!
//! Photovoltaic systems, solar panels, and distributed solar generation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PVTechnology {
    Monocrystalline,
    Polycrystalline,
    ThinFilm,
    Perovskite,
    Tandem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarPVSystem {
    pub system_id: String,
    pub pv_technology: PVTechnology,
    pub rated_power_kw: f64,
    pub panel_count: u32,
    pub efficiency_percent: f64,
    pub capacity_factor: f64,
    pub annual_output_kwh: f64,
    pub installation_cost_usd_kw: f64,
}

impl SolarPVSystem {
    pub fn new(system_id: String) -> Self {
        Self {
            system_id,
            pv_technology: PVTechnology::Monocrystalline,
            rated_power_kw: 0.0,
            panel_count: 0,
            efficiency_percent: 0.0,
            capacity_factor: 0.0,
            annual_output_kwh: 0.0,
            installation_cost_usd_kw: 0.0,
        }
    }

    pub fn lcoe_cents_per_kwh(&self) -> f64 {
        let lifetime_years = 25.0;
        let annual_output = self.annual_output_kwh;
        let total_cost = self.installation_cost_usd_kw * self.rated_power_kw;
        let total_output = lifetime_years * annual_output;
        if total_output <= 0.0 { return 0.0; }
        (total_cost / total_output) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solar_pv() {
        let pv = SolarPVSystem::new("PV-001".into());
        assert_eq!(pv.system_id, "PV-001");
    }
}
