//! Topological Insulators Module (808)
use serde::{Deserialize, Serialize};
pub struct TopologicalInsulator { pub id: String, pub material: String, pub surface_conductance_e2_h: f64, pub bulk_gap_mev: f64, pub robustness_percent: f64 }
impl TopologicalInsulator { pub fn new(id: String) -> Self { Self { id, material: "Bi2Se3".into(), surface_conductance_e2_h: 0.5, bulk_gap_mev: 300.0, robustness_percent: 95.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let t = TopologicalInsulator::new("TI-1".into()); assert_eq!(t.id, "TI-1"); } }
