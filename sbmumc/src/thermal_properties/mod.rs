//! Thermal Properties Module (836)
use serde::{Deserialize, Serialize};
pub struct ThermalProperty { pub id: String, pub property: String, pub value: f64, pub unit: String, pub temperature_range_c: (f64, f64) }
impl ThermalProperty { pub fn new(id: String) -> Self { Self { id, property: "Thermal Conductivity".into(), value: 200.0, unit: "W/(m·K)".into(), temperature_range_c: (25.0, 500.0) } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let t = ThermalProperty::new("TP-1".into()); assert_eq!(t.id, "TP-1"); } }
