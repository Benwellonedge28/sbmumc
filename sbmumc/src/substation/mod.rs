//! Substation Module (790)
use serde::{Deserialize, Serialize};
pub struct Substation { pub id: String, pub voltage_kv: f64, pub capacity_mva: f64, pub switching_type: String }
impl Substation { pub fn new(id: String) -> Self { Self { id, voltage_kv: 0.0, capacity_mva: 0.0, switching_type: "GIS".into() } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let s = Substation::new("SUB-1".into()); assert_eq!(s.id, "SUB-1"); } }
