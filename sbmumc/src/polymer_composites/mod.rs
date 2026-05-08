//! Polymer Composites Module (817)
use serde::{Deserialize, Serialize};
pub struct PolymerComposite { pub id: String, pub polymer_matrix: String, pub filler: String, pub filler_percent: f64, pub tensile_strength_mpa: f64 }
impl PolymerComposite { pub fn new(id: String) -> Self { Self { id, polymer_matrix: "Epoxy".into(), filler: "Glass Fiber".into(), filler_percent: 60.0, tensile_strength_mpa: 300.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let p = PolymerComposite::new("PC-1".into()); assert_eq!(p.id, "PC-1"); } }
