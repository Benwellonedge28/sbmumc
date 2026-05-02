//! Virtual Union Module (636)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualUnion {
    pub vu_id: String,
    pub member_nations: u32,
    pub union_strength: f64,
}

impl VirtualUnion {
    pub fn new() -> Self {
        Self {
            vu_id: String::from("virtual_union_v1"),
            member_nations: 100,
            union_strength: 0.98,
        }
    }
}

impl Default for VirtualUnion {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_union() {
        let u = VirtualUnion::new();
        assert!(u.member_nations > 0);
    }
}
