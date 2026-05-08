//! Structural Materials Module (816)
use serde::{Deserialize, Serialize};
pub struct StructuralMaterial { pub id: String, pub category: String, pub yield_strength_mpa: f64, pub ultimate_strength_mpa: f64, pub fatigue_limit_mpa: f64 }
impl StructuralMaterial { pub fn new(id: String) -> Self { Self { id, category: "Steel".into(), yield_strength_mpa: 250.0, ultimate_strength_mpa: 460.0, fatigue_limit_mpa: 240.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let s = StructuralMaterial::new("SM-1".into()); assert_eq!(s.id, "SM-1"); } }
