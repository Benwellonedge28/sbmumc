//! Materials Characterization Module (833)
use serde::{Deserialize, Serialize};
pub struct MaterialsCharacterization { pub id: String, pub technique: String, pub resolution_nm: f64, pub sample_prep: String, pub information_obtained: Vec<String> }
impl MaterialsCharacterization { pub fn new(id: String) -> Self { Self { id, technique: "TEM".into(), resolution_nm: 0.1, sample_prep: "Ion Milling".into(), information_obtained: vec!["Crystal Structure".into()] } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let m = MaterialsCharacterization::new("MC-1".into()); assert_eq!(m.id, "MC-1"); } }
