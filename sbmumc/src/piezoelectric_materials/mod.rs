//! Piezoelectric Materials Module (826)
use serde::{Deserialize, Serialize};
pub struct PiezoelectricMaterial { pub id: String, pub material: String, pub d33_pc_n: f64, pub coupling_factor: f64, pub Curie_temp_c: f64 }
impl PiezoelectricMaterial { pub fn new(id: String) -> Self { Self { id, material: "PZT".into(), d33_pc_n: 300.0, coupling_factor: 0.5, Curie_temp_c: 350.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let p = PiezoelectricMaterial::new("PE-1".into()); assert_eq!(p.id, "PE-1"); } }
