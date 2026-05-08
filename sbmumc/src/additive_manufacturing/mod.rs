//! Additive Manufacturing Materials Module (811)
use serde::{Deserialize, Serialize};
pub struct AMMaterial { pub id: String, pub process: String, pub material: String, pub density_percent: f64, pub surface_roughness_ra_um: f64 }
impl AMMaterial { pub fn new(id: String) -> Self { Self { id, process: "SLM".into(), material: "Ti-6Al-4V".into(), density_percent: 99.5, surface_roughness_ra_um: 10.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let m = AMMaterial::new("AMM-1".into()); assert_eq!(m.id, "AMM-1"); } }
