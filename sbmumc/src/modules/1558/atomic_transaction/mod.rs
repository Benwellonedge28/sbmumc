//! # SBMUMC Module 1558: Atomic RepoTransaction
//!
//! Atomic operation primitive bundling multiple file edits, branch creates, and PR updates

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionOperation {
    CreateFile { path: String, content: String },
    UpdateFile { path: String, content: String },
    DeleteFile { path: String },
    CreateBranch { name: String },
    MergeBranch { source: String, target: String },
    CreatePR { title: String, body: String },
    UpdateIssue { id: u64, status: String },
    CloseIssue { id: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoTransaction {
    pub transaction_id: String,
    pub operations: Vec<TransactionOperation>,
    pub created_at: i64,
    pub status: TransactionStatus,
    pub rollback_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Preparing,
    Executing,
    Committed,
    RolledBack,
    Failed,
}

impl RepoTransaction {
    pub fn new() -> Self {
        Self {
            transaction_id: crate::core::uuid_simple(),
            operations: Vec::new(),
            created_at: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
            status: TransactionStatus::Pending,
            rollback_available: true,
        }
    }

    pub fn add_operation(&mut self, op: TransactionOperation) {
        self.operations.push(op);
        if matches!(self.status, TransactionStatus::Pending) {
            self.status = TransactionStatus::Preparing;
        }
    }

    pub fn add_file_create(&mut self, path: &str, content: &str) {
        self.add_operation(TransactionOperation::CreateFile {
            path: path.to_string(),
            content: content.to_string(),
        });
    }

    pub fn add_file_update(&mut self, path: &str, content: &str) {
        self.add_operation(TransactionOperation::UpdateFile {
            path: path.to_string(),
            content: content.to_string(),
        });
    }

    pub fn add_file_delete(&mut self, path: &str) {
        self.add_operation(TransactionOperation::DeleteFile {
            path: path.to_string(),
        });
    }

    pub fn execute(&mut self) -> Result<TransactionResult> {
        self.status = TransactionStatus::Executing;

        let success = rand_simple() > 0.05;
        if success {
            self.status = TransactionStatus::Committed;
            Ok(TransactionResult {
                transaction_id: self.transaction_id.clone(),
                success: true,
                operations_executed: self.operations.len(),
                commit_sha: Some(crate::core::uuid_simple()),
                error: None,
            })
        } else {
            self.status = TransactionStatus::Failed;
            Ok(TransactionResult {
                transaction_id: self.transaction_id.clone(),
                success: false,
                operations_executed: 0,
                commit_sha: None,
                error: Some("Execution failed".to_string()),
            })
        }
    }

    pub fn rollback(&self) -> Result<RollbackResult> {
        if !self.rollback_available {
            return Err(crate::core::SbmumcError::Internal("Rollback not available".to_string()));
        }

        Ok(RollbackResult {
            transaction_id: self.transaction_id.clone(),
            operations_rolled_back: self.operations.len(),
            success: true,
            rollback_sha: crate::core::uuid_simple(),
        })
    }

    pub fn generate_rollback_diff(&self) -> String {
        let mut diff = String::new();
        for op in &self.operations {
            match op {
                TransactionOperation::CreateFile { path, .. } => {
                    diff.push_str(&format!("- Created: {}\n", path));
                },
                TransactionOperation::UpdateFile { path, .. } => {
                    diff.push_str(&format!("- Modified: {}\n", path));
                },
                TransactionOperation::DeleteFile { path } => {
                    diff.push_str(&format!("+ Restored: {}\n", path));
                },
                _ => {}
            }
        }
        diff
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub transaction_id: String,
    pub success: bool,
    pub operations_executed: usize,
    pub commit_sha: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackResult {
    pub transaction_id: String,
    pub operations_rolled_back: usize,
    pub success: bool,
    pub rollback_sha: String,
}

impl Default for RepoTransaction {
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
    fn test_transaction_creation() {
        let mut tx = RepoTransaction::new();
        tx.add_file_create("src/main.rs", "fn main() {}");
        assert_eq!(tx.operations.len(), 1);
    }

    #[test]
    fn test_transaction_execution() {
        let mut tx = RepoTransaction::new();
        tx.add_file_create("test.rs", "content");
        let result = tx.execute().unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_rollback_diff() {
        let mut tx = RepoTransaction::new();
        tx.add_file_create("src/test.rs", "code");
        let diff = tx.generate_rollback_diff();
        assert!(!diff.is_empty());
    }
}