//! Superconductor Magnets Module (842)
use serde::{Deserialize, Serialize};
pub struct SuperconductorMagnet { pub id: String, pub field_tesla: f64, pub bore_diameter_mm: f64, pub operating_temp_k: f64, pub quench_protection: bool }
impl SuperconductorMagnet { pub fn new(id: String) -> Self { Self { id, field_tesla: 10.0, bore_diameter_mm: 50.0, operating_temp_k: 4.2, quench_protection: true } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let s = SuperconductorMagnet::new("SCM-1".into()); assert_eq!(s.id, "SCM-1"); } }
