//! Electricity Market Module (776)
use serde::{Deserialize, Serialize};
pub struct ElectricityMarket { pub id: String, pub market_type: String, pub price_usd_mwh: f64, pub volume_mwh: f64 }
impl ElectricityMarket { pub fn new(id: String) -> Self { Self { id, market_type: "Spot".into(), price_usd_mwh: 0.0, volume_mwh: 0.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let m = ElectricityMarket::new("EM-1".into()); assert_eq!(m.id, "EM-1"); } }
