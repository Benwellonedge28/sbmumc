//! Advanced Ceramics Module (815)
use serde::{Deserialize, Serialize};
pub struct AdvancedCeramic { pub id: String, pub type_: String, pub max_temp_c: f64, pub thermal_shock_resistance: String, pub applications: Vec<String> }
impl AdvancedCeramic { pub fn new(id: String) -> Self { Self { id, type_: "SiC".into(), max_temp_c: 1600.0, thermal_shock_resistance: "Excellent".into(), applications: vec!["Aerospace".into()] } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let a = AdvancedCeramic::new("AC-1".into()); assert_eq!(a.id, "AC-1"); } }
