//! Optical Properties Module (837)
use serde::{Deserialize, Serialize};
pub struct OpticalProperty { pub id: String, pub property: String, pub value: f64, pub wavelength_nm: f64, pub measurement_method: String }
impl OpticalProperty { pub fn new(id: String) -> Self { Self { id, property: "Refractive Index".into(), value: 1.5, wavelength_nm: 589.0, measurement_method: "Ellipsometry".into() } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let o = OpticalProperty::new("OP-1".into()); assert_eq!(o.id, "OP-1"); } }
