//! Metal Alloys Module (798)
use serde::{Deserialize, Serialize};
pub struct MetalAlloy { pub id: String, pub base_metal: String, pub alloying_elements: Vec<String>, pub strength_mpa: f64, pub ductility_percent: f64 }
impl MetalAlloy { pub fn new(id: String) -> Self { Self { id, base_metal: "Aluminum".into(), alloying_elements: vec!["Magnesium".into(), "Silicon".into()], strength_mpa: 300.0, ductility_percent: 10.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let m = MetalAlloy::new("MA-1".into()); assert_eq!(m.id, "MA-1"); } }
