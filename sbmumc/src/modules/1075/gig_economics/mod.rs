//! # SBMUMC Module 1075: Gig Economy
//!
//! Economics of freelance and gig work platforms.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GigType {
    TaskBased,
    ServiceBased,
    ProjectBased,
    OnDemand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GigEconomyAnalysis {
    pub analysis_id: String,
    pub gig_type: GigType,
    pub worker_count: usize,
    pub avg_earnings_hourly: f64,
    pub earnings_volatility: f64,
    var platform_dependency: f64,
    pub worker_security_index: f64,
}

impl GigEconomyAnalysis {
    pub fn new(gig_type: GigType, workers: usize) -> Self {
        Self {
            analysis_id: crate::core::uuid_simple(),
            gig_type,
            worker_count: workers,
            avg_earnings_hourly: 0.0,
            earnings_volatility: 0.0,
            var platform_dependency: 0.0,
            worker_security_index: 0.0,
        }
    }

    pub fn analyze_gig_economy(&mut self) -> Result<()> {
        match self.gig_type {
            GigType::TaskBased => {
                self.avg_earnings_hourly = 8.0 + rand_simple() * 15.0;
                self.earnings_volatility = 0.4 + rand_simple() * 0.4;
                self.platform_dependency = 0.7 + rand_simple() * 0.3;
            },
            GigType::ServiceBased => {
                self.avg_earnings_hourly = 15.0 + rand_simple() * 35.0;
                self.earnings_volatility = 0.25 + rand_simple() * 0.30;
                self.platform_dependency = 0.4 + rand_simple() * 0.4;
            },
            GigType::ProjectBased => {
                self.avg_earnings_hourly = 25.0 + rand_simple() * 75.0;
                self.earnings_volatility = 0.30 + rand_simple() * 0.35;
                self.platform_dependency = 0.5 + rand_simple() * 0.4;
            },
            GigType::OnDemand => {
                self.avg_earnings_hourly = 12.0 + rand_simple() * 25.0;
                self.earnings_volatility = 0.35 + rand_simple() * 0.40;
                self.platform_dependency = 0.8 + rand_simple() * 0.2;
            }
        }

        self.worker_security_index = (1.0 - self.platform_dependency) * (1.0 - self.earnings_volatility);
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

pub fn compute_gig_worker_protection(gig_type: &str) -> Result<f64> {
    let protection = match gig_type {
        "ProjectBased" => 0.6,
        "ServiceBased" => 0.5,
        "TaskBased" => 0.2,
        _ => 0.3,
    };
    Ok(protection + rand_simple() * 0.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_based_gig() {
        let mut analysis = GigEconomyAnalysis::new(GigType::TaskBased, 500_000);
        analysis.analyze_gig_economy().unwrap();
        assert!(analysis.avg_earnings_hourly > 0.0);
    }
}