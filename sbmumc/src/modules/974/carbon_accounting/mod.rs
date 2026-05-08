//! # SBMUMC Module 974: Carbon Accounting
//! 
//! Frameworks for comprehensive carbon accounting and reporting.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountingScope {
    Scope1,
    Scope2,
    Scope3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonAccount {
    pub account_id: String,
    pub scope: AccountingScope,
    pub entity_name: String,
    pub reporting_period: String,
    pub emissions_tco2e: f64,
    pub uncertainty: f64,
    pub verification_body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonBalance {
    pub balance_id: String,
    pub accounts: Vec<CarbonAccount>,
    pub total_emissions: f64,
    pub total_offsets: f64,
    pub net_emissions: f64,
    pub offset_credits: f64,
}

impl CarbonAccount {
    pub fn new(scope: AccountingScope, entity: &str, period: &str) -> Self {
        Self {
            account_id: format!("ca_{}", uuid_simple()),
            scope,
            entity_name: entity.to_string(),
            reporting_period: period.to_string(),
            emissions_tco2e: 0.0,
            uncertainty: 0.0,
            verification_body: None,
        }
    }

    pub fn record_emissions(&mut self, emissions: f64, uncertainty: f64) {
        self.emissions_tco2e = emissions;
        self.uncertainty = uncertainty.clamp(0.0, 1.0);
    }

    pub fn verified(&mut self, body: &str) {
        self.verification_body = Some(body.to_string());
    }
}

impl CarbonBalance {
    pub fn new() -> Self {
        Self {
            balance_id: format!("cb_{}", uuid_simple()),
            accounts: Vec::new(),
            total_emissions: 0.0,
            total_offsets: 0.0,
            net_emissions: 0.0,
            offset_credits: 0.0,
        }
    }

    pub fn add_account(&mut self, account: CarbonAccount) {
        self.accounts.push(account);
        self.compute_balance();
    }

    pub fn compute_balance(&mut self) {
        self.total_emissions = self.accounts.iter()
            .map(|a| a.emissions_tco2e)
            .sum();
        self.net_emissions = self.total_emissions - self.total_offsets;
    }

    pub fn add_offsets(&mut self, offsets: f64) {
        self.total_offsets = offsets;
        self.compute_balance();
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
    fn test_carbon_account() {
        let mut account = CarbonAccount::new(
            AccountingScope::Scope1,
            "Manufacturing Plant A",
            "2025",
        );
        account.record_emissions(5000.0, 0.05);
        assert!(account.emissions_tco2e > 0.0);
    }

    #[test]
    fn test_carbon_balance() {
        let mut balance = CarbonBalance::new();
        balance.add_account(CarbonAccount::new(AccountingScope::Scope2, "HQ", "2025"));
        balance.add_offsets(1000.0);
        assert!(balance.total_emissions >= 0.0);
    }
}
