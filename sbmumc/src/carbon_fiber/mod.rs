//! Carbon Fiber Module (810)
use serde::{Deserialize, Serialize};
pub struct CarbonFiber { pub id: String, pub grade: String, pub tensile_strength_mpa: f64, pub modulus_gpa: f64, pub strain_percent: f64 }
impl CarbonFiber { pub fn new(id: String) -> Self { Self { id, grade: "T700".into(), tensile_strength_mpa: 4900.0, modulus_gpa: 230.0, strain_percent: 2.1 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let c = CarbonFiber::new("CF-1".into()); assert_eq!(c.id, "CF-1"); } }
