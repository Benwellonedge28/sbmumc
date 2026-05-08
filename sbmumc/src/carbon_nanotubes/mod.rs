//! Carbon Nanotubes Module (795)
use serde::{Deserialize, Serialize};
pub struct CarbonNanotube { pub id: String, pub type_: String, pub diameter_nm: f64, pub length_um: f64, pub conductivity: String }
impl CarbonNanotube { pub fn new(id: String) -> Self { Self { id, type_: "SWNT".into(), diameter_nm: 1.0, length_um: 10.0, conductivity: "Metallic".into() } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let c = CarbonNanotube::new("CNT-1".into()); assert_eq!(c.id, "CNT-1"); } }
