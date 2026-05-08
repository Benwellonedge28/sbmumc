//! Thermal Barrier Coatings Module (821)
use serde::{Deserialize, Serialize};
pub struct ThermalBarrierCoating { pub id: String, pub thickness_um: f64, pub material: String, pub thermal_conductivity_w_mk: f64, pub service_temp_c: f64 }
impl ThermalBarrierCoating { pub fn new(id: String) -> Self { Self { id, thickness_um: 200.0, material: "YSZ".into(), thermal_conductivity_w_mk: 1.5, service_temp_c: 1200.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let t = ThermalBarrierCoating::new("TBC-1".into()); assert_eq!(t.id, "TBC-1"); } }
