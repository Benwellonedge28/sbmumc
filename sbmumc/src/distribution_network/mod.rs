//! Distribution Network Module (774)
use serde::{Deserialize, Serialize};
pub struct DistributionNetwork { pub id: String, pub feeders: u32, pub voltage_v: f64, pub reliability_percent: f64 }
impl DistributionNetwork { pub fn new(id: String) -> Self { Self { id, feeders: 0, voltage_v: 11.0, reliability_percent: 99.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let d = DistributionNetwork::new("DN-1".into()); assert_eq!(d.id, "DN-1"); } }
