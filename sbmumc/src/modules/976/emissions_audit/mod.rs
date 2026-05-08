//! # SBMUMC Module 976: Emissions Audit
//! 
//! Frameworks for auditing and verifying emissions data.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditStandard {
    ISO14064,
    GHGProtocol,
    CDP,
    TCFD,
    EUETS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionsAudit {
    pub audit_id: String,
    pub entity_name: String,
    pub audit_standard: AuditStandard,
    pub audit_period: String,
    pub scope_covered: Vec<AccountingScope>,
    pub findings: Vec<String>,
    pub opinion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub report_id: String,
    pub audits: Vec<EmissionsAudit>,
    pub total_findings: u32,
    pub major_findings: u32,
    pub compliance_rate: f64,
}

impl EmissionsAudit {
    pub fn new(entity: &str, standard: AuditStandard, period: &str) -> Self {
        Self {
            audit_id: format!("ea_{}", uuid_simple()),
            entity_name: entity.to_string(),
            audit_standard: standard,
            audit_period: period.to_string(),
            scope_covered: Vec::new(),
            findings: Vec::new(),
            opinion: "Unqualified".to_string(),
        }
    }

    pub fn add_scope(&mut self, scope: AccountingScope) {
        self.scope_covered.push(scope);
    }

    pub fn add_finding(&mut self, finding: &str) {
        self.findings.push(finding.to_string());
    }

    pub fn issue_opinion(&mut self, opinion: &str) {
        self.opinion = opinion.to_string();
    }
}

impl AuditReport {
    pub fn new() -> Self {
        Self {
            report_id: format!("ar_{}", uuid_simple()),
            audits: Vec::new(),
            total_findings: 0,
            major_findings: 0,
            compliance_rate: 0.0,
        }
    }

    pub fn add_audit(&mut self, audit: EmissionsAudit) {
        self.audits.push(audit);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_findings = self.audits.iter()
            .map(|a| a.findings.len() as u32)
            .sum();
        self.major_findings = self.audits.iter()
            .filter(|a| a.opinion != "Unqualified")
            .count() as u32;
        let compliant = self.audits.iter()
            .filter(|a| a.opinion == "Unqualified")
            .count() as f64;
        self.compliance_rate = if self.audits.is_empty() {
            0.0
        } else {
            compliant / self.audits.len() as f64
        };
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emissions_audit() {
        let mut audit = EmissionsAudit::new(
            "Steel Company A",
            AuditStandard::GHGProtocol,
            "2025",
        );
        audit.add_scope(AccountingScope::Scope1);
        audit.add_finding("Minor data quality issues");
        assert!(audit.findings.len() == 1);
    }

    #[test]
    fn test_audit_report() {
        let mut report = AuditReport::new();
        report.add_audit(EmissionsAudit::new(
            "Cement Plant",
            AuditStandard::ISO14064,
            "2024",
        ));
        assert!(report.compliance_rate >= 0.0);
    }
}
