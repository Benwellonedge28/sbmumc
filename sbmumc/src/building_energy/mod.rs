//! Building Energy Module (781)
use serde::{Deserialize, Serialize};
pub struct BuildingEnergy { pub id: String, pub area_m2: f64, pub energy_use_kwh_m2: f64, pub efficiency_rating: String }
impl BuildingEnergy { pub fn new(id: String) -> Self { Self { id, area_m2: 0.0, energy_use_kwh_m2: 0.0, efficiency_rating: "A".into() } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let b = BuildingEnergy::new("BE-1".into()); assert_eq!(b.id, "BE-1"); } }
