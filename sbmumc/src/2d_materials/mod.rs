//! 2D Materials Module (809)
use serde::{Deserialize, Serialize};
pub struct Material2D { pub id: String, pub material: String, pub layers: u8, pub bandgap_ev: f64, pub electron_mobility_cm2_vs: f64 }
impl Material2D { pub fn new(id: String) -> Self { Self { id, material: "MoS2".into(), layers: 1, bandgap_ev: 1.8, electron_mobility_cm2_vs: 200.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let m = Material2D::new("2DM-1".into()); assert_eq!(m.id, "2DM-1"); } }
