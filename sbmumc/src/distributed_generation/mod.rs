//! Distributed Generation Module (789)
use serde::{Deserialize, Serialize};
pub struct DistributedGen { pub id: String, pub capacity_kw: f64, pub technology: String, pub grid_connection: String }
impl DistributedGen { pub fn new(id: String) -> Self { Self { id, capacity_kw: 0.0, technology: "Solar PV".into(), grid_connection: "Grid-Tied".into() } } }
#[cfg(test)] mod tests { use super::*; #[test] fn test() { let d = DistributedGen::new("DG-1".into()); assert_eq!(d.id, "DG-1"); } }
