//! Microgrid Module (771)
use serde::{Deserialize, Serialize};
pub struct Microgrid { pub id: String, pub capacity_kw: f64, pub island_capable: bool, pub renewable_percent: f64 }
impl Microgrid { pub fn new(id: String) -> Self { Self { id, capacity_kw: 0.0, island_capable: true, renewable_percent: 80.0 } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let m = Microgrid::new("MG-1".into()); assert_eq!(m.id, "MG-1"); } }
