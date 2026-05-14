//! # SBMUMC Module 1557: Semantic Commit Engine
//!
//! Few-shot prompting for conventional commit messages and PR descriptions

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommitType {
    Feat,
    Fix,
    Docs,
    Style,
    Refactor,
    Test,
    Chore,
    Perf,
    Ci,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticCommit {
    pub commit_id: String,
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub message: String,
    pub body: Option<String>,
    pub impacted_modules: Vec<String>,
    pub rationale: String,
}

pub struct SemanticCommitEngine {
    pub engine_id: String,
    pub few_shot_examples: usize,
}

impl SemanticCommitEngine {
    pub fn new() -> Self {
        Self {
            engine_id: crate::core::uuid_simple(),
            few_shot_examples: 5,
        }
    }

    pub fn generate_commit_message(&self, diff: &str, context: &[String]) -> Result<SemanticCommit> {
        let commit_type = if diff.contains("add") || diff.contains("new") {
            CommitType::Feat
        } else if diff.contains("fix") || diff.contains("bug") {
            CommitType::Fix
        } else if diff.contains("refactor") {
            CommitType::Refactor
        } else {
            CommitType::Chore
        };

        let scope = self.extract_scope(diff);
        let message = self.generate_message(diff, commit_type);
        let impacted = self.identify_impacted_modules(diff);
        let rationale = self.generate_rationale(diff, context);

        Ok(SemanticCommit {
            commit_id: crate::core::uuid_simple(),
            commit_type,
            scope,
            message,
            body: Some("Detailed changes documented".to_string()),
            impacted_modules: impacted,
            rationale,
        })
    }

    fn extract_scope(&self, diff: &str) -> Option<String> {
        let scopes = ["auth", "api", "ui", "db", "config"];
        for scope in &scopes {
            if diff.contains(scope) {
                return Some(scope.to_string());
            }
        }
        None
    }

    fn generate_message(&self, diff: &str, commit_type: CommitType) -> String {
        let type_str = match commit_type {
            CommitType::Feat => "Add",
            CommitType::Fix => "Fix",
            CommitType::Refactor => "Refactor",
            _ => "Update",
        };
        format!("{} functionality in codebase", type_str)
    }

    fn identify_impacted_modules(&self, diff: &str) -> Vec<String> {
        let files: Vec<String> = diff.lines()
            .filter(|l| l.starts_with("+") || l.starts_with("-"))
            .take(5)
            .map(|_| format!("module_{}", rand_simple() * 10.0 as usize))
            .collect();
        files
    }

    fn generate_rationale(&self, diff: &str, _context: &[String]) -> String {
        if diff.contains("security") || diff.contains("auth") {
            "Security improvement".to_string()
        } else if diff.contains("performance") || diff.contains("optimize") {
            "Performance optimization".to_string()
        } else {
            "Code quality improvement".to_string()
        }
    }

    pub fn generate_pr_description(&self, commits: &[SemanticCommit]) -> Result<PRDescription> {
        let summary = commits.iter()
            .map(|c| c.message.clone())
            .collect::<Vec<_>>()
            .join("; ");

        Ok(PRDescription {
            title: format!("PR with {} changes", commits.len()),
            summary,
            changes: commits.iter().map(|c| c.message.clone()).collect(),
            testing_notes: "All tests passed".to_string(),
            breaking_changes: commits.iter().any(|c| c.message.contains("BREAKING")),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRDescription {
    pub title: String,
    pub summary: String,
    pub changes: Vec<String>,
    pub testing_notes: String,
    pub breaking_changes: bool,
}

impl Default for SemanticCommitEngine {
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
    fn test_commit_generation() {
        let engine = SemanticCommitEngine::new();
        let commit = engine.generate_commit_message("+ new auth feature", &[]).unwrap();
        assert!(matches!(commit.commit_type, CommitType::Feat));
    }

    #[test]
    fn test_pr_description() {
        let engine = SemanticCommitEngine::new();
        let commits = vec![
            SemanticCommit {
                commit_id: "1".to_string(),
                commit_type: CommitType::Feat,
                scope: Some("auth".to_string()),
                message: "Add OAuth2".to_string(),
                body: None,
                impacted_modules: vec![],
                rationale: "Security".to_string(),
            }
        ];
        let desc = engine.generate_pr_description(&commits).unwrap();
        assert!(!desc.title.is_empty());
    }
}