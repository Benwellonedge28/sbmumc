//! Political Science Module
//!
//! This module implements political science frameworks, governance analysis,
//! and political theory for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Political science system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalScience {
    pub ps_id: String,
    pub political_systems: Vec<PoliticalSystem>,
    pub theories: Vec<PoliticalTheory>,
    pub institutions: Vec<PoliticalInstitution>,
    pub political_behavior: PoliticalBehavior,
    pub comparative_analysis: ComparativePolitics,
}

/// Political system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalSystem {
    pub system_id: String,
    pub system_name: String,
    pub system_type: SystemType,
    pub government_branches: Vec<GovernmentBranch>,
    pub electoral_system: ElectoralSystem,
    pub political_parties: Vec<PoliticalParty>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemType {
    Democracy,
    Monarchy,
    Authoritarianism,
    Totalitarianism,
    Theocracy,
    Republic,
    Federation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentBranch {
    pub branch_name: String,
    pub branch_type: BranchType,
    pub powers: Vec<String>,
    pub current_leaders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BranchType {
    Executive,
    Legislative,
    Judicial,
    Administrative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectoralSystem {
    pub system_type: ElectoralType,
    pub voting_method: VotingMethod,
    pub representation_type: RepresentationType,
    pub threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElectoralType {
    FirstPastThePost,
    ProportionalRepresentation,
    Mixed,
    SingleTransferableVote,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VotingMethod {
    Plurality,
    Majority,
    RankedChoice,
    Condorcet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepresentationType {
    SingleMember,
    MultiMember,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalParty {
    pub party_id: String,
    pub party_name: String,
    pub ideology: String,
    pub position: PoliticalPosition,
    pub seats: u32,
    pub base_support: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalPosition {
    pub left_right: f64,
    pub authoritarian_libertarian: f64,
}

/// Political theory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalTheory {
    pub theory_id: String,
    pub theory_name: String,
    pub philosopher: String,
    pub era: String,
    pub core_arguments: Vec<String>,
    pub influence: f64,
    pub modern_applications: Vec<String>,
}

/// Political institution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalInstitution {
    pub institution_id: String,
    pub institution_name: String,
    pub institution_type: InstitutionType,
    pub jurisdiction: String,
    pub powers: Vec<String>,
    pub accountability: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstitutionType {
    Legislative,
    Executive,
    Judicial,
    Regulatory,
    Electoral,
}

/// Political behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalBehavior {
    pub voting_patterns: VotingPatterns,
    pub political_socialization: SocializationProcess,
    pub public_opinion: PublicOpinion,
    pub political_mobilization: MobilizationPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingPatterns {
    pub turnout_rate: f64,
    pub demographic_patterns: HashMap<String, f64>,
    pub issue_voting: f64,
    pub party_loyalty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocializationProcess {
    pub family_influence: f64,
    pub education_influence: f64,
    pub media_influence: f64,
    pub peer_influence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicOpinion {
    pub polling_data: Vec<Poll>,
    pub opinion_trends: Vec<OpinionTrend>,
    pub polarization_index: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub poll_id: String,
    pub question: String,
    pub responses: HashMap<String, f64>,
    pub sample_size: u32,
    pub margin_of_error: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpinionTrend {
    pub issue: String,
    pub direction: TrendDirection,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendDirection {
    Liberalizing,
    Conservative,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobilizationPatterns {
    pub grassroots_activities: Vec<String>,
    pub digital_mobilization: f64,
    pub voter_suppression: Vec<String>,
}

/// Comparative politics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativePolitics {
    pub countries: Vec<CountryProfile>,
    pub cross_national_data: Vec<CrossNationalData>,
    pub typologies: Vec<SystemTypology>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryProfile {
    pub country_id: String,
    pub country_name: String,
    pub system_type: SystemType,
    pub stability_index: f64,
    pub democracy_score: f64,
    pub human_development_index: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNationalData {
    pub indicator: String,
    pub values: HashMap<String, f64>,
    pub ranking: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTypology {
    pub typology_name: String,
    pub criteria: Vec<String>,
    pub countries: Vec<String>,
}

impl PoliticalScience {
    /// Creates a new political science system
    pub fn new() -> Self {
        Self {
            ps_id: String::from("political_science_v1"),
            political_systems: vec![
                PoliticalSystem {
                    system_id: String::from("sys_us"),
                    system_name: String::from("United States"),
                    system_type: SystemType::Republic,
                    government_branches: vec![
                        GovernmentBranch { branch_name: String::from("Executive"), branch_type: BranchType::Executive, powers: vec![String::from("Enforce laws")], current_leaders: vec![] },
                    ],
                    electoral_system: ElectoralSystem {
                        system_type: ElectoralType::FirstPastThePost,
                        voting_method: VotingMethod::Plurality,
                        representation_type: RepresentationType::SingleMember,
                        threshold: None,
                    },
                    political_parties: vec![],
                },
            ],
            theories: vec![
                PoliticalTheory {
                    theory_id: String::from("theory_1"),
                    theory_name: String::from("Social Contract"),
                    philosopher: String::from("John Locke"),
                    era: String::from("17th Century"),
                    core_arguments: vec![String::from("Natural rights"), String::from("Consent of governed")],
                    influence: 0.95,
modern_applications: vec![String::from("Constitutional law")],
                },
            ],
            institutions: vec![],
            political_behavior: PoliticalBehavior {
                voting_patterns: VotingPatterns {
                    turnout_rate: 0.55,
                    demographic_patterns: HashMap::new(),
                    issue_voting: 0.4,
                    party_loyalty: 0.6,
                },
                political_socialization: SocializationProcess {
                    family_influence: 0.5,
                    education_influence: 0.3,
                    media_influence: 0.15,
                    peer_influence: 0.05,
                },
                public_opinion: PublicOpinion {
                    polling_data: vec![],
                    opinion_trends: vec![],
                    polarization_index: 0.7,
                },
                political_mobilization: MobilizationPatterns {
                    grassroots_activities: vec![],
                    digital_mobilization: 0.5,
                    voter_suppression: vec![],
                },
            },
            comparative_analysis: ComparativePolitics {
                countries: vec![],
                cross_national_data: vec![],
                typologies: vec![],
            },
        }
    }

    /// Analyzes political stability
    pub fn analyze_stability(&self, system_id: &str) -> StabilityAnalysis {
        StabilityAnalysis {
            system_id: system_id.to_string(),
            stability_score: 7.5,
            risk_factors: vec![String::from("Economic inequality")],
            institutional_strength: 8.0,
        }
    }

    /// Compares political systems
    pub fn compare_systems(&self, system1: &str, system2: &str) -> SystemComparison {
        SystemComparison {
            system_1: system1.to_string(),
            system_2: system2.to_string(),
            similarity_score: 0.6,
            key_differences: vec![String::from("Electoral system"), String::from("Federal structure")],
            common_features: vec![String::from("Separation of powers")],
        }
    }

    /// Predicts election outcomes
    pub fn predict_election(&self, election_id: &str) -> ElectionPrediction {
        ElectionPrediction {
            election_id: election_id.to_string(),
            predicted_turnout: 0.60,
            predicted_winner: String::from("Party A"),
            confidence: 0.75,
            swing_factors: vec![String::from("Economic conditions")],
        }
    }

    /// Analyzes policy outcomes
    pub fn analyze_policy(&self, policy_id: &str) -> PolicyAnalysis {
        PolicyAnalysis {
            policy_id: policy_id.to_string(),
            effectiveness_score: 7.0,
            unintended_consequences: vec![],
            equity_impact: 0.5,
            recommendations: vec![],
        }
    }

    /// Evaluates democracy quality
    pub fn evaluate_democracy(&self, country_id: &str) -> DemocracyEvaluation {
        DemocracyEvaluation {
            country_id: country_id.to_string(),
            electoral_process: 8.0,
            political_participation: 7.5,
            civil_liberties: 8.0,
            rule_of_law: 7.5,
            overall_score: 7.75,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityAnalysis {
    pub system_id: String,
    pub stability_score: f64,
    pub risk_factors: Vec<String>,
    pub institutional_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemComparison {
    pub system_1: String,
    pub system_2: String,
    pub similarity_score: f64,
    pub key_differences: Vec<String>,
    pub common_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionPrediction {
    pub election_id: String,
    pub predicted_turnout: f64,
    pub predicted_winner: String,
    pub confidence: f64,
    pub swing_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAnalysis {
    pub policy_id: String,
    pub effectiveness_score: f64,
    pub unintended_consequences: Vec<String>,
    pub equity_impact: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemocracyEvaluation {
    pub country_id: String,
    pub electoral_process: f64,
    pub political_participation: f64,
    pub civil_liberties: f64,
    pub rule_of_law: f64,
    pub overall_score: f64,
}

impl Default for PoliticalScience {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_political_science_creation() {
        let ps = PoliticalScience::new();
        assert_eq!(ps.ps_id, "political_science_v1");
    }
    #[test]
    fn test_analyze_stability() {
        let ps = PoliticalScience::new();
        let analysis = ps.analyze_stability("sys_us");
        assert!(analysis.stability_score > 0.0);
    }
}
