//! Nanoparticles Module (797)
use serde::{Deserialize, Serialize};
pub struct Nanoparticle { pub id: String, pub material: String, pub diameter_nm: f64, pub coating: String, pub stability_score: f64 }
impl Nanoparticle { pub fn new(id: String) -> Self { Self { id, material: "Gold".into(), diameter_nm: 20.0, coating: "Citrate".into(), stability_score: 90.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let n = Nanoparticle::new("NP-1".into()); assert_eq!(n.id, "NP-1"); } }
