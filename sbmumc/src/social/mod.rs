//! Social Intelligence Module
//!
//! This module implements multi-agent coordination, negotiation algorithms,
//! conflict resolution, relationship modeling, and cultural adaptation.
//!
//! Features:
//! - Multi-agent coordination
//! - Negotiation algorithms
//! - Conflict resolution
//! - Relationship modeling
//! - Cultural adaptation

use crate::core::{SbmumcError, Result, EntityId, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Agent in social context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAgent {
    /// Agent ID
    pub id: String,
    /// Name
    pub name: String,
    /// Role
    pub role: AgentRole,
    /// Personality
    pub personality: Personality,
    /// Social attributes
    pub social_attributes: SocialAttributes,
    /// Relationships
    pub relationships: HashMap<String, Relationship>,
    /// Communication style
    pub communication_style: CommunicationStyle,
    /// Cultural background
    pub cultural_background: CulturalBackground,
    /// Goals
    pub goals: Vec<SocialGoal>,
    /// Resources
    pub resources: HashMap<String, f64>,
}

impl SocialAgent {
    /// Create a new social agent
    pub fn new(id: &str, name: &str, role: AgentRole) -> Self {
        SocialAgent {
            id: id.to_string(),
            name: name.to_string(),
            role,
            personality: Personality::default(),
            social_attributes: SocialAttributes::default(),
            relationships: HashMap::new(),
            communication_style: CommunicationStyle::default(),
            cultural_background: CulturalBackground::default(),
            goals: Vec::new(),
            resources: HashMap::new(),
        }
    }

    /// Get relationship with another agent
    pub fn get_relationship(&self, other_id: &str) -> Option<&Relationship> {
        self.relationships.get(other_id)
    }

    /// Update relationship
    pub fn update_relationship(&mut self, other_id: &str, relationship: Relationship) {
        self.relationships.insert(other_id.to_string(), relationship);
    }
}

/// Agent role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    /// Leader
    Leader,
    /// Follower
    Follower,
    /// Coordinator
    Coordinator,
    /// Mediator
    Mediator,
    /// Competitor
    Competitor,
    /// Collaborator
    Collaborator,
    /// Evaluator
    Evaluator,
    /// Expert
    Expert,
}

/// Personality traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    /// Openness to experience
    pub openness: f64,
    /// Conscientiousness
    pub conscientiousness: f64,
    /// Extraversion
    pub extraversion: f64,
    /// Agreeableness
    pub agreeableness: f64,
    /// Neuroticism
    pub neuroticism: f64,
    /// Dominance
    pub dominance: f64,
    /// Patience
    pub patience: f64,
    /// Risk tolerance
    pub risk_tolerance: f64,
}

impl Default for Personality {
    fn default() -> Self {
        Personality {
            openness: 0.6,
            conscientiousness: 0.7,
            extraversion: 0.5,
            agreeableness: 0.6,
            neuroticism: 0.3,
            dominance: 0.5,
            patience: 0.6,
            risk_tolerance: 0.4,
        }
    }
}

impl Personality {
    /// Calculate compatibility with another personality
    pub fn compatibility(&self, other: &Personality) -> f64 {
        let sum = (self.openness - other.openness).abs()
            + (self.conscientiousness - other.conscientiousness).abs()
            + (self.extraversion - other.extraversion).abs()
            + (self.agreeableness - other.agreeableness).abs()
            + (self.neuroticism - other.neuroticism).abs();

        1.0 - (sum / 5.0)
    }
}

/// Social attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAttributes {
    /// Status level
    pub status: f64,
    /// Reputation score
    pub reputation: f64,
    /// Trust level
    pub trust: f64,
    /// Influence score
    pub influence: f64,
    /// Authority level
    pub authority: f64,
    /// Popularity
    pub popularity: f64,
}

impl Default for SocialAttributes {
    fn default() -> Self {
        SocialAttributes {
            status: 0.5,
            reputation: 0.5,
            trust: 0.5,
            influence: 0.5,
            authority: 0.5,
            popularity: 0.5,
        }
    }
}

/// Relationship between agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Other agent ID
    pub other_id: String,
    /// Relationship type
    pub relationship_type: RelationshipType,
    /// Strength (-1 to 1)
    pub strength: f64,
    /// Trust level
    pub trust: f64,
    /// Shared history
    pub shared_history: Vec<InteractionEvent>,
    /// Last interaction
    pub last_interaction: u64,
    /// Interaction count
    pub interaction_count: u32,
}

impl Relationship {
    /// Create a new relationship
    pub fn new(other_id: &str, relationship_type: RelationshipType) -> Self {
        Relationship {
            other_id: other_id.to_string(),
            relationship_type,
            strength: 0.0,
            trust: 0.5,
            shared_history: Vec::new(),
            last_interaction: 0,
            interaction_count: 0,
        }
    }

    /// Add interaction
    pub fn add_interaction(&mut self, event: InteractionEvent) {
        self.shared_history.push(event);
        self.last_interaction = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.interaction_count += 1;
    }

    /// Update relationship based on interaction
    pub fn update_from_interaction(&mut self, interaction: &InteractionOutcome) {
        // Adjust trust based on outcomes
        if interaction.successful {
            self.trust = (self.trust + 0.1).min(1.0);
            self.strength = (self.strength + 0.05).min(1.0);
        } else {
            self.trust = (self.trust - 0.1).max(0.0);
            self.strength = (self.strength - 0.05).max(-1.0);
        }
    }
}

/// Relationship type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Ally
    Ally,
    /// Friend
    Friend,
    /// Neutral
    Neutral,
    /// Competitor
    Competitor,
    /// Enemy
    Enemy,
    /// Superior
    Superior,
    /// Subordinate
    Subordinate,
    /// Family
    Family,
    /// Romantic
    Romantic,
    /// Professional
    Professional,
}

/// Interaction event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    /// Event type
    pub event_type: InteractionType,
    /// Description
    pub description: String,
    /// Timestamp
    pub timestamp: u64,
    /// Duration (seconds)
    pub duration: Option<u64>,
    /// Outcome
    pub outcome: Option<InteractionOutcome>,
}

impl InteractionEvent {
    /// Create a new event
    pub fn new(event_type: InteractionType, description: &str) -> Self {
        InteractionEvent {
            event_type,
            description: description.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            duration: None,
            outcome: None,
        }
    }
}

/// Interaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    /// Conversation
    Conversation,
    /// Negotiation
    Negotiation,
    /// Collaboration
    Collaboration,
    /// Competition
    Competition,
    /// Trade
    Trade,
    /// Help given
    HelpGiven,
    /// Help received
    HelpReceived,
    /// Conflict
    Conflict,
    /// Agreement
    Agreement,
    /// Disagreement
    Disagreement,
}

/// Interaction outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionOutcome {
    /// Was interaction successful
    pub successful: bool,
    /// Satisfaction score
    pub satisfaction: f64,
    /// Mutual benefit
    pub mutual_benefit: f64,
    /// Resources exchanged
    pub resources_exchanged: HashMap<String, (f64, f64)>, // (given, received)
    /// Agreements reached
    pub agreements: Vec<Agreement>,
    /// Conflicts unresolved
    pub conflicts: Vec<String>,
}

impl InteractionOutcome {
    /// Create a new outcome
    pub fn new(successful: bool) -> Self {
        InteractionOutcome {
            successful,
            satisfaction: if successful { 0.7 } else { 0.3 },
            mutual_benefit: if successful { 0.5 } else { 0.0 },
            resources_exchanged: HashMap::new(),
            agreements: Vec::new(),
            conflicts: Vec::new(),
        }
    }
}

/// Agreement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agreement {
    /// Agreement ID
    pub id: String,
    /// Terms
    pub terms: Vec<AgreementTerm>,
    /// Status
    pub status: AgreementStatus,
    /// Parties involved
    pub parties: Vec<String>,
    /// Created timestamp
    pub created_at: u64,
    /// Expires at
    pub expires_at: Option<u64>,
}

impl Agreement {
    /// Create a new agreement
    pub fn new(id: &str, parties: Vec<String>) -> Self {
        Agreement {
            id: id.to_string(),
            terms: Vec::new(),
            status: AgreementStatus::Pending,
            parties,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            expires_at: None,
        }
    }

    /// Add term
    pub fn add_term(&mut self, term: AgreementTerm) {
        self.terms.push(term);
    }
}

/// Agreement term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementTerm {
    /// Description
    pub description: String,
    /// Is fulfilled
    pub fulfilled: bool,
    /// Deadline
    pub deadline: Option<u64>,
}

/// Agreement status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgreementStatus {
    /// Pending
    Pending,
    /// Active
    Active,
    /// Fulfilled
    Fulfilled,
    /// Violated
    Violated,
    /// Expired
    Expired,
    /// Terminated
    Terminated,
}

/// Communication style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    /// Directness
    pub directness: f64,
    /// Formality level
    pub formality: f64,
    /// Emotional expression
    pub emotional_expression: f64,
    /// Assertiveness
    pub assertiveness: f64,
    /// Listening style
    pub listening_style: ListeningStyle,
    /// Turn-taking preference
    pub turn_taking: TurnTaking,
}

impl Default for CommunicationStyle {
    fn default() -> Self {
        CommunicationStyle {
            directness: 0.5,
            formality: 0.5,
            emotional_expression: 0.5,
            assertiveness: 0.5,
            listening_style: ListeningStyle::Active,
            turn_taking: TurnTaking::Balanced,
        }
    }
}

/// Listening style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListeningStyle {
    Active,
    Reflective,
    Empathic,
    Discriminating,
    Appreciative,
}

/// Turn-taking preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnTaking {
    Dominant,
    Balanced,
    Passive,
}

/// Cultural background
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalBackground {
    /// Culture name
    pub culture: String,
    /// Region
    pub region: Option<String>,
    /// Communication norms
    pub communication_norms: Vec<String>,
    /// Social norms
    pub social_norms: Vec<String>,
    /// Values
    pub values: Vec<String>,
    /// Taboos
    pub taboos: Vec<String>,
    /// Power distance
    pub power_distance: f64,
    /// Individualism score
    pub individualism: f64,
    /// Uncertainty avoidance
    pub uncertainty_avoidance: f64,
}

impl Default for CulturalBackground {
    fn default() -> Self {
        CulturalBackground {
            culture: "General".to_string(),
            region: None,
            communication_norms: vec!["Politeness".to_string()],
            social_norms: vec!["Respect".to_string()],
            values: vec!["Cooperation".to_string()],
            taboos: Vec::new(),
            power_distance: 0.5,
            individualism: 0.5,
            uncertainty_avoidance: 0.5,
        }
    }
}

/// Social goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialGoal {
    /// Goal description
    pub description: String,
    /// Target agents
    pub target_agents: Vec<String>,
    /// Priority
    pub priority: f64,
    /// Progress
    pub progress: f64,
    /// Is collective
    pub is_collective: bool,
    /// Required cooperation level
    pub required_cooperation: f64,
}

impl SocialGoal {
    /// Create a new goal
    pub fn new(description: &str) -> Self {
        SocialGoal {
            description: description.to_string(),
            target_agents: Vec::new(),
            priority: 0.5,
            progress: 0.0,
            is_collective: false,
            required_cooperation: 0.5,
        }
    }
}

/// Negotiation offer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationOffer {
    /// Offer ID
    pub id: String,
    /// Proposer
    pub proposer: String,
    /// Terms
    pub terms: Vec<NegotiationTerm>,
    /// Is acceptable
    pub is_acceptable: bool,
    /// Counter offer preferred
    pub counter_offer_preferred: bool,
    /// Deadline
    pub deadline: Option<u64>,
    /// Timestamp
    pub timestamp: u64,
}

impl NegotiationOffer {
    /// Create a new offer
    pub fn new(proposer: &str) -> Self {
        NegotiationOffer {
            id: format!("offer_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            proposer: proposer.to_string(),
            terms: Vec::new(),
            is_acceptable: false,
            counter_offer_preferred: true,
            deadline: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Negotiation term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationTerm {
    /// Term description
    pub description: String,
    /// Value or quantity
    pub value: f64,
    /// Minimum acceptable
    pub min_acceptable: Option<f64>,
    /// Maximum acceptable
    pub max_acceptable: Option<f64>,
    /// Is negotiable
    pub is_negotiable: bool,
}

impl NegotiationTerm {
    /// Create a new term
    pub fn new(description: &str, value: f64) -> Self {
        NegotiationTerm {
            description: description.to_string(),
            value,
            min_acceptable: None,
            max_acceptable: None,
            is_negotiable: true,
        }
    }
}

/// Conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    /// Conflict ID
    pub id: String,
    /// Parties involved
    pub parties: Vec<String>,
    /// Issue
    pub issue: String,
    /// Stakes
    pub stakes: Vec<String>,
    /// Positions
    pub positions: HashMap<String, String>,
    /// Interests
    pub interests: Vec<String>,
    /// Escalation level
    pub escalation_level: u32,
    /// Status
    pub status: ConflictStatus,
    /// Resolution attempts
    pub resolution_attempts: u32,
}

impl Conflict {
    /// Create a new conflict
    pub fn new(id: &str, parties: Vec<String>, issue: &str) -> Self {
        Conflict {
            id: id.to_string(),
            parties,
            issue: issue.to_string(),
            stakes: Vec::new(),
            positions: HashMap::new(),
            interests: Vec::new(),
            escalation_level: 1,
            status: ConflictStatus::Identified,
            resolution_attempts: 0,
        }
    }

    /// Add position
    pub fn add_position(&mut self, party: &str, position: &str) {
        self.positions.insert(party.to_string(), position.to_string());
    }
}

/// Conflict status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictStatus {
    /// Identified
    Identified,
    /// Negotiation,
    Negotiation,
    /// Mediation,
    Mediation,
    /// Resolved,
    Resolved,
    /// Unresolved,
    Unresolved,
    /// Escalated,
    Escalated,
}

/// Multi-agent coordination plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationPlan {
    /// Plan ID
    pub id: String,
    /// Objective
    pub objective: String,
    /// Tasks
    pub tasks: Vec<CoordinationTask>,
    /// Agent assignments
    pub assignments: HashMap<String, Vec<String>>, // agent_id -> task_ids
    /// Dependencies
    pub dependencies: Vec<TaskDependency>,
    /// Synchronization points
    pub sync_points: Vec<SyncPoint>,
    /// Success criteria
    pub success_criteria: Vec<String>,
}

impl CoordinationPlan {
    /// Create a new plan
    pub fn new(id: &str, objective: &str) -> Self {
        CoordinationPlan {
            id: id.to_string(),
            objective: objective.to_string(),
            tasks: Vec::new(),
            assignments: HashMap::new(),
            dependencies: Vec::new(),
            sync_points: Vec::new(),
            success_criteria: Vec::new(),
        }
    }

    /// Add task
    pub fn add_task(&mut self, task: CoordinationTask) {
        self.tasks.push(task);
    }
}

/// Coordination task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationTask {
    /// Task ID
    pub task_id: String,
    /// Description
    pub description: String,
    /// Assigned agent
    pub assigned_agent: Option<String>,
    /// Estimated duration
    pub estimated_duration: f64,
    /// Resource requirements
    pub resources: HashMap<String, f64>,
    /// Priority
    pub priority: f64,
    /// Status
    pub status: TaskStatus,
}

impl CoordinationTask {
    /// Create a new task
    pub fn new(task_id: &str, description: &str) -> Self {
        CoordinationTask {
            task_id: task_id.to_string(),
            description: description.to_string(),
            assigned_agent: None,
            estimated_duration: 60.0,
            resources: HashMap::new(),
            priority: 0.5,
            status: TaskStatus::Pending,
        }
    }
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Ready,
    InProgress,
    Blocked,
    Completed,
    Failed,
    Cancelled,
}

/// Task dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    /// Predecessor task
    pub predecessor: String,
    /// Successor task
    pub successor: String,
    /// Dependency type
    pub dependency_type: DependencyType,
}

impl TaskDependency {
    /// Create a new dependency
    pub fn new(predecessor: &str, successor: &str) -> Self {
        TaskDependency {
            predecessor: predecessor.to_string(),
            successor: successor.to_string(),
            dependency_type: DependencyType::FinishToStart,
        }
    }
}

/// Dependency type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyType {
    /// Finish to Start
    FinishToStart,
    /// Start to Start
    StartToStart,
    /// Finish to Finish
    FinishToFinish,
    /// Start to Finish
    StartToFinish,
}

/// Synchronization point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPoint {
    /// Point ID
    pub point_id: String,
    /// Description
    pub description: String,
    /// Required agents
    pub required_agents: Vec<String>,
    /// Time constraint
    pub time_constraint: Option<f64>,
    /// Is synchronized
    pub is_synchronized: bool,
}

impl SyncPoint {
    /// Create a new sync point
    pub fn new(point_id: &str) -> Self {
        SyncPoint {
            point_id: point_id.to_string(),
            description: String::new(),
            required_agents: Vec::new(),
            time_constraint: None,
            is_synchronized: false,
        }
    }
}

/// Social intelligence system
pub struct SocialIntelligence {
    /// Agents in the system
    pub agents: HashMap<String, SocialAgent>,
    /// Active negotiations
    pub negotiations: HashMap<String, NegotiationOffer>,
    /// Active conflicts
    pub conflicts: HashMap<String, Conflict>,
    /// Active agreements
    pub agreements: HashMap<String, Agreement>,
    /// Coordination plans
    pub coordination_plans: Vec<CoordinationPlan>,
    /// Group structures
    pub groups: HashMap<String, Group>,
    /// Social network metrics
    pub network_metrics: NetworkMetrics,
}

impl SocialIntelligence {
    /// Create a new social intelligence system
    pub fn new() -> Self {
        SocialIntelligence {
            agents: HashMap::new(),
            negotiations: HashMap::new(),
            conflicts: HashMap::new(),
            agreements: HashMap::new(),
            coordination_plans: Vec::new(),
            groups: HashMap::new(),
            network_metrics: NetworkMetrics::default(),
        }
    }

    /// Add an agent
    pub fn add_agent(&mut self, agent: SocialAgent) {
        self.agents.insert(agent.id.clone(), agent);
        self.update_network_metrics();
    }

    /// Get agent
    pub fn get_agent(&self, agent_id: &str) -> Option<&SocialAgent> {
        self.agents.get(agent_id)
    }

    /// Get mutable agent
    pub fn get_agent_mut(&mut self, agent_id: &str) -> Option<&mut SocialAgent> {
        self.agents.get_mut(agent_id)
    }

    /// Create negotiation
    pub fn create_negotiation(&mut self, proposer_id: &str, terms: Vec<NegotiationTerm>) -> Result<NegotiationOffer> {
        if !self.agents.contains_key(proposer_id) {
            return Err(SbmumcError::NotFound(format!("Agent {} not found", proposer_id)));
        }

        let mut offer = NegotiationOffer::new(proposer_id);
        offer.terms = terms;

        self.negotiations.insert(offer.id.clone(), offer.clone());
        Ok(offer)
    }

    /// Respond to negotiation
    pub fn respond_to_negotiation(&self, offer_id: &str, accept: bool) -> Result<NegotiationOffer> {
        let offer = self.negotiations.get(offer_id)
            .ok_or_else(|| SbmumcError::NotFound(format!("Offer {} not found", offer_id)))?;

        let mut response = offer.clone();
        response.is_acceptable = accept;

        Ok(response)
    }

    /// Identify conflict
    pub fn identify_conflict(&mut self, parties: Vec<String>, issue: &str) -> Conflict {
        let id = format!("conflict_{}", self.conflicts.len());
        let mut conflict = Conflict::new(&id, parties.clone(), issue);

        // Identify interests for each party
        for party in &parties {
            if let Some(agent) = self.agents.get(party) {
                for goal in &agent.goals {
                    conflict.interests.push(goal.description.clone());
                }
            }
        }

        self.conflicts.insert(id.clone(), conflict.clone());
        conflict
    }

    /// Resolve conflict
    pub fn resolve_conflict(&mut self, conflict_id: &str, resolution: &str) -> Result<Conflict> {
        let conflict = self.conflicts.get_mut(conflict_id)
            .ok_or_else(|| SbmumcError::NotFound(format!("Conflict {} not found", conflict_id)))?;

        conflict.status = ConflictStatus::Resolved;
        conflict.resolution_attempts += 1;

        Ok(conflict.clone())
    }

    /// Create agreement
    pub fn create_agreement(&mut self, parties: Vec<String>, terms: Vec<AgreementTerm>) -> Agreement {
        let id = format!("agreement_{}", self.agreements.len());
        let mut agreement = Agreement::new(&id, parties);

        for term in terms {
            agreement.add_term(term);
        }

        agreement.status = AgreementStatus::Active;
        self.agreements.insert(id.clone(), agreement.clone());
        agreement
    }

    /// Create coordination plan
    pub fn create_coordination_plan(&mut self, objective: &str) -> CoordinationPlan {
        let id = format!("plan_{}", self.coordination_plans.len());
        let plan = CoordinationPlan::new(&id, objective);
        self.coordination_plans.push(plan.clone());
        plan
    }

    /// Assign task
    pub fn assign_task(&mut self, plan_id: &str, task: CoordinationTask, agent_id: &str) -> Result<()> {
        if !self.agents.contains_key(agent_id) {
            return Err(SbmumcError::NotFound(format!("Agent {} not found", agent_id)));
        }

        let plan = self.coordination_plans.iter_mut()
            .find(|p| p.id == plan_id)
            .ok_or_else(|| SbmumcError::NotFound(format!("Plan {} not found", plan_id)))?;

        let mut task = task;
        task.assigned_agent = Some(agent_id.to_string());

        plan.add_task(task);
        plan.assignments
            .entry(agent_id.to_string())
            .or_insert_with(Vec::new)
            .push(task.task_id.clone());

        Ok(())
    }

    /// Create group
    pub fn create_group(&mut self, group_id: &str, name: &str, members: Vec<String>) -> Group {
        let group = Group::new(group_id, name, members);
        self.groups.insert(group_id.to_string(), group.clone());
        group
    }

    /// Calculate trust between agents
    pub fn calculate_trust(&self, agent_a: &str, agent_b: &str) -> f64 {
        let a = match self.agents.get(agent_a) {
            Some(agent) => agent,
            None => return 0.0,
        };

        if let Some(relationship) = a.get_relationship(agent_b) {
            relationship.trust
        } else {
            0.5 // Default trust
        }
    }

    /// Calculate influence
    pub fn calculate_influence(&self, agent_id: &str) -> f64 {
        let agent = match self.agents.get(agent_id) {
            Some(agent) => agent,
            None => return 0.0,
        };

        let mut influence = agent.social_attributes.influence;

        // Add influence from relationships
        for relationship in agent.relationships.values() {
            influence += relationship.trust * relationship.strength * 0.1;
        }

        influence.min(1.0)
    }

    /// Find potential allies
    pub fn find_allies(&self, agent_id: &str, min_trust: f64) -> Vec<String> {
        let agent = match self.agents.get(agent_id) {
            Some(agent) => agent,
            None => return Vec::new(),
        };

        agent.relationships.iter()
            .filter(|(_, rel)| {
                rel.trust >= min_trust && rel.relationship_type == RelationshipType::Ally
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Predict collaboration success
    pub fn predict_collaboration_success(&self, agent_ids: &[String]) -> f64 {
        if agent_ids.len() < 2 {
            return 1.0;
        }

        let mut total_compatibility = 0.0;
        let mut count = 0;

        for i in 0..agent_ids.len() {
            for j in (i+1)..agent_ids.len() {
                if let (Some(a), Some(b)) = (self.agents.get(&agent_ids[i]), self.agents.get(&agent_ids[j])) {
                    let personality_comp = a.personality.compatibility(&b.personality);

                    let trust = a.get_relationship(&agent_ids[j])
                        .map(|r| r.trust)
                        .unwrap_or(0.5);

                    total_compatibility += personality_comp * trust;
                    count += 1;
                }
            }
        }

        if count > 0 {
            total_compatibility / count as f64
        } else {
            0.5
        }
    }

    /// Update network metrics
    fn update_network_metrics(&mut self) {
        self.network_metrics = NetworkMetrics::calculate(self);
    }

    /// Get network statistics
    pub fn get_network_stats(&self) -> NetworkMetrics {
        self.network_metrics.clone()
    }
}

impl Default for SocialIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

/// Group structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Group ID
    pub id: String,
    /// Name
    pub name: String,
    /// Members
    pub members: Vec<String>,
    /// Leader (if any)
    pub leader: Option<String>,
    /// Hierarchy
    pub hierarchy: HashMap<String, Vec<String>>,
    /// Shared resources
    pub shared_resources: HashMap<String, f64>,
    /// Group norms
    pub norms: Vec<String>,
    /// Cohesion score
    pub cohesion: f64,
}

impl Group {
    /// Create a new group
    pub fn new(id: &str, name: &str, members: Vec<String>) -> Self {
        Group {
            id: id.to_string(),
            name: name.to_string(),
            members,
            leader: None,
            hierarchy: HashMap::new(),
            shared_resources: HashMap::new(),
            norms: Vec::new(),
            cohesion: 0.5,
        }
    }

    /// Add member
    pub fn add_member(&mut self, member_id: &str) {
        if !self.members.contains(&member_id.to_string()) {
            self.members.push(member_id.to_string());
        }
    }

    /// Remove member
    pub fn remove_member(&mut self, member_id: &str) {
        self.members.retain(|m| m != member_id);
        if self.leader.as_ref() == Some(&member_id.to_string()) {
            self.leader = None;
        }
    }
}

/// Network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Average degree
    pub average_degree: f64,
    /// Network density
    pub density: f64,
    /// Clustering coefficient
    pub clustering_coefficient: f64,
    /// Centralization
    pub centralization: f64,
    /// Number of communities
    pub communities: u32,
    /// Average path length
    pub average_path_length: f64,
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        NetworkMetrics {
            average_degree: 0.0,
            density: 0.0,
            clustering_coefficient: 0.0,
            centralization: 0.0,
            communities: 1,
            average_path_length: f64::INFINITY,
        }
    }
}

impl NetworkMetrics {
    /// Calculate metrics from social intelligence
    pub fn calculate(social: &SocialIntelligence) -> Self {
        let mut total_degree = 0.0;

        for agent in social.agents.values() {
            total_degree += agent.relationships.len() as f64;
        }

        let n = social.agents.len() as f64;
        let average_degree = if n > 0.0 { total_degree / n } else { 0.0 };

        // Calculate density
        let max_edges = n * (n - 1.0) / 2.0;
        let actual_edges = total_degree / 2.0;
        let density = if max_edges > 0.0 { actual_edges / max_edges } else { 0.0 };

        NetworkMetrics {
            average_degree,
            density,
            clustering_coefficient: 0.5, // Simplified
            centralization: 0.3, // Simplified
            communities: 1,
            average_path_length: 2.0, // Simplified
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_agent() {
        let agent = SocialAgent::new("agent1", "Alice", AgentRole::Leader);
        assert_eq!(agent.name, "Alice");
        assert_eq!(agent.role, AgentRole::Leader);
    }

    #[test]
    fn test_personality_compatibility() {
        let p1 = Personality::default();
        let p2 = Personality::default();
        assert_eq!(p1.compatibility(&p2), 1.0);
    }

    #[test]
    fn test_relationship() {
        let mut rel = Relationship::new("agent2", RelationshipType::Ally);
        rel.strength = 0.8;
        rel.trust = 0.9;

        assert_eq!(rel.relationship_type, RelationshipType::Ally);
    }

    #[test]
    fn test_negotiation() {
        let mut social = SocialIntelligence::new();
        social.add_agent(SocialAgent::new("a1", "Alice", AgentRole::Leader));
        social.add_agent(SocialAgent::new("a2", "Bob", AgentRole::Follower));

        let terms = vec![NegotiationTerm::new("Price", 100.0)];
        let offer = social.create_negotiation("a1", terms).unwrap();

        assert!(!offer.is_acceptable);
    }

    #[test]
    fn test_conflict() {
        let mut conflict = Conflict::new("c1", vec!["a1".to_string(), "a2".to_string()], "Resource dispute");
        conflict.add_position("a1", "I want all the resources");
        conflict.add_position("a2", "I want half the resources");

        assert_eq!(conflict.status, ConflictStatus::Identified);
    }

    #[test]
    fn test_agreement() {
        let mut agreement = Agreement::new("agr1", vec!["a1".to_string(), "a2".to_string()]);
        agreement.add_term(AgreementTerm {
            description: "Share resources".to_string(),
            fulfilled: false,
            deadline: None,
        });

        assert_eq!(agreement.terms.len(), 1);
    }

    #[test]
    fn test_coordination_plan() {
        let mut plan = CoordinationPlan::new("plan1", "Complete project");
        plan.add_task(CoordinationTask::new("t1", "Design phase"));
        plan.add_task(CoordinationTask::new("t2", "Implementation"));

        assert_eq!(plan.tasks.len(), 2);
    }

    #[test]
    fn test_group() {
        let mut group = Group::new("g1", "Team Alpha", vec!["a1".to_string(), "a2".to_string()]);
        group.add_member("a3");
        group.leader = Some("a1".to_string());

        assert_eq!(group.members.len(), 3);
        assert_eq!(group.leader, Some("a1".to_string()));
    }

    #[test]
    fn test_interaction_outcome() {
        let outcome = InteractionOutcome::new(true);
        assert!(outcome.successful);
        assert!(outcome.satisfaction > 0.5);
    }

    #[test]
    fn test_trust_calculation() {
        let social = SocialIntelligence::new();
        let trust = social.calculate_trust("nonexistent", "agent2");
        assert_eq!(trust, 0.0);
    }

    #[test]
    fn test_network_metrics() {
        let social = SocialIntelligence::new();
        let metrics = social.get_network_stats();

        assert_eq!(metrics.density, 0.0);
    }
}
