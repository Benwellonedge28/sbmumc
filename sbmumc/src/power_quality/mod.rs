//! Power Quality Module (783)
use serde::{Deserialize, Serialize};
pub struct PowerQuality { pub id: String, pub thd_percent: f64, pub voltage_swell_percent: f64, pub interruption_count: u32 }
impl PowerQuality { pub fn new(id: String) -> Self { Self { id, thd_percent: 5.0, voltage_swell_percent: 0.0, interruption_count: 0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let p = PowerQuality::new("PQ-1".into()); assert_eq!(p.id, "PQ-1"); } }
