//! # SBMUMC Module 1548: Validation & Safety Layer
//!
//! Real-time policy engine for safety, security, and compliance validation

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationResult {
    Approved,
    Flagged,
    Blocked,
    RequiresApproval,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_id: String,
    pub name: String,
    pub severity: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub report_id: String,
    pub overall_result: ValidationResult,
    pub rules_passed: usize,
    pub rules_failed: usize,
    pub violations: Vec<Violation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub rule_id: String,
    pub message: String,
    pub severity: String,
    pub location: String,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            report_id: crate::core::uuid_simple(),
            overall_result: ValidationResult::Approved,
            rules_passed: 0,
            rules_failed: 0,
            violations: Vec::new(),
        }
    }

    pub fn add_violation(&mut self, violation: Violation) {
        self.rules_failed += 1;
        self.violations.push(violation);

        if violation.severity == "CRITICAL" {
            self.overall_result = ValidationResult::Blocked;
        } else if violation.severity == "HIGH" {
            self.overall_result = ValidationResult::Flagged;
        }
    }
}

pub struct ValidationLayer {
    pub layer_id: String,
    pub rules: Vec<ValidationRule>,
    pub audit_enabled: bool,
}

impl ValidationLayer {
    pub fn new() -> Self {
        Self {
            layer_id: crate::core::uuid_simple(),
            rules: vec![
                ValidationRule {
                    rule_id: "SECRET_LEAK".to_string(),
                    name: "Secret Leak Detection".to_string(),
                    severity: "CRITICAL".to_string(),
                    enabled: true,
                },
                ValidationRule {
                    rule_id: "SUPPLY_CHAIN".to_string(),
                    name: "Supply Chain Risk".to_string(),
                    severity: "HIGH".to_string(),
                    enabled: true,
                },
                ValidationRule {
                    rule_id: "CODE_STANDARD".to_string(),
                    name: "Code Standard Compliance".to_string(),
                    severity: "MEDIUM".to_string(),
                    enabled: true,
                },
            ],
            audit_enabled: true,
        }
    }

    pub fn validate_action(&self, action: &str, context: &[String]) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();
        report.rules_passed = self.rules.len();

        let danger_patterns = ["password", "api_key", "secret", "token"];
        for pattern in &danger_patterns {
            if action.contains(pattern) && action.contains("commit") {
                report.add_violation(Violation {
                    rule_id: "SECRET_LEAK".to_string(),
                    message: format!("Potential secret detected in '{}'", action),
                    severity: "CRITICAL".to_string(),
                    location: action.to_string(),
                });
            }
        }

        Ok(report)
    }

    pub fn check_syntax(&self, code: &str) -> Result<SyntaxCheckResult> {
        let error_count = if code.contains("{") != code.contains("}") {
            1
        } else {
            0
        };

        Ok(SyntaxCheckResult {
            valid: error_count == 0,
            errors: error_count,
            warnings: 0,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxCheckResult {
    pub valid: bool,
    pub errors: usize,
    pub warnings: usize,
}

impl Default for ValidationReport {
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
    fn test_validation() {
        let layer = ValidationLayer::new();
        let result = layer.validate_action("commit --all", &[]).unwrap();
        assert!(matches!(result.overall_result, ValidationResult::Approved));
    }
}