//! Photonic Crystals Module (806)
use serde::{Deserialize, Serialize};
pub struct PhotonicCrystal { pub id: String, pub lattice_type: String, pub bandgap_nm: (f64, f64), pub dielectric_ratio: f64, pub defect_tolerance_nm: f64 }
impl PhotonicCrystal { pub fn new(id: String) -> Self { Self { id, lattice_type: "FCC".into(), bandgap_nm: (500.0, 800.0), dielectric_ratio: 3.0, defect_tolerance_nm: 10.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let p = PhotonicCrystal::new("PC-1".into()); assert_eq!(p.id, "PC-1"); } }
