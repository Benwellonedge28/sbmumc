//! Transformers Module (784)
use serde::{Deserialize, Serialize};
pub struct Transformer { pub id: String, pub rating_kva: f64, pub voltage_ratio: String, pub efficiency_percent: f64 }
impl Transformer { pub fn new(id: String) -> Self { Self { id, rating_kva: 0.0, voltage_ratio: "11/0.4".into(), efficiency_percent: 98.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let t = Transformer::new("TR-1".into()); assert_eq!(t.id, "TR-1"); } }
