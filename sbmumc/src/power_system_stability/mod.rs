//! Power System Stability Module (775)
use serde::{Deserialize, Serialize};
pub struct PowerStability { pub id: String, pub stability_type: String, pub margin_percent: f64, pub critical_clearance_s: f64 }
impl PowerStability { pub fn new(id: String) -> Self { Self { id, stability_type: "Transient".into(), margin_percent: 15.0, critical_clearance_s: 0.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let p = PowerStability::new("PS-1".into()); assert_eq!(p.id, "PS-1"); } }
