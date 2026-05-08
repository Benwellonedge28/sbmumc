//! Waste Heat Recovery Module (779)
use serde::{Deserialize, Serialize};
pub struct WasteHeatRecovery { pub id: String, pub recovered_mwth: f64, pub temperature_c: f64, pub savings_percent: f64 }
impl WasteHeatRecovery { pub fn new(id: String) -> Self { Self { id, recovered_mwth: 0.0, temperature_c: 200.0, savings_percent: 20.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let w = WasteHeatRecovery::new("WHR-1".into()); assert_eq!(w.id, "WHR-1"); } }
