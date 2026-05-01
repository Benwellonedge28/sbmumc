//! Democratic Consensus Module (516)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemocraticConsensus {
    pub dc_id: String,
    pub voting_system: VotingSystem,
    pub quorum_percentage: f64,
    pub deliberation_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VotingSystem {
    DirectDemocracy,
    RepresentativeDemocracy,
    LiquidDemocracy,
    Futarchy,
    QuadraticVoting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub vote_id: String,
    pub voter_id: String,
    pub proposal_id: String,
    pub choice: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub proposal_id: String,
    pub description: String,
    pub votes: Vec<Vote>,
    pub outcome: Option<Outcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Approved { support_percentage: f64 },
    Rejected { rejection_percentage: f64 },
    Tied,
    Pending,
}

impl DemocraticConsensus {
    pub fn new() -> Self {
        Self {
            dc_id: String::from("democratic_consensus_v1"),
            voting_system: VotingSystem::LiquidDemocracy,
            quorum_percentage: 0.5,
            deliberation_depth: 5,
        }
    }

    pub fn tally(&self, proposal: &mut Proposal) -> Outcome {
        let total_weight: f64 = proposal.votes.iter().map(|v| v.weight).sum();
        let approval_weight: f64 = proposal.votes.iter()
            .filter(|v| v.choice == "approve")
            .map(|v| v.weight)
            .sum();
        let percentage = if total_weight > 0.0 { approval_weight / total_weight } else { 0.0 };
        
        proposal.outcome = Some(if percentage >= self.quorum_percentage {
            Outcome::Approved { support_percentage: percentage }
        } else {
            Outcome::Rejected { rejection_percentage: 1.0 - percentage }
        });
        proposal.outcome.clone().unwrap()
    }
}

impl Default for DemocraticConsensus {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_democratic_consensus() {
        let dc = DemocraticConsensus::new();
        assert_eq!(dc.quorum_percentage, 0.5);
    }
}
