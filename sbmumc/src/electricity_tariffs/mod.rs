//! Electricity Tariffs Module (785)
use serde::{Deserialize, Serialize};
pub struct ElectricityTariff { pub id: String, pub tariff_type: String, pub rate_usd_kwh: f64, pub peak_rate_usd_kwh: f64 }
impl ElectricityTariff { pub fn new(id: String) -> Self { Self { id, tariff_type: "TimeOfUse".into(), rate_usd_kwh: 0.0, peak_rate_usd_kwh: 0.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let t = ElectricityTariff::new("ET-1".into()); assert_eq!(t.id, "ET-1"); } }
