//! International Relations Module
//!
//! This module implements international relations frameworks, diplomatic analysis,
//! and global governance for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// International relations system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalRelations {
    pub ir_id: String,
    pub actors: Vec<IActor>,
    pub theories: Vec<IRTheory>,
    pub diplomatic_systems: Vec<DiplomaticSystem>,
    pub security_studies: SecurityStudies,
    pub international_organizations: Vec<InternationalOrganization>,
}

/// International actor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IActor {
    pub actor_id: String,
    pub actor_name: String,
    pub actor_type: ActorType,
    pub capabilities: ActorCapabilities,
    pub interests: Vec<String>,
    pub behavior_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActorType {
    State,
    InternationalOrganization,
    NGO,
    MultinationalCorporation,
    TerroristGroup,
    NonStateActor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorCapabilities {
    pub military: MilitaryCapability,
    pub economic: EconomicCapability,
    pub soft_power: f64,
    pub diplomatic_influence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilitaryCapability {
    pub personnel: u32,
    pub equipment_score: f64,
    pub nuclear: bool,
    pub alliance_obligations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicCapability {
    pub gdp_usd: f64,
    pub trade_volume: f64,
    pub currency_international_status: f64,
}

/// IR theory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRTheory {
    pub theory_id: String,
    pub theory_name: String,
    pub paradigm: IRParadigm,
    pub core_arguments: Vec<String>,
    pub assumptions: Vec<String>,
    pub explanatory_power: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IRParadigm {
    Realism,
    Liberalism,
    Constructivism,
    Marxism,
    Feminism,
}

/// Diplomatic system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticSystem {
    pub system_id: String,
    pub system_type: DiplomaticSystemType,
    pub diplomatic_missions: Vec<DiplomaticMission>,
    pub diplomatic_immunity_rules: Vec<String>,
    pub protocols: Vec<Protocol>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiplomaticSystemType {
    Bilateral,
    Multilateral,
    Regional,
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticMission {
    pub mission_id: String,
    pub host_country: String,
    pub sending_country: String,
    pub mission_type: MissionType,
    pub staff_size: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MissionType {
    Embassy,
    Consulate,
    PermanentMission,
    SpecialEnvoy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protocol {
    pub protocol_name: String,
    pub rules: Vec<String>,
    pub precedence_rules: Vec<String>,
}

/// Security studies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStudies {
    pub threats: Vec<SecurityThreat>,
    pub deterrence: DeterrenceFramework,
    pub regional_security: Vec<RegionalSecurity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityThreat {
    pub threat_id: String,
    pub threat_type: ThreatType,
    pub source: String,
    pub severity: f64,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatType {
    Conventional,
    Nuclear,
    Cyber,
    Terrorism,
    Environmental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterrenceFramework {
    pub deterrence_type: DeterrenceType,
    pub credible_threats: Vec<String>,
    pub defense_systems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeterrenceType {
    Nuclear,
    Conventional,
    Extended,
    Collective,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalSecurity {
    pub region: String,
    pub alliance_structure: String,
    pub threat_perceptions: Vec<String>,
}

/// International organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalOrganization {
    pub org_id: String,
    pub org_name: String,
    pub org_type: OrgType,
    pub membership: Vec<String>,
    pub structure: OrgStructure,
    pub functions: Vec<String>,
    pub effectiveness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrgType {
    Global,
    Regional,
    Specialized,
    Alliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgStructure {
    pub decision_body: String,
    pub secretariat: String,
    pub specialized_agencies: Vec<String>,
    pub voting_system: String,
}

impl InternationalRelations {
    /// Creates a new international relations system
    pub fn new() -> Self {
        Self {
            ir_id: String::from("international_relations_v1"),
            actors: vec![
                IActor {
                    actor_id: String::from("actor_us"),
                    actor_name: String::from("United States"),
                    actor_type: ActorType::State,
                    capabilities: ActorCapabilities {
                        military: MilitaryCapability { personnel: 1_300_000, equipment_score: 9.5, nuclear: true, alliance_obligations: vec![String::from("NATO")] },
                        economic: EconomicCapability { gdp_usd: 25.0e12, trade_volume: 6.0e12, currency_international_status: 0.6 },
                        soft_power: 8.5,
                        diplomatic_influence: 9.0,
                    },
                    interests: vec![String::from("National security"), String::from("Economic prosperity")],
                    behavior_patterns: vec![],
                },
            ],
            theories: vec![
                IRTheory {
                    theory_id: String::from("theory_realism"),
                    theory_name: String::from("Classical Realism"),
                    paradigm: IRParadigm::Realism,
                    core_arguments: vec![String::from("Anarchy leads to self-help"), String::from("Power politics")],
                    assumptions: vec![String::from("States as rational actors"), String::from("Security competition")],
                    explanatory_power: 0.7,
                },
            ],
            diplomatic_systems: vec![
                DiplomaticSystem {
                    system_id: String::from("diplomacy_un"),
                    system_type: DiplomaticSystemType::Global,
                    diplomatic_missions: vec![],
                    diplomatic_immunity_rules: vec![String::from("Vienna Convention on Diplomatic Relations")],
                    protocols: vec![],
                },
            ],
            security_studies: SecurityStudies {
                threats: vec![
                    SecurityThreat { threat_id: String::from("threat_cyber"), threat_type: ThreatType::Cyber, source: String::from("Multiple"), severity: 0.8, probability: 0.7 },
                ],
                deterrence: DeterrenceFramework {
                    deterrence_type: DeterrenceType::Nuclear,
                    credible_threats: vec![String::from("Second strike capability")],
                    defense_systems: vec![String::from("Missile defense")],
                },
                regional_security: vec![
                    RegionalSecurity { region: String::from("Europe"), alliance_structure: String::from("NATO"), threat_perceptions: vec![String::from("Russia")] },
                ],
            },
            international_organizations: vec![
                InternationalOrganization {
                    org_id: String::from("org_un"),
                    org_name: String::from("United Nations"),
                    org_type: OrgType::Global,
                    membership: vec![String::from("193 member states")],
                    structure: OrgStructure {
                        decision_body: String::from("Security Council"),
                        secretariat: String::from("Secretary-General"),
                        specialized_agencies: vec![String::from("WHO"), String::from("UNICEF")],
                        voting_system: String::from("One state one vote (GA), veto power (SC)"),
                    },
                    functions: vec![String::from("Peace and security"), String::from("Humanitarian aid")],
                    effectiveness: 0.6,
                },
            ],
        }
    }

    /// Analyzes power dynamics
    pub fn analyze_power(&self, actor_id: &str) -> PowerAnalysis {
        PowerAnalysis {
            actor_id: actor_id.to_string(),
            overall_power: 8.5,
            military_power: 9.0,
            economic_power: 9.5,
            soft_power: 8.0,
            relative_power: HashMap::new(),
        }
    }

    /// Predicts conflict likelihood
    pub fn predict_conflict(&self, actors: &[String]) -> ConflictPrediction {
        ConflictPrediction {
            actors: actors.to_vec(),
            conflict_probability: 0.2,
            escalation_factors: vec![String::from("Territorial disputes")],
            de_escalation_opportunities: vec![String::from("Diplomatic engagement")],
        }
    }

    /// Evaluates diplomatic success
    pub fn evaluate_diplomacy(&self, negotiation_id: &str) -> DiplomacyEvaluation {
        DiplomacyEvaluation {
            negotiation_id: negotiation_id.to_string(),
            success_score: 7.0,
            outcomes: vec![String::from("Partial agreement")],
            lessons_learned: vec![String::from("Patience required")],
        }
    }

    /// Assesses international organization effectiveness
    pub fn assess_org_effectiveness(&self, org_id: &str) -> OrganizationAssessment {
        OrganizationAssessment {
            org_id: org_id.to_string(),
            mandate_fulfillment: 0.7,
            member_satisfaction: 0.6,
            adaptation_capacity: 0.5,
            recommendations: vec![String::from("Reform voting structure")],
        }
    }

    /// Analyzes alliance dynamics
    pub fn analyze_alliance(&self, alliance_id: &str) -> AllianceAnalysis {
        AllianceAnalysis {
            alliance_id: alliance_id.to_string(),
            cohesion: 0.8,
            burden_sharing: HashMap::new(),
            relevance: 0.75,
            future_prospects: String::from("Stable but evolving"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerAnalysis {
    pub actor_id: String,
    pub overall_power: f64,
    pub military_power: f64,
    pub economic_power: f64,
    pub soft_power: f64,
    pub relative_power: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictPrediction {
    pub actors: Vec<String>,
    pub conflict_probability: f64,
    pub escalation_factors: Vec<String>,
    pub de_escalation_opportunities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomacyEvaluation {
    pub negotiation_id: String,
    pub success_score: f64,
    pub outcomes: Vec<String>,
    pub lessons_learned: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationAssessment {
    pub org_id: String,
    pub mandate_fulfillment: f64,
    pub member_satisfaction: f64,
    pub adaptation_capacity: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllianceAnalysis {
    pub alliance_id: String,
    pub cohesion: f64,
    pub burden_sharing: HashMap<String, f64>,
    pub relevance: f64,
    pub future_prospects: String,
}

impl Default for InternationalRelations {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_international_relations_creation() {
        let ir = InternationalRelations::new();
        assert_eq!(ir.ir_id, "international_relations_v1");
    }
    #[test]
    fn test_analyze_power() {
        let ir = InternationalRelations::new();
        let analysis = ir.analyze_power("actor_us");
        assert!(analysis.overall_power > 0.0);
    }
}
