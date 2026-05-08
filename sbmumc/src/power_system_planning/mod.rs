//! Power System Planning Module (788)
use serde::{Deserialize, Serialize};
pub struct PowerPlanning { pub id: String, pub peak_load_mw: f64, pub reserve_margin_percent: f64, pub expansion_cost_usd: f64 }
impl PowerPlanning { pub fn new(id: String) -> Self { Self { id, peak_load_mw: 0.0, reserve_margin_percent: 15.0, expansion_cost_usd: 0.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let p = PowerPlanning::new("PSP-1".into()); assert_eq!(p.id, "PSP-1"); } }
