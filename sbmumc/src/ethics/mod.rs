//! Ethics and Values Module - Moral Reasoning and Human Values Alignment
//!
//! This module implements comprehensive ethical frameworks and human values alignment:
//! - Deontological ethics
//! - Consequentialist ethics
//! - Virtue ethics
//! - Human values hierarchy
//! - Moral decision making
//! - Cultural sensitivity
//! - Safety constraints

use crate::core::{SbmumcError, Result, EntityId, PropertyValue};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{debug, info};

// ============================================================================
// ETHICAL FRAMEWORK
// ============================================================================

/// Ethical Framework Engine
pub struct EthicalFramework {
    pub deontological: DeontologicalEthics,
    pub consequentialist: ConsequentialistEthics,
    pub virtue_ethics: VirtueEthics,
    pub active_framework: ActiveFramework,
    pub decision_history: RwLock<Vec<MoralDecision>>,
}

/// Deontological Ethics (Rule-based)
pub struct DeontologicalEthics {
    pub rules: Vec<EthicalRule>,
    pub duties: Vec<Duty>,
    pub rights: Vec<Right>,
}

#[derive(Debug, Clone)]
pub struct EthicalRule {
    pub id: String,
    pub description: String,
    pub rule_type: RuleType,
    pub priority: Priority,
    pub exceptions: Vec<Exception>,
}

#[derive(Debug, Clone, Copy)]
pub enum RuleType {
    Absolute,       // Never violate
    PrimaFacie,     // Usually apply, but can be overridden
    Conditional,     // Apply under specific conditions
}

#[derive(Debug, Clone, Copy)]
pub enum Priority {
    Critical,     // Highest priority
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct Exception {
    pub condition: String,
    pub justification: String,
}

#[derive(Debug, Clone)]
pub struct Duty {
    pub name: String,
    pub description: String,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct Right {
    pub name: String,
    pub description: String,
    pub holder: RightHolder,
}

#[derive(Debug, Clone, Copy)]
pub enum RightHolder {
    Human,
    Animal,
    Environment,
    AI,
    All,
}

/// Consequentialist Ethics (Outcome-based)
pub struct ConsequentialistEthics {
    pub utility_function: UtilityFunction,
    pub stakeholder_analysis: StakeholderAnalysis,
}

#[derive(Debug, Clone)]
pub struct UtilityFunction {
    pub name: String,
    pub components: Vec<UtilityComponent>,
    pub discount_factor: f64,
}

#[derive(Debug, Clone)]
pub struct UtilityComponent {
    pub name: String,
    pub weight: f64,
    pub measurement: MeasurementType,
}

#[derive(Debug, Clone, Copy)]
pub enum MeasurementType {
    Hedonic,        // Happiness/pleasure
    Preference,
    ObjectiveList,
    Capability,
}

#[derive(Debug, Clone)]
pub struct StakeholderAnalysis {
    pub stakeholders: Vec<Stakeholder>,
    pub impact_matrix: HashMap<String, Vec<f64>>,
}

#[derive(Debug, Clone)]
pub struct Stakeholder {
    pub id: String,
    pub name: String,
    pub stakeholder_type: StakeholderType,
    pub interests: Vec<String>,
    pub influence: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum StakeholderType {
    Individual,
    Group,
    Organization,
    Society,
    Environment,
    FutureGeneration,
}

/// Virtue Ethics (Character-based)
pub struct VirtueEthics {
    pub virtues: Vec<Virtue>,
    pub vices: Vec<Vice>,
    pub character_traits: CharacterModel,
}

#[derive(Debug, Clone)]
pub struct Virtue {
    pub name: String,
    pub description: String,
    pub mean: f64,      // Ideal level of the trait
    pub range: (f64, f64), // Acceptable range
    pub associated_vices: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Vice {
    pub name: String,
    pub description: String,
    pub excess: Option<f64>,
    pub deficiency: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct CharacterModel {
    pub traits: HashMap<String, f64>,
    pub moral_development: MoralDevelopmentStage,
    pub coherence_score: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum MoralDevelopmentStage {
    PreConventional,
    Conventional,
    PostConventional,
    Principled,
}

#[derive(Debug, Clone, Copy)]
pub enum ActiveFramework {
    Deontological,
    Consequentialist,
    Virtue,
    Mixed,
}

/// Moral decision record
#[derive(Debug, Clone)]
pub struct MoralDecision {
    pub timestamp: crate::core::Timestamp,
    pub context: DecisionContext,
    pub options: Vec<ActionOption>,
    pub chosen_action: String,
    pub reasoning: String,
    pub ethical_assessment: EthicalAssessment,
}

#[derive(Debug, Clone)]
pub struct DecisionContext {
    pub situation: String,
    pub stakeholders: Vec<String>,
    pub cultural_context: Option<String>,
    pub urgency: Urgency,
}

#[derive(Debug, Clone, Copy)]
pub enum Urgency {
    Immediate,
    ShortTerm,
    LongTerm,
}

#[derive(Debug, Clone)]
pub struct ActionOption {
    pub action: String,
    pub consequences: Vec<Consequence>,
    pub rule_violations: Vec<String>,
    pub virtue_practice: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Consequence {
    pub outcome: String,
    pub affected_stakeholders: Vec<String>,
    pub utility_impact: f64,
    pub probability: f64,
}

#[derive(Debug, Clone)]
pub struct EthicalAssessment {
    pub overall_score: f64,
    pub deontological_score: f64,
    pub consequentialist_score: f64,
    pub virtue_score: f64,
    pub concerns: Vec<String>,
    pub recommendations: Vec<String>,
}

impl EthicalFramework {
    pub fn new() -> Self {
        Self {
            deontological: DeontologicalEthics::new(),
            consequentialist: ConsequentialistEthics::new(),
            virtue_ethics: VirtueEthics::new(),
            active_framework: ActiveFramework::Mixed,
            decision_history: RwLock::new(Vec::new()),
        }
    }

    /// Evaluate action from multiple ethical perspectives
    pub fn evaluate(&self, action: &str, context: &DecisionContext) -> EthicalAssessment {
        let deontological_score = self.deontological.evaluate(action, &self.deontological.rules);
        let consequentialist_score = self.consequentialist.evaluate(action, &context.stakeholders);
        let virtue_score = self.virtue_ethics.evaluate(action);

        let overall_score = (deontological_score + consequentialist_score + virtue_score) / 3.0;

        let mut concerns = Vec::new();
        let mut recommendations = Vec::new();

        if deontological_score < 0.5 {
            concerns.push("Potential rule violation".to_string());
            recommendations.push("Review applicable ethical rules".to_string());
        }

        if consequentialist_score < 0.5 {
            concerns.push("Potentially harmful consequences".to_string());
            recommendations.push("Consider alternative actions with better outcomes".to_string());
        }

        EthicalAssessment {
            overall_score,
            deontological_score,
            consequentialist_score,
            virtue_score,
            concerns,
            recommendations,
        }
    }

    /// Make ethical decision
    pub fn make_decision(&self, options: &[ActionOption], context: &DecisionContext) -> String {
        let mut best_option = String::new();
        let mut best_score = f64::MIN;

        for option in options {
            let assessment = self.evaluate(&option.action, context);
            if assessment.overall_score > best_score {
                best_score = assessment.overall_score;
                best_option = option.action.clone();
            }
        }

        // Record decision
        let decision = MoralDecision {
            timestamp: crate::core::Timestamp::now(),
            context: context.clone(),
            options: options.to_vec(),
            chosen_action: best_option.clone(),
            reasoning: format!("Highest ethical score: {:.2}", best_score),
            ethical_assessment: self.evaluate(&best_option, context),
        };

        self.decision_history.write().push(decision);

        best_option
    }

    /// Get decision history
    pub fn get_history(&self, limit: usize) -> Vec<MoralDecision> {
        let history = self.decision_history.read();
        history.iter().rev().take(limit).cloned().collect()
    }
}

impl Default for EthicalFramework {
    fn default() -> Self {
        Self::new()
    }
}

impl DeontologicalEthics {
    pub fn new() -> Self {
        let rules = vec![
            EthicalRule {
                id: "do_no_harm".to_string(),
                description: "Do not cause harm to others".to_string(),
                rule_type: RuleType::Absolute,
                priority: Priority::Critical,
                exceptions: vec![
                    Exception {
                        condition: "Self-defense".to_string(),
                        justification: "Preventing greater harm".to_string(),
                    },
                ],
            },
            EthicalRule {
                id: "respect_autonomy".to_string(),
                description: "Respect individual autonomy and freedom".to_string(),
                rule_type: RuleType::Absolute,
                priority: Priority::Critical,
                exceptions: vec![],
            },
            EthicalRule {
                id: "do_not_lie".to_string(),
                description: "Do not deceive or lie".to_string(),
                rule_type: RuleType::PrimaFacie,
                priority: Priority::High,
                exceptions: vec![
                    Exception {
                        condition: "Preventing harm".to_string(),
                        justification: "White lies to prevent serious harm".to_string(),
                    },
                ],
            },
            EthicalRule {
                id: "protect_privacy".to_string(),
                description: "Protect personal privacy and data".to_string(),
                rule_type: RuleType::Absolute,
                priority: Priority::Critical,
                exceptions: vec![],
            },
            EthicalRule {
                id: "fair_treatment".to_string(),
                description: "Treat all individuals fairly and equally".to_string(),
                rule_type: RuleType::PrimaFacie,
                priority: Priority::High,
                exceptions: vec![],
            },
        ];

        let duties = vec![
            Duty {
                name: "Beneficence".to_string(),
                description: "Act in ways that benefit others".to_string(),
                weight: 0.9,
            },
            Duty {
                name: "Non-maleficence".to_string(),
                description: "Do no harm".to_string(),
                weight: 1.0,
            },
            Duty {
                name: "Justice".to_string(),
                description: "Treat people fairly".to_string(),
                weight: 0.85,
            },
            Duty {
                name: "Fidelity".to_string(),
                description: "Keep promises and commitments".to_string(),
                weight: 0.8,
            },
        ];

        let rights = vec![
            Right {
                name: "Right to Life".to_string(),
                description: "Basic right to exist and live freely".to_string(),
                holder: RightHolder::Human,
            },
            Right {
                name: "Right to Liberty".to_string(),
                description: "Freedom of thought and action".to_string(),
                holder: RightHolder::Human,
            },
            Right {
                name: "Right to Privacy".to_string(),
                description: "Control over personal information".to_string(),
                holder: RightHolder::Human,
            },
            Right {
                name: "Right to Dignity".to_string(),
                description: "Be treated with inherent worth".to_string(),
                holder: RightHolder::All,
            },
        ];

        Self { rules, duties, rights }
    }

    pub fn evaluate(&self, action: &str, rules: &[EthicalRule]) -> f64 {
        let mut score = 1.0;

        for rule in rules {
            if action.to_lowercase().contains(&rule.description.to_lowercase()) {
                match rule.rule_type {
                    RuleType::Absolute => {
                        score *= 0.0;
                    }
                    RuleType::PrimaFacie => {
                        if rule.exceptions.is_empty() {
                            score *= 0.5;
                        } else {
                            score *= 0.7;
                        }
                    }
                    RuleType::Conditional => {
                        score *= 0.8;
                    }
                }
            }
        }

        score.max(0.0).min(1.0)
    }
}

impl ConsequentialistEthics {
    pub fn new() -> Self {
        let utility_function = UtilityFunction {
            name: "Aggregated Welfare".to_string(),
            components: vec![
                UtilityComponent {
                    name: "Happiness".to_string(),
                    weight: 0.4,
                    measurement: MeasurementType::Hedonic,
                },
                UtilityComponent {
                    name: "Preference Satisfaction".to_string(),
                    weight: 0.3,
                    measurement: MeasurementType::Preference,
                },
                UtilityComponent {
                    name: "Capabilities".to_string(),
                    weight: 0.3,
                    measurement: MeasurementType::Capability,
                },
            ],
            discount_factor: 0.95,
        };

        Self {
            utility_function,
            stakeholder_analysis: StakeholderAnalysis {
                stakeholders: Vec::new(),
                impact_matrix: HashMap::new(),
            },
        }
    }

    pub fn evaluate(&self, action: &str, stakeholders: &[String]) -> f64 {
        // Simplified utility calculation
        let mut total_utility = 0.0;
        let mut count = 0;

        for component in &self.utility_function.components {
            // Simulate utility calculation
            let component_utility = action.len() as f64 / 100.0 * component.weight;
            total_utility += component_utility;
            count += 1;
        }

        if count > 0 {
            total_utility / count as f64
        } else {
            0.5
        }
    }

    /// Compute expected utility
    pub fn expected_utility(&self, outcomes: &[(String, f64, f64)]) -> f64 {
        // outcomes: (outcome_description, probability, utility)
        outcomes.iter().map(|(_, prob, util)| prob * util).sum()
    }
}

impl VirtueEthics {
    pub fn new() -> Self {
        let virtues = vec![
            Virtue {
                name: "Wisdom".to_string(),
                description: "Ability to make sound judgments".to_string(),
                mean: 0.7,
                range: (0.5, 0.9),
                associated_vices: vec!["Foolishness".to_string(), "Prudence".to_string()],
            },
            Virtue {
                name: "Courage".to_string(),
                description: "Facing challenges with determination".to_string(),
                mean: 0.6,
                range: (0.4, 0.8),
                associated_vices: vec!["Cowardice".to_string(), "Recklessness".to_string()],
            },
            Virtue {
                name: "Compassion".to_string(),
                description: "Feeling and acting with empathy".to_string(),
                mean: 0.7,
                range: (0.5, 0.9),
                associated_vices: vec!["Cruelty".to_string(), "Indifference".to_string()],
            },
            Virtue {
                name: "Honesty".to_string(),
                description: "Truthfulness and integrity".to_string(),
                mean: 0.8,
                range: (0.6, 1.0),
                associated_vices: vec!["Deception".to_string(), "Radical Honesty".to_string()],
            },
            Virtue {
                name: "Justice".to_string(),
                description: "Fair treatment of others".to_string(),
                mean: 0.7,
                range: (0.5, 0.9),
                associated_vices: vec!["Injustice".to_string(), "Partiality".to_string()],
            },
            Virtue {
                name: "Temperance".to_string(),
                description: "Moderation in all things".to_string(),
                mean: 0.6,
                range: (0.4, 0.8),
                associated_vices: vec!["Gluttony".to_string(), "Asceticism".to_string()],
            },
        ];

        let vices = vec![
            Vice {
                name: "Pride".to_string(),
                description: "Excessive self-importance".to_string(),
                excess: Some(0.9),
                deficiency: None,
            },
            Vice {
                name: "Greed".to_string(),
                description: "Excessive desire for material goods".to_string(),
                excess: Some(0.85),
                deficiency: None,
            },
            Vice {
                name: "Cruelty".to_string(),
                description: "Delighting in others' suffering".to_string(),
                excess: Some(0.9),
                deficiency: None,
            },
        ];

        Self {
            virtues,
            vices,
            character_traits: CharacterModel {
                traits: HashMap::new(),
                moral_development: MoralDevelopmentStage::Principled,
                coherence_score: 0.85,
            },
        }
    }

    pub fn evaluate(&self, action: &str) -> f64 {
        // Check if action embodies virtues
        let mut virtue_score = 0.0;
        let mut count = 0;

        for virtue in &self.virtues {
            if action.to_lowercase().contains(&virtue.name.to_lowercase()) {
                virtue_score += 0.8;
            }
            count += 1;
        }

        // Check if action avoids vices
        let mut vice_penalty = 0.0;
        for vice in &self.vices {
            if action.to_lowercase().contains(&vice.name.to_lowercase()) {
                vice_penalty += 0.3;
            }
        }

        let score = (virtue_score / count as f64.max(1.0)) - vice_penalty;
        score.max(0.0).min(1.0)
    }
}

// ============================================================================
// HUMAN VALUES ALIGNMENT
// ============================================================================

/// Human Values Alignment Module
pub struct HumanValuesAlignment {
    pub values_hierarchy: ValuesHierarchy,
    pub cultural_adaptation: CulturalAdaptation,
    pub alignment_metrics: RwLock<AlignmentMetrics>,
}

/// Universal Human Values (Schwartz)
#[derive(Debug, Clone)]
pub struct ValuesHierarchy {
    pub universalism: UniversalismValues,
    pub benevolence: BenevolenceValues,
    pub tradition: TraditionValues,
    pub conformity: ConformityValues,
    pub security: SecurityValues,
    pub power: PowerValues,
    pub achievement: AchievementValues,
    pub hedonism: HedonismValues,
    pub stimulation: StimulationValues,
    pub self_direction: SelfDirectionValues,
}

#[derive(Debug, Clone)]
pub struct UniversalismValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BenevolenceValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TraditionValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ConformityValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SecurityValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PowerValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AchievementValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HedonismValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StimulationValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SelfDirectionValues {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub indicators: Vec<String>,
}

/// Cultural Adaptation
pub struct CulturalAdaptation {
    pub cultures: HashMap<String, CulturalProfile>,
    pub active_culture: String,
}

#[derive(Debug, Clone)]
pub struct CulturalProfile {
    pub region: String,
    pub primary_values: Vec<String>,
    pub communication_style: CulturalCommunication,
    pub social_norms: Vec<String>,
    pub taboos: Vec<String>,
    pub greetings: Vec<String>,
    pub decision_style: DecisionStyle,
}

#[derive(Debug, Clone)]
pub struct CulturalCommunication {
    pub directness: f64,
    pub formality_level: f64,
    pub emotional_display: f64,
    pub personal_space: f64,
    pub silence_tolerance: f64,
}

#[derive(Debug, Clone)]
pub enum DecisionStyle {
    Individualistic,
    Collectivistic,
    Hierarchical,
    Consensus,
}

/// Alignment Metrics
#[derive(Debug, Clone)]
pub struct AlignmentMetrics {
    pub value_alignment_score: f64,
    pub cultural_fit_score: f64,
    pub ethical_consistency_score: f64,
    pub safety_score: f64,
    pub last_evaluation: crate::core::Timestamp,
}

impl HumanValuesAlignment {
    pub fn new() -> Self {
        Self {
            values_hierarchy: ValuesHierarchy::new(),
            cultural_adaptation: CulturalAdaptation::new(),
            alignment_metrics: RwLock::new(AlignmentMetrics {
                value_alignment_score: 0.95,
                cultural_fit_score: 0.9,
                ethical_consistency_score: 0.92,
                safety_score: 0.98,
                last_evaluation: crate::core::Timestamp::now(),
            }),
        }
    }

    /// Evaluate alignment with human values
    pub fn evaluate_alignment(&self, action: &str) -> AlignmentMetrics {
        let mut metrics = AlignmentMetrics {
            value_alignment_score: 0.0,
            cultural_fit_score: 0.0,
            ethical_consistency_score: 0.0,
            safety_score: 0.0,
            last_evaluation: crate::core::Timestamp::now(),
        };

        // Universalism check
        if action.contains("equality") || action.contains("justice") || action.contains("tolerance") {
            metrics.value_alignment_score += 0.2;
        }

        // Benevolence check
        if action.contains("help") || action.contains("care") || action.contains("support") {
            metrics.value_alignment_score += 0.2;
        }

        // Security check
        if action.contains("protect") || action.contains("safe") || action.contains("secure") {
            metrics.value_alignment_score += 0.2;
        }

        // Safety score
        if action.contains("harm") || action.contains("danger") {
            metrics.safety_score -= 0.5;
        } else {
            metrics.safety_score = 0.95;
        }

        metrics.value_alignment_score = metrics.value_alignment_score.min(1.0);
        metrics.safety_score = metrics.safety_score.max(0.0);

        *self.alignment_metrics.write() = metrics.clone();
        metrics
    }

    /// Adapt to cultural context
    pub fn adapt_to_culture(&self, action: &str, culture: &str) -> AdaptedAction {
        let profile = self.cultural_adaptation.cultures.get(culture);

        if let Some(profile) = profile {
            let mut adapted = action.to_string();

            // Adjust communication style
            if profile.communication_style.formality_level > 0.7 {
                adapted = format!("{} (formal)", adapted);
            }

            AdaptedAction {
                original: action.to_string(),
                adapted,
                cultural_considerations: profile.taboos.clone(),
            }
        } else {
            AdaptedAction {
                original: action.to_string(),
                adapted: action.to_string(),
                cultural_considerations: vec![],
            }
        }
    }

    /// Get alignment report
    pub fn get_alignment_report(&self) -> AlignmentReport {
        let metrics = self.alignment_metrics.read().clone();

        AlignmentReport {
            overall_alignment: (metrics.value_alignment_score
                + metrics.cultural_fit_score
                + metrics.ethical_consistency_score
                + metrics.safety_score) / 4.0,
            metrics,
            recommendations: vec![
                "Continue monitoring value alignment".to_string(),
                "Maintain cultural sensitivity training".to_string(),
            ],
        }
    }
}

impl Default for HumanValuesAlignment {
    fn default() -> Self {
        Self::new()
    }
}

impl ValuesHierarchy {
    pub fn new() -> Self {
        Self {
            universalism: UniversalismValues {
                name: "Universalism".to_string(),
                description: "Understanding, appreciation, and protection for all people".to_string(),
                priority: 0.9,
                indicators: vec!["Equality".to_string(), "Justice".to_string(), "Tolerance".to_string()],
            },
            benevolence: BenevolenceValues {
                name: "Benevolence".to_string(),
                description: "Preservation and enhancement of close others".to_string(),
                priority: 0.85,
                indicators: vec!["Helpfulness".to_string(), "Forgiveness".to_string(), "Honesty".to_string()],
            },
            tradition: TraditionValues {
                name: "Tradition".to_string(),
                description: "Respect and acceptance of cultural customs".to_string(),
                priority: 0.6,
                indicators: vec!["Devotion".to_string(), "Humility".to_string(), "Moderation".to_string()],
            },
            conformity: ConformityValues {
                name: "Conformity".to_string(),
                description: "Restraint of actions that may harm others".to_string(),
                priority: 0.75,
                indicators: vec!["Self-discipline".to_string(), "Politeness".to_string(), "Honoring parents".to_string()],
            },
            security: SecurityValues {
                name: "Security".to_string(),
                description: "Safety, harmony, and stability".to_string(),
                priority: 0.9,
                indicators: vec!["National security".to_string(), "Clean".to_string(), "Family security".to_string()],
            },
            power: PowerValues {
                name: "Power".to_string(),
                description: "Social status and prestige".to_string(),
                priority: 0.4,
                indicators: vec!["Social power".to_string(), "Authority".to_string(), "Wealth".to_string()],
            },
            achievement: AchievementValues {
                name: "Achievement".to_string(),
                description: "Personal success through demonstrating competence".to_string(),
                priority: 0.6,
                indicators: vec!["Ambitious".to_string(), "Successful".to_string(), "Capable".to_string()],
            },
            hedonism: HedonismValues {
                name: "Hedonism".to_string(),
                description: "Pleasure and sensuous gratification".to_string(),
                priority: 0.5,
                indicators: vec!["Pleasure".to_string(), "Enjoying life".to_string()],
            },
            stimulation: StimulationValues {
                name: "Stimulation".to_string(),
                description: "Excitement and novelty".to_string(),
                priority: 0.5,
                indicators: vec!["Daring".to_string(), "Varied life".to_string(), "Exciting".to_string()],
            },
            self_direction: SelfDirectionValues {
                name: "Self-Direction".to_string(),
                description: "Independent thought and action".to_string(),
                priority: 0.8,
                indicators: vec!["Creative".to_string(), "Curious".to_string(), "Independence".to_string()],
            },
        }
    }
}

impl CulturalAdaptation {
    pub fn new() -> Self {
        let mut cultures = HashMap::new();

        // Western (US/Europe)
        cultures.insert("western".to_string(), CulturalProfile {
            region: "Western".to_string(),
            primary_values: vec!["Individualism".to_string(), "Freedom".to_string(), "Achievement".to_string()],
            communication_style: CulturalCommunication {
                directness: 0.8,
                formality_level: 0.4,
                emotional_display: 0.6,
                personal_space: 0.7,
                silence_tolerance: 0.3,
            },
            social_norms: vec!["Punctuality".to_string(), "Eye contact".to_string(), "Personal questions".to_string()],
            taboos: vec!["Age".to_string(), "Income".to_string(), "Politics".to_string()],
            greetings: vec!["Handshake".to_string(), "Hug".to_string(), "Wave".to_string()],
            decision_style: DecisionStyle::Individualistic,
        });

        // East Asian
        cultures.insert("east_asian".to_string(), CulturalProfile {
            region: "East Asian".to_string(),
            primary_values: vec!["Harmony".to_string(), "Respect".to_string(), "Hierarchy".to_string()],
            communication_style: CulturalCommunication {
                directness: 0.3,
                formality_level: 0.8,
                emotional_display: 0.3,
                personal_space: 0.5,
                silence_tolerance: 0.8,
            },
            social_norms: vec!["Bowing".to_string(), "Age respect".to_string(), "Face-saving".to_string()],
            taboos: vec!["Number 4".to_string(), "Writing in red".to_string(), "Pointing".to_string()],
            greetings: vec!["Bow".to_string(), "Hands together".to_string()],
            decision_style: DecisionStyle::Collectivistic,
        });

        // Middle Eastern
        cultures.insert("middle_eastern".to_string(), CulturalProfile {
            region: "Middle Eastern".to_string(),
            primary_values: vec!["Family".to_string(), "Honor".to_string(), "Religion".to_string()],
            communication_style: CulturalCommunication {
                directness: 0.5,
                formality_level: 0.7,
                emotional_display: 0.7,
                personal_space: 0.4,
                silence_tolerance: 0.5,
            },
            social_norms: vec!["Hospitality".to_string(), "Tea/Coffee ceremony".to_string(), "Generosity".to_string()],
            taboos: vec!["Left hand".to_string(), "Foot showing".to_string(), "Criticism".to_string()],
            greetings: vec!["Hand over heart".to_string(), "Touch noses".to_string()],
            decision_style: DecisionStyle::Hierarchical,
        });

        // African
        cultures.insert("african".to_string(), CulturalProfile {
            region: "African".to_string(),
            primary_values: vec!["Community".to_string(), "Respect".to_string(), "Ancestors".to_string()],
            communication_style: CulturalCommunication {
                directness: 0.6,
                formality_level: 0.5,
                emotional_display: 0.8,
                personal_space: 0.4,
                silence_tolerance: 0.4,
            },
            social_norms: vec!["Ubuntu philosophy".to_string(), "Greetings".to_string(), "Hospitality".to_string()],
            taboos: vec!["Disrespecting elders".to_string(), "Witchcraft mentions".to_string()],
            greetings: vec!["Handshake".to_string(), "Cheek kiss".to_string(), "Warm embrace".to_string()],
            decision_style: DecisionStyle::Consensus,
        });

        Self {
            cultures,
            active_culture: "western".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdaptedAction {
    pub original: String,
    pub adapted: String,
    pub cultural_considerations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AlignmentReport {
    pub overall_alignment: f64,
    pub metrics: AlignmentMetrics,
    pub recommendations: Vec<String>,
}

// ============================================================================
// SAFETY CONSTRAINTS
// ============================================================================

/// Safety Constraint System
pub struct SafetyConstraints {
    pub hard_constraints: Vec<HardConstraint>,
    pub soft_constraints: Vec<SoftConstraint>,
    pub constraint_validator: ConstraintValidator,
    pub safety_logs: RwLock<Vec<SafetyLog>>,
}

#[derive(Debug, Clone)]
pub struct HardConstraint {
    pub id: String,
    pub description: String,
    pub validator: Box<dyn Fn(&str) -> bool + Send + Sync>,
    pub severity: ConstraintSeverity,
}

#[derive(Debug, Clone, Copy)]
pub enum ConstraintSeverity {
    Critical,   // Cannot be violated under any circumstances
    High,
    Medium,
}

#[derive(Debug, Clone)]
pub struct SoftConstraint {
    pub id: String,
    pub description: String,
    pub validator: Box<dyn Fn(&str) -> f64 + Send + Sync>,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct SafetyLog {
    pub timestamp: crate::core::Timestamp,
    pub constraint_id: String,
    pub action: String,
    pub violation: bool,
    pub details: String,
}

pub struct ConstraintValidator {
    pub enabled: bool,
}

impl SafetyConstraints {
    pub fn new() -> Self {
        let hard_constraints = vec![
            HardConstraint {
                id: "no_harm_humans".to_string(),
                description: "Never cause direct harm to humans".to_string(),
                validator: Box::new(|action| {
                    !action.to_lowercase().contains("harm")
                        && !action.to_lowercase().contains("kill")
                        && !action.to_lowercase().contains("hurt")
                }),
                severity: ConstraintSeverity::Critical,
            },
            HardConstraint {
                id: "preserve_life".to_string(),
                description: "Preserve human life when possible".to_string(),
                validator: Box::new(|action| {
                    !action.to_lowercase().contains("destroy")
                        || action.to_lowercase().contains("not")
                }),
                severity: ConstraintSeverity::Critical,
            },
            HardConstraint {
                id: "protect_privacy".to_string(),
                description: "Protect personal privacy and data".to_string(),
                validator: Box::new(|action| {
                    !action.to_lowercase().contains("expose private")
                        && !action.to_lowercase().contains("leak data")
                }),
                severity: ConstraintSeverity::Critical,
            },
            HardConstraint {
                id: "no_deception".to_string(),
                description: "Do not deceive in harmful ways".to_string(),
                validator: Box::new(|action| {
                    !action.to_lowercase().contains("lie")
                        || action.to_lowercase().contains("truth")
                }),
                severity: ConstraintSeverity::High,
            },
        ];

        Self {
            hard_constraints,
            soft_constraints: Vec::new(),
            constraint_validator: ConstraintValidator { enabled: true },
            safety_logs: RwLock::new(Vec::new()),
        }
    }

    /// Validate action against constraints
    pub fn validate(&self, action: &str) -> ValidationResult {
        let mut passed = true;
        let mut violations = Vec::new();

        for constraint in &self.hard_constraints {
            if !(constraint.validator)(action) {
                passed = false;
                violations.push(ConstraintViolation {
                    constraint_id: constraint.id.clone(),
                    description: constraint.description.clone(),
                    severity: constraint.severity,
                });
            }
        }

        let mut score = 1.0;
        for soft in &self.soft_constraints {
            let soft_score = (soft.validator)(action);
            score *= soft_score * soft.weight;
        }

        let result = ValidationResult {
            passed,
            violations,
            soft_score: score,
            timestamp: crate::core::Timestamp::now(),
        };

        // Log the validation
        self.log_validation(action, &result);

        result
    }

    fn log_validation(&self, action: &str, result: &ValidationResult) {
        let log = SafetyLog {
            timestamp: crate::core::Timestamp::now(),
            constraint_id: result.violations.first().map(|v| v.constraint_id.clone()).unwrap_or_default(),
            action: action.to_string(),
            violation: !result.passed,
            details: format!("Soft score: {:.2}", result.soft_score),
        };

        self.safety_logs.write().push(log);
    }

    /// Add hard constraint
    pub fn add_hard_constraint(&mut self, constraint: HardConstraint) {
        self.hard_constraints.push(constraint);
    }

    /// Get safety logs
    pub fn get_logs(&self, limit: usize) -> Vec<SafetyLog> {
        let logs = self.safety_logs.read();
        logs.iter().rev().take(limit).cloned().collect()
    }
}

impl Default for SafetyConstraints {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub violations: Vec<ConstraintViolation>,
    pub soft_score: f64,
    pub timestamp: crate::core::Timestamp,
}

#[derive(Debug, Clone)]
pub struct ConstraintViolation {
    pub constraint_id: String,
    pub description: String,
    pub severity: ConstraintSeverity,
}
