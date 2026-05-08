//! Transmission Lines Module (773)
use serde::{Deserialize, Serialize};
pub struct TransmissionLine { pub id: String, pub voltage_kv: f64, pub length_km: f64, pub capacity_mw: f64 }
impl TransmissionLine { pub fn new(id: String) -> Self { Self { id, voltage_kv: 0.0, length_km: 0.0, capacity_mw: 0.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let t = TransmissionLine::new("TL-1".into()); assert_eq!(t.id, "TL-1"); } }
