//! Virtual Power Plant Module (786)
use serde::{Deserialize, Serialize};
pub struct VirtualPowerPlant { pub id: String, pub capacity_mw: f64, pub resources: Vec<String>, pub reliability_percent: f64 }
impl VirtualPowerPlant { pub fn new(id: String) -> Self { Self { id, capacity_mw: 0.0, resources: Vec::new(), reliability_percent: 90.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let v = VirtualPowerPlant::new("VPP-1".into()); assert_eq!(v.id, "VPP-1"); } }
