//! X-ray Diffraction Module (834)
use serde::{Deserialize, Serialize};
pub struct XRDAnalysis { pub id: String, pub radiation: String, pub wavelength_angstrom: f64, pub detector_type: String, pub phase_identification_accuracy: f64 }
impl XRDAnalysis { pub fn new(id: String) -> Self { Self { id, radiation: "Cu K-alpha".into(), wavelength_angstrom: 1.54, detector_type: "Point Detector".into(), phase_identification_accuracy: 95.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let x = XRDAnalysis::new("XRD-1".into()); assert_eq!(x.id, "XRD-1"); } }
