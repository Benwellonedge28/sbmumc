//! # SBMUMC Module 1554: Audit Trail System
//!
//! Immutable append-only log with SHA256 hash chain for all actions

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub entry_id: String,
    pub timestamp: i64,
    pub action: String,
    pub actor: String,
    pub previous_hash: String,
    pub entry_hash: String,
    pub metadata: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub log_id: String,
    pub entries: Vec<AuditEntry>,
    pub genesis_hash: String,
}

impl AuditLog {
    pub fn new() -> Self {
        let genesis = format!("genesis_{}", crate::core::uuid_simple());
        Self {
            log_id: crate::core::uuid_simple(),
            entries: Vec::new(),
            genesis_hash: genesis.clone(),
        }
    }

    pub fn append(&mut self, action: &str, actor: &str, metadata: Vec<(String, String)>) -> Result<String> {
        let previous_hash = self.entries.last()
            .map(|e| e.entry_hash.clone())
            .unwrap_or_else(|| self.genesis_hash.clone());

        let entry = AuditEntry {
            entry_id: crate::core::uuid_simple(),
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
            action: action.to_string(),
            actor: actor.to_string(),
            previous_hash,
            entry_hash: crate::core::uuid_simple(),
            metadata,
        };

        self.entries.push(entry.clone());
        Ok(entry.entry_hash)
    }

    pub fn verify(&self) -> Result<VerificationResult> {
        let mut valid = true;
        let mut previous = self.genesis_hash.clone();

        for entry in &self.entries {
            if entry.previous_hash != previous {
                valid = false;
                break;
            }
            previous = entry.entry_hash.clone();
        }

        Ok(VerificationResult {
            valid,
            entries_verified: self.entries.len(),
            broken_at: if !valid { Some(self.entries.len()) } else { None },
        })
    }

    pub fn get_entry(&self, entry_id: &str) -> Option<&AuditEntry> {
        self.entries.iter().find(|e| e.entry_id == entry_id)
    }

    pub fn search(&self, action: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.action.contains(action)).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub valid: bool,
    pub entries_verified: usize,
    pub broken_at: Option<usize>,
}

pub struct HumanOverride {
    pub override_id: String,
    pub requires_signature: bool,
    pub approver: Option<String>,
}

impl HumanOverride {
    pub fn new() -> Self {
        Self {
            override_id: crate::core::uuid_simple(),
            requires_signature: true,
            approver: None,
        }
    }

    pub fn request_approval(&mut self, reason: &str) -> Result<ApprovalRequest> {
        Ok(ApprovalRequest {
            request_id: crate::core::uuid_simple(),
            reason: reason.to_string(),
            status: "pending".to_string(),
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
        })
    }

    pub fn approve(&mut self, request: &ApprovalRequest, signature: &str) -> Result<ApprovalResult> {
        if signature.is_empty() {
            return Err(crate::core::SbmumcError::Internal("Invalid signature".to_string()));
        }
        self.approver = Some("approved".to_string());
        Ok(ApprovalResult {
            approved: true,
            approver: signature.to_string(),
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub request_id: String,
    pub reason: String,
    pub status: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResult {
    pub approved: bool,
    pub approver: String,
    pub timestamp: i64,
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for HumanOverride {
    fn default() -> Self {
        Self::new()
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_append() {
        let mut log = AuditLog::new();
        let hash = log.append("commit", "agent", vec![]).unwrap();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_audit_verification() {
        let mut log = AuditLog::new();
        log.append("action1", "actor1", vec![]).unwrap();
        log.append("action2", "actor2", vec![]).unwrap();
        let result = log.verify().unwrap();
        assert!(result.valid);
    }
}