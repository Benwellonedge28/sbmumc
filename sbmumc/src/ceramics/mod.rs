//! Ceramics Module (802)
use serde::{Deserialize, Serialize};
pub struct Ceramic { pub id: String, pub type_: String, pub hardness_hv: f64, pub toughness_mpa_sqrtm: f64, pub max_temp_c: f64 }
impl Ceramic { pub fn new(id: String) -> Self { Self { id, type_: "Alumina".into(), hardness_hv: 1800.0, toughness_mpa_sqrtm: 4.0, max_temp_c: 1500.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let c = Ceramic::new("CE-1".into()); assert_eq!(c.id, "CE-1"); } }
