//! Power Trading Module (787)
use serde::{Deserialize, Serialize};
pub struct PowerTrading { pub id: String, pub volume_mwh: f64, pub price_usd_mwh: f64, pub contract_type: String }
impl PowerTrading { pub fn new(id: String) -> Self { Self { id, volume_mwh: 0.0, price_usd_mwh: 0.0, contract_type: "Bilateral".into() } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let p = PowerTrading::new("PT-1".into()); assert_eq!(p.id, "PT-1"); } }
