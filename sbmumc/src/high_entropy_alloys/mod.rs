//! High Entropy Alloys Module (813)
use serde::{Deserialize, Serialize};
pub struct HighEntropyAlloy { pub id: String, pub elements: Vec<String>, pub equimolar: bool, pub hardness_hv: f64, pub oxidation_resistance_percent: f64 }
impl HighEntropyAlloy { pub fn new(id: String) -> Self { Self { id, elements: vec!["Fe".into(), "Ni".into(), "Co".into(), "Cr".into(), "Mn".into()], equimolar: true, hardness_hv: 600.0, oxidation_resistance_percent: 90.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let h = HighEntropyAlloy::new("HEA-1".into()); assert_eq!(h.id, "HEA-1"); } }
