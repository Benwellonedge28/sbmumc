//! Demand Response Module (770)
use serde::{Deserialize, Serialize};
pub struct DemandResponse { pub id: String, pub load_mw: f64, pub curtailment_mw: f64, pub incentives_usd_mwh: f64 }
impl DemandResponse { pub fn new(id: String) -> Self { Self { id, load_mw: 0.0, curtailment_mw: 0.0, incentives_usd_mwh: 50.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let d = DemandResponse::new("DR-1".into()); assert_eq!(d.id, "DR-1"); } }
