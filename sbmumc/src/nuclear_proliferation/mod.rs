//! Nuclear Proliferation Module (782)
use serde::{Deserialize, Serialize};
pub struct NuclearProliferation { pub id: String, pub material: String, pub enrichment_percent: f64, pub security_level: f64 }
impl NuclearProliferation { pub fn new(id: String) -> Self { Self { id, material: "Uranium".into(), enrichment_percent: 0.0, security_level: 95.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let n = NuclearProliferation::new("NP-1".into()); assert_eq!(n.id, "NP-1"); } }
