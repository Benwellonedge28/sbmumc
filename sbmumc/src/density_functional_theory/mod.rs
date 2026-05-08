//! Density Functional Theory Module (830)
use serde::{Deserialize, Serialize};
pub struct DFTCalculation { pub id: String, pub functional: String, pub basis_set: String, pub cutoff_energy_ry: f64, pub k_points: u32, pub accuracy_meV: f64 }
impl DFTCalculation { pub fn new(id: String) -> Self { Self { id, functional: "PBE".into(), basis_set: "PAW".into(), cutoff_energy_ry: 50.0, k_points: 100, accuracy_meV: 10.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let d = DFTCalculation::new("DFT-1".into()); assert_eq!(d.id, "DFT-1"); } }
