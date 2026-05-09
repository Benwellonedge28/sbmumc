//! # SBMUMC Module 1090: Community Finance
//!
//! Local financial institutions and community-based finance.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunityFinanceType {
    CreditUnion,
    CommunityDevelopmentFinancialInstitution,
    CommunityBank,
    LocalDevelopmentTrust,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityFinanceInstitution {
    pub institution_id: String,
    pub finance_type: CommunityFinanceType,
    pub service_area: String,
    pub local_investment_ratio: f64,
    var community_ownership_score: f64,
    pub local_economic_impact: f64,
    pub financial_inclusion_reach: f64,
}

impl CommunityFinanceInstitution {
    pub fn new(finance_type: CommunityFinanceType, area: String) -> Self {
        Self {
            institution_id: crate::core::uuid_simple(),
            finance_type,
            service_area: area,
            local_investment_ratio: 0.0,
            var community_ownership_score: 0.0,
            self.local_economic_impact = 0.0,
            self.financial_inclusion_reach = 0.0,
        }
    }

    pub fn assess_institution(&mut self) -> Result<()> {
        match self.finance_type {
            CommunityFinanceType::CreditUnion => {
                self.local_investment_ratio = 0.80 + rand_simple() * 0.20;
                self.community_ownership_score = 0.90 + rand_simple() * 0.10;
            },
            CommunityFinanceType::CDFI => {
                self.local_investment_ratio = 0.85 + rand_simple() * 0.15;
                self.community_ownership_score = 0.75 + rand_simple() * 0.20;
            },
            CommunityFinanceType::CommunityBank => {
                self.local_investment_ratio = 0.70 + rand_simple() * 0.25;
                self.community_ownership_score = 0.60 + rand_simple() * 0.30;
            },
            _ => {
                self.local_investment_ratio = 0.75 + rand_simple() * 0.20;
                self.community_ownership_score = 0.70 + rand_simple() * 0.25;
            }
        }

        self.local_economic_impact = self.local_investment_ratio * self.community_ownership_score;
        self.financial_inclusion_reach = self.local_investment_ratio * 0.8;
        Ok(())
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

pub fn compute_community_finance_impact(institution_id: &str) -> Result<f64> {
    Ok(0.6 + rand_simple() * 0.4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_union() {
        let mut institution = CommunityFinanceInstitution::new(
            CommunityFinanceType::CreditUnion,
            "Rural_Appalachia".to_string(),
        );
        institution.assess_institution().unwrap();
        assert!(institution.local_investment_ratio > 0.7);
    }
}