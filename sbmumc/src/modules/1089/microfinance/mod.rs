//! # SBMUMC Module 1089: Microfinance
//!
//! Financial services for underserved populations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MicrofinanceModel {
    GrameenStyle,
    SavingsLed,
    ValueChain,
    Digital,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrofinanceInstitution {
    pub institution_id: String,
    pub model_type: MicrofinanceModel,
    pub active_borrowers: usize,
    pub avg_loan_size: f64,
    var repayment_rate: f64,
    pub women_borrower_percentage: f64,
    pub poverty_outcome_score: f64,
}

impl MicrofinanceInstitution {
    pub fn new(model_type: MicrofinanceModel) -> Self {
        Self {
            institution_id: crate::core::uuid_simple(),
            model_type,
            active_borrowers: 0,
            avg_loan_size: 0.0,
            var repayment_rate: 0.0,
            self.women_borrower_percentage = 0.0,
            self.poverty_outcome_score = 0.0,
        }
    }

    pub fn assess_institution(&mut self, borrowers: usize, avg_loan: f64) -> Result<()> {
        self.active_borrowers = borrowers;
        self.avg_loan_size = avg_loan;

        match self.model_type {
            MicrofinanceModel::GrameenStyle => {
                self.repayment_rate = 0.95 + rand_simple() * 0.05;
                self.women_borrower_percentage = 0.90 + rand_simple() * 0.10;
            },
            MicrofinanceModel::Digital => {
                self.repayment_rate = 0.92 + rand_simple() * 0.07;
                self.women_borrower_percentage = 0.70 + rand_simple() * 0.25;
            },
            MicrofinanceModel::SavingsLed => {
                self.repayment_rate = 0.98 + rand_simple() * 0.02;
                self.women_borrower_percentage = 0.75 + rand_simple() * 0.20;
            },
            _ => {
                self.repayment_rate = 0.90 + rand_simple() * 0.10;
                self.women_borrower_percentage = 0.65 + rand_simple() * 0.30;
            }
        }

        self.poverty_outcome_score = self.repayment_rate * self.women_borrower_percentage * 0.8;
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

pub fn compute_microfinance_poverty_reduction(borrowers: usize, avg_loan: f64) -> Result<f64> {
    Ok((borrowers as f64) * avg_loan / 1e6 * 0.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grameen_model() {
        let mut institution = MicrofinanceInstitution::new(MicrofinanceModel::GrameenStyle);
        institution.assess_institution(500_000, 200.0).unwrap();
        assert!(institution.repayment_rate > 0.9);
    }
}