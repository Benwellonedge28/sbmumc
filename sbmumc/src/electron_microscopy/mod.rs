//! Electron Microscopy Module (828)
use serde::{Deserialize, Serialize};
pub struct ElectronMicroscope { pub id: String, pub type_: String, pub resolution_nm: f64, pub magnification_max: u32, pub sample_type: String }
impl ElectronMicroscope { pub fn new(id: String) -> Self { Self { id, type_: "SEM".into(), resolution_nm: 1.0, magnification_max: 500000, sample_type: "Solid".into() } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let e = ElectronMicroscope::new("EM-1".into()); assert_eq!(e.id, "EM-1"); } }
