//! Composites Module (801)
use serde::{Deserialize, Serialize};
pub struct Composite { pub id: String, pub matrix: String, pub reinforcement: String, pub fiber_volume_percent: f64, pub strength_to_weight_ratio: f64 }
impl Composite { pub fn new(id: String) -> Self { Self { id, matrix: "Polymer".into(), reinforcement: "Carbon Fiber".into(), fiber_volume_percent: 60.0, strength_to_weight_ratio: 20.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let c = Composite::new("CO-1".into()); assert_eq!(c.id, "CO-1"); } }
