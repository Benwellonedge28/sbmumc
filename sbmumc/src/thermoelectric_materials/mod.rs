//! Thermoelectric Materials Module (825)
use serde::{Deserialize, Serialize};
pub struct ThermoelectricMaterial { pub id: String, pub figure_of_merit_zT: f64, pub temp_range_c: (f64, f64), pub conversion_efficiency_percent: f64 }
impl ThermoelectricMaterial { pub fn new(id: String) -> Self { Self { id, figure_of_merit_zT: 1.0, temp_range_c: (100.0, 500.0), conversion_efficiency_percent: 10.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let t = ThermoelectricMaterial::new("TE-1".into()); assert_eq!(t.id, "TE-1"); } }
