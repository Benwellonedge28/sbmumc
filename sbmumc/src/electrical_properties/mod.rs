//! Electrical Properties Module (838)
use serde::{Deserialize, Serialize};
pub struct ElectricalProperty { pub id: String, pub property: String, pub value: f64, pub unit: String, pub temperature_c: f64 }
impl ElectricalProperty { pub fn new(id: String) -> Self { Self { id, property: "Resistivity".into(), value: 1e-8, unit: "ohm·m".into(), temperature_c: 20.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let e = ElectricalProperty::new("EP-1".into()); assert_eq!(e.id, "EP-1"); } }
