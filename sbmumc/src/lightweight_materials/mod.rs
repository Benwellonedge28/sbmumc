//! Lightweight Materials Module (819)
use serde::{Deserialize, Serialize};
pub struct LightweightMaterial { pub id: String, pub material: String, pub density_g_cm3: f64, pub specific_strength: f64, pub applications: Vec<String> }
impl LightweightMaterial { pub fn new(id: String) -> Self { Self { id, material: "Magnesium".into(), density_g_cm3: 1.74, specific_strength: 150.0, applications: vec!["Automotive".into()] } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let l = LightweightMaterial::new("LM-1".into()); assert_eq!(l.id, "LM-1"); } }
