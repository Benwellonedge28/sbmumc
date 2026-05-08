//! Carbon Storage Module (764)
//!
//! CO2 storage, geological sequestration, and carbon reservoirs.

use serde::{Deserialize, Serialize};

pub struct CarbonStorage {
    pub storage_id: String,
    pub storage_type: String,
    pub capacity_mt: f64,
    pub security_percent: f64,
}

impl CarbonStorage {
    pub fn new(id: String) -> Self { Self { storage_id: id, storage_type: "Saline".into(), capacity_mt: 0.0, security_percent: 99.0 } }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn test_cs() { let s = CarbonStorage::new("CS-1".into()); assert_eq!(s.storage_id, "CS-1"); } }
