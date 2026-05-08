//! Combined Heat and Power Module (778)
use serde::{Deserialize, Serialize};
pub struct CHPSystem { pub id: String, pub electrical_eff: f64, pub thermal_eff: f64, pub total_eff: f64 }
impl CHPSystem { pub fn new(id: String) -> Self { Self { id, electrical_eff: 30.0, thermal_eff: 45.0, total_eff: 75.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let c = CHPSystem::new("CHP-1".into()); assert_eq!(c.id, "CHP-1"); } }
