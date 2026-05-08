//! Magnetic Properties Module (839)
use serde::{Deserialize, Serialize};
pub struct MagneticProperty { pub id: String, pub property: String, pub value: f64, pub unit: String, pub measurement_temperature_k: f64 }
impl MagneticProperty { pub fn new(id: String) -> Self { Self { id, property: "Saturation Magnetization".into(), value: 1e6, unit: "A/m".into(), measurement_temperature_k: 300.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let m = MagneticProperty::new("MAGP-1".into()); assert_eq!(m.id, "MAGP-1"); } }
