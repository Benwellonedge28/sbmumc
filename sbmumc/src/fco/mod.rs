//! Fundamental Computational Ontology (FCO) Module
//!
//! This module implements the foundational computational layer for GSTM INFINITY,
//! providing the smallest possible unit of computation that can accommodate all
//! higher-level units, architectures, frameworks, and paradigms.
//!
//! # Core Concepts
//!
//! ## Mukandara State (MS)
//! The ternary logic foundation (-1, 0, 1) that forms the base of all computations.
//! Named after the Shona concept of "Mukandara" representing balance and transformation.
//!
//! ## Quantum Mukandara State (qmstate)
//! Superposition of -1, 0, and 1 simultaneously, enabling quantum-inspired
//! computational paradigms at the foundational level.
//!
//! ## Infinitism Operations
//! Handling of infinite and infinitesimal values within the computational framework.
//!
//! ## Mukandara Time (mts)
//! The smallest temporal unit beyond Planck time, providing temporal resolution
//! for high-precision computations.
//!
//! # Design Philosophy
//!
//! 1. **Foundational Minimalism**: Every higher-level construct must be derivable from FCO
//! 2. **Universal Accommodation**: Support for all known and future computational paradigms
//! 3. **Ternary Foundation**: Embrace the natural balance of -1, 0, 1
//! 4. **Quantum Readiness**: Native support for superposition and entanglement concepts
//! 5. **Temporal Precision**: Sub-Planck temporal resolution
//!
//! # Architecture Layers
//!
//! Layer 0: Pure Mukandara States (-1, 0, 1)
//! Layer 1: Quantum Mukandara Superpositions
//! Layer 2: Infinitism Values
//! Layer 3: Mukandara Time Operations
//! Layer 4: FCO Expression Language
//! Layer 5: Meta-Compilation Targets

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;

// ============================================================================
// MUKANDARA STATE - The Ternary Logic Foundation
// ============================================================================

/// The fundamental ternary value representing the state of a computation.
///
/// # Mukandara State Values
///
/// - **-1 (Nhaika/Negative)**: Represents negation, absence, false, destruction
/// - **0 (Zera/Zero)**: Represents neutrality, balance, undefined, potential
/// - **1 (Posi/Positive)**: Represents affirmation, presence, true, creation
///
/// The term "Mukandara" comes from Shona philosophy representing the
/// principle of balance and transformation between opposites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub enum MukandaraState {
    /// Negative state (-1): negation, absence, false
    Nhaika = -1,
    /// Neutral state (0): balance, potential, undefined
    Zera = 0,
    /// Positive state (1): affirmation, presence, true
    Posi = 1,
}

impl MukandaraState {
    /// Create a MukandaraState from an i8 value (-1, 0, or 1)
    pub fn from_i8(value: i8) -> Option<Self> {
        match value {
            -1 => Some(MukandaraState::Nhaika),
            0 => Some(MukandaraState::Zera),
            1 => Some(MukandaraState::Posi),
            _ => None,
        }
    }

    /// Convert to i8 representation
    pub fn to_i8(&self) -> i8 {
        match self {
            MukandaraState::Nhaika => -1,
            MukandaraState::Zera => 0,
            MukandaraState::Posi => 1,
        }
    }

    /// Logical AND operation
    pub fn and(&self, other: &MukandaraState) -> MukandaraState {
        match (self, other) {
            (MukandaraState::Posi, MukandaraState::Posi) => MukandaraState::Posi,
            (MukandaraState::Nhaika, _) | (_, MukandaraState::Nhaika) => MukandaraState::Nhaika,
            _ => MukandaraState::Zera,
        }
    }

    /// Logical OR operation
    pub fn or(&self, other: &MukandaraState) -> MukandaraState {
        match (self, other) {
            (MukandaraState::Posi, _) | (_, MukandaraState::Posi) => MukandaraState::Posi,
            (MukandaraState::Nhaika, MukandaraState::Nhaika) => MukandaraState::Nhaika,
            _ => MukandaraState::Zera,
        }
    }

    /// Logical NOT operation
    pub fn not(&self) -> MukandaraState {
        match self {
            MukandaraState::Nhaika => MukandaraState::Posi,
            MukandaraState::Posi => MukandaraState::Nhaika,
            MukandaraState::Zera => MukandaraState::Zera,
        }
    }

    /// Logical XOR operation
    pub fn xor(&self, other: &MukandaraState) -> MukandaraState {
        match (self, other) {
            (MukandaraState::Zera, _) | (_, MukandaraState::Zera) => MukandaraState::Zera,
            (MukandaraState::Nhaika, MukandaraState::Nhaika) => MukandaraState::Nhaika,
            (MukandaraState::Posi, MukandaraState::Posi) => MukandaraState::Nhaika,
            (MukandaraState::Nhaika, MukandaraState::Posi) | (MukandaraState::Posi, MukandaraState::Nhaika) => MukandaraState::Posi,
        }
    }

    /// Get the opposite state
    pub fn opposite(&self) -> MukandaraState {
        match self {
            MukandaraState::Nhaika => MukandaraState::Posi,
            MukandaraState::Posi => MukandaraState::Nhaika,
            MukandaraState::Zera => MukandaraState::Zera,
        }
    }

    /// Check if this is a definite state (not Zera)
    pub fn is_definite(&self) -> bool {
        !matches!(self, MukandaraState::Zera)
    }

    /// Check if this is the neutral state
    pub fn is_neutral(&self) -> bool {
        matches!(self, MukandaraState::Zera)
    }

    /// Get the complementary state (including Zera's self-complementarity)
    pub fn complement(&self) -> MukandaraState {
        *self
    }

    /// Transition function: apply a transformation to get a new state
    pub fn transition(&self, delta: i8) -> MukandaraState {
        let new_val = self.to_i8().saturating_add(delta).clamp(-1, 1);
        MukandaraState::from_i8(new_val).unwrap_or(MukandaraState::Zera)
    }

    /// Weighted vote: combine multiple states with weights
    pub fn weighted_vote(states: &[(MukandaraState, f64)]) -> MukandaraState {
        let sum: f64 = states.iter()
            .map(|(state, weight)| state.to_i8() as f64 * weight)
            .sum();

        if sum > 0.0 {
            MukandaraState::Posi
        } else if sum < 0.0 {
            MukandaraState::Nhaika
        } else {
            MukandaraState::Zera
        }
    }

    /// Get the spiritual name in Shona
    pub fn shona_name(&self) -> &'static str {
        match self {
            MukandaraState::Nhaika => "Nhaika",
            MukandaraState::Zera => "Zera",
            MukandaraState::Posi => "Posi",
        }
    }

    /// Get the philosophical description
    pub fn philosophy(&self) -> &'static str {
        match self {
            MukandaraState::Nhaika => "Absence, potential energy, the void before creation",
            MukandaraState::Zera => "Balance, neutrality, the space between, infinite potential",
            MukandaraState::Posi => "Presence, manifestation, creation in action",
        }
    }
}

impl Default for MukandaraState {
    fn default() -> Self {
        MukandaraState::Zera
    }
}

impl fmt::Display for MukandaraState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MukandaraState::Nhaika => write!(f, "Nhaika (-1)"),
            MukandaraState::Zera => write!(f, "Zera (0)"),
            MukandaraState::Posi => write!(f, "Posi (1)"),
        }
    }
}

// ============================================================================
// QUANTUM MUKANDARA STATE - Superposition States
// ============================================================================

/// Coefficient for a quantum superposition component
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Amplitude {
    /// Real component of the amplitude
    pub re: f64,
    /// Imaginary component of the amplitude
    pub im: f64,
}

impl Amplitude {
    /// Create a new amplitude from real and imaginary parts
    pub fn new(re: f64, im: f64) -> Self {
        Amplitude { re, im }
    }

    /// Create amplitude magnitude (real-only)
    pub fn magnitude(mag: f64) -> Self {
        Amplitude { re: mag, im: 0.0 }
    }

    /// Calculate the probability (magnitude squared)
    pub fn probability(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Normalize the amplitude
    pub fn normalize(&mut self) {
        let mag = self.magnitude().sqrt();
        if mag > 0.0 {
            self.re /= mag;
            self.im /= mag;
        }
    }

    /// Get the magnitude
    pub fn magnitude_value(&self) -> f64 {
        self.probability().sqrt()
    }

    /// Complex conjugate
    pub fn conj(&self) -> Amplitude {
        Amplitude { re: self.re, im: -self.im }
    }
}

impl Add for Amplitude {
    type Output = Amplitude;
    fn add(self, other: Amplitude) -> Amplitude {
        Amplitude {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub for Amplitude {
    type Output = Amplitude;
    fn sub(self, other: Amplitude) -> Amplitude {
        Amplitude {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl Mul for Amplitude {
    type Output = Amplitude;
    fn mul(self, other: Amplitude) -> Amplitude {
        Amplitude {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl Default for Amplitude {
    fn default() -> Self {
        Amplitude { re: 0.0, im: 0.0 }
    }
}

/// Represents a component of a quantum superposition with its state and amplitude
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperpositionComponent {
    /// The base state in the superposition
    pub state: MukandaraState,
    /// The amplitude (probability amplitude) of this component
    pub amplitude: Amplitude,
    /// Optional phase information
    pub phase: f64,
}

impl SuperpositionComponent {
    /// Create a new superposition component
    pub fn new(state: MukandaraState, amplitude: Amplitude) -> Self {
        SuperpositionComponent {
            state,
            amplitude,
            phase: 0.0,
        }
    }

    /// Create with phase
    pub fn with_phase(state: MukandaraState, amplitude: Amplitude, phase: f64) -> Self {
        SuperpositionComponent {
            state,
            amplitude,
            phase,
        }
    }

    /// Get the probability of measuring this component
    pub fn probability(&self) -> f64 {
        self.amplitude.probability()
    }

    /// Get the effective state contribution (amplitude-weighted)
    pub fn contribution(&self) -> f64 {
        self.state.to_i8() as f64 * self.amplitude.magnitude_value()
    }
}

/// Quantum Mukandara State - superposition of -1, 0, and 1 simultaneously.
///
/// This represents the quantum-inspired foundation of FCO, where a single
/// qmstate can exist in multiple classical states simultaneously until
/// observed (measured).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMukandaraState {
    /// The superposition components
    pub components: Vec<SuperpositionComponent>,
    /// Normalization factor
    pub normalization: f64,
    /// Entanglement links to other qmstates
    pub entanglements: Vec<EntanglementLink>,
    /// Coherence measure (0.0 = fully decohered, 1.0 = fully coherent)
    pub coherence: f64,
}

impl QuantumMukandaraState {
    /// Create a pure classical state as a qmstate
    pub fn pure(state: MukandaraState) -> Self {
        QuantumMukandaraState {
            components: vec![SuperpositionComponent::new(state, Amplitude::magnitude(1.0))],
            normalization: 1.0,
            entanglements: Vec::new(),
            coherence: 1.0,
        }
    }

    /// Create a uniform superposition of all states
    pub fn uniform() -> Self {
        let amp = Amplitude::magnitude(1.0 / 3.0_f64.sqrt());
        QuantumMukandaraState {
            components: vec![
                SuperpositionComponent::new(MukandaraState::Nhaika, amp),
                SuperpositionComponent::new(MukandaraState::Zera, amp),
                SuperpositionComponent::new(MukandaraState::Posi, amp),
            ],
            normalization: 1.0,
            entanglements: Vec::new(),
            coherence: 1.0,
        }
    }

    /// Create from specific amplitudes for each state
    pub fn from_amplitudes(
        nhaika: f64,
        zera: f64,
        posi: f64,
    ) -> Result<Self, FcoError> {
        let total = nhaika * nhaika + zera * zera + posi * posi;
        if total <= 0.0 {
            return Err(FcoError::InvalidAmplitude("Total probability must be positive".into()));
        }

        let norm = total.sqrt();
        QuantumMukandaraState {
            components: vec![
                SuperpositionComponent::new(MukandaraState::Nhaika, Amplitude::magnitude(nhaika / norm)),
                SuperpositionComponent::new(MukandaraState::Zera, Amplitude::magnitude(zera / norm)),
                SuperpositionComponent::new(MukandaraState::Posi, Amplitude::magnitude(posi / norm)),
            ],
            normalization: norm,
            entanglements: Vec::new(),
            coherence: 1.0,
        }
    }

    /// Create a maximally entangled state between two qmstates
    pub fn bell_state() -> (QuantumMukandaraState, QuantumMukandaraState) {
        let amp = Amplitude::magnitude(1.0 / 2.0_f64.sqrt());
        let qm1 = QuantumMukandaraState {
            components: vec![
                SuperpositionComponent::new(MukandaraState::Nhaika, amp),
                SuperpositionComponent::new(MukandaraState::Posi, amp),
            ],
            normalization: 1.0,
            entanglements: Vec::new(),
            coherence: 1.0,
        };
        let qm2 = QuantumMukandaraState {
            components: vec![
                SuperpositionComponent::new(MukandaraState::Nhaika, amp),
                SuperpositionComponent::new(MukandaraState::Posi, amp),
            ],
            normalization: 1.0,
            entanglements: Vec::new(),
            coherence: 1.0,
        };
        (qm1, qm2)
    }

    /// Measure (collapse) the quantum state to a classical state
    pub fn measure(&self) -> MukandaraState {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let total_prob: f64 = self.components.iter().map(|c| c.probability()).sum();
        let r = rng.gen::<f64>() * total_prob;

        let mut cumsum = 0.0;
        for component in &self.components {
            cumsum += component.probability();
            if r <= cumsum {
                return component.state;
            }
        }

        // Fallback to most probable
        self.components
            .iter()
            .max_by(|a, b| a.probability().partial_cmp(&b.probability()).unwrap())
            .map(|c| c.state)
            .unwrap_or(MukandaraState::Zera)
    }

    /// Get the expectation value (expected classical state value)
    pub fn expectation(&self) -> f64 {
        self.components
            .iter()
            .map(|c| c.state.to_i8() as f64 * c.probability())
            .sum()
    }

    /// Get the probability distribution over classical states
    pub fn probability_distribution(&self) -> [f64; 3] {
        let mut probs = [0.0; 3];
        for component in &self.components {
            let idx = match component.state {
                MukandaraState::Nhaika => 0,
                MukandaraState::Zera => 1,
                MukandaraState::Posi => 2,
            };
            probs[idx] = component.probability();
        }
        probs
    }

    /// Apply a unitary transformation
    pub fn apply_unitary(&mut self, matrix: &[[f64; 3]; 3]) {
        let probs = self.probability_distribution();
        let new_probs = [
            matrix[0][0] * probs[0] + matrix[0][1] * probs[1] + matrix[0][2] * probs[2],
            matrix[1][0] * probs[0] + matrix[1][1] * probs[1] + matrix[1][2] * probs[2],
            matrix[2][0] * probs[0] + matrix[2][1] * probs[1] + matrix[2][2] * probs[2],
        ];

        let total: f64 = new_probs.iter().sum();
        if total > 0.0 {
            let norm = 1.0 / total.sqrt();
            for (i, component) in self.components.iter_mut().enumerate() {
                component.amplitude = Amplitude::magnitude(new_probs[i].sqrt() * norm);
            }
        }
    }

    /// Create an entanglement link
    pub fn entangle_with(&mut self, other: &mut QuantumMukandaraState, strength: f64) {
        let link = EntanglementLink {
            target_id: format!("{:p}", other),
            strength,
            link_type: EntanglementType::Bell,
        };
        self.entanglements.push(link);

        let link_back = EntanglementLink {
            target_id: format!("{:p}", self),
            strength,
            link_type: EntanglementType::Bell,
        };
        other.entanglements.push(link_back);
    }

    /// Apply decoherence
    pub fn decohere(&mut self, amount: f64) {
        self.coherence = (self.coherence * (1.0 - amount)).max(0.0);
        if self.coherence < 0.01 {
            // Collapse to classical state
            let measured = self.measure();
            self.components = vec![SuperpositionComponent::new(measured, Amplitude::magnitude(1.0))];
        }
    }

    /// Get the von Neumann entropy (measure of quantum uncertainty)
    pub fn entropy(&self) -> f64 {
        use std::f64::consts::LN_2;

        self.components
            .iter()
            .map(|c| {
                let p = c.probability();
                if p > 0.0 { -p * p.log2() } else { 0.0 }
            })
            .sum()
    }

    /// Tensor product with another qmstate
    pub fn tensor_product(&self, other: &QuantumMukandaraState) -> QuantumMukandaraState {
        let mut components = Vec::new();

        for c1 in &self.components {
            for c2 in &other.components {
                let combined_state = match (c1.state, c2.state) {
                    (MukandaraState::Nhaika, _) => CombinedMukandara::NhaikaNhaika,
                    (MukandaraState::Nhaika, MukandaraState::Zera) => CombinedMukandara::NhaikaZera,
                    (MukandaraState::Nhaika, MukandaraState::Posi) => CombinedMukandara::NhaikaPosi,
                    (MukandaraState::Zera, _) => CombinedMukandara::ZeraNhaika,
                    (MukandaraState::Zera, MukandaraState::Zera) => CombinedMukandara::ZeraZera,
                    (MukandaraState::Zera, MukandaraState::Posi) => CombinedMukandara::ZeraPosi,
                    (MukandaraState::Posi, _) => CombinedMukandara::PosiNhaika,
                    (MukandaraState::Posi, MukandaraState::Zera) => CombinedMukandara::PosiZera,
                    (MukandaraState::Posi, MukandaraState::Posi) => CombinedMukandara::PosiPosi,
                };

                components.push(SuperpositionComponent {
                    state: MukandaraState::Zera, // Placeholder - combined states handled differently
                    amplitude: c1.amplitude * c2.amplitude,
                    phase: c1.phase + c2.phase,
                });
            }
        }

        QuantumMukandaraState {
            components,
            normalization: self.normalization * other.normalization,
            entanglements: Vec::new(),
            coherence: self.coherence.min(other.coherence),
        }
    }

    /// Check if this is a classical (non-superposition) state
    pub fn is_classical(&self) -> bool {
        self.components.iter().filter(|c| c.probability() > 0.0).count() == 1
    }

    /// Get the classical state if this is classical
    pub fn as_classical(&self) -> Option<MukandaraState> {
        if self.is_classical() {
            self.components.iter().find(|c| c.probability() > 0.0).map(|c| c.state)
        } else {
            None
        }
    }
}

impl Default for QuantumMukandaraState {
    fn default() -> Self {
        QuantumMukandaraState::pure(MukandaraState::Zera)
    }
}

/// Combined states for tensor products
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombinedMukandara {
    NhaikaNhaika,
    NhaikaZera,
    NhaikaPosi,
    ZeraNhaika,
    ZeraZera,
    ZeraPosi,
    PosiNhaika,
    PosiZera,
    PosiPosi,
}

impl CombinedMukandara {
    /// Project to a single MukandaraState (partial trace)
    pub fn project(&self, trace_out: TraceTarget) -> MukandaraState {
        match trace_out {
            TraceTarget::First => match self {
                CombinedMukandara::NhaikaNhaika | CombinedMukandara::NhaikaZera | CombinedMukandara::NhaikaPosi => MukandaraState::Nhaika,
                CombinedMukandara::ZeraNhaika | CombinedMukandara::ZeraZera | CombinedMukandara::ZeraPosi => MukandaraState::Zera,
                CombinedMukandara::PosiNhaika | CombinedMukandara::PosiZera | CombinedMukandara::PosiPosi => MukandaraState::Posi,
            },
            TraceTarget::Second => match self {
                CombinedMukandara::NhaikaNhaika | CombinedMukandara::ZeraNhaika | CombinedMukandara::PosiNhaika => MukandaraState::Nhaika,
                CombinedMukandara::NhaikaZera | CombinedMukandara::ZeraZera | CombinedMukandara::PosiZera => MukandaraState::Zera,
                CombinedMukandara::NhaikaPosi | CombinedMukandara::ZeraPosi | CombinedMukandara::PosiPosi => MukandaraState::Posi,
            },
        }
    }
}

impl fmt::Display for CombinedMukandara {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CombinedMukandara::NhaikaNhaika => write!(f, "(-1, -1)"),
            CombinedMukandara::NhaikaZera => write!(f, "(-1, 0)"),
            CombinedMukandara::NhaikaPosi => write!(f, "(-1, +1)"),
            CombinedMukandara::ZeraNhaika => write!(f, "(0, -1)"),
            CombinedMukandara::ZeraZera => write!(f, "(0, 0)"),
            CombinedMukandara::ZeraPosi => write!(f, "(0, +1)"),
            CombinedMukandara::PosiNhaika => write!(f, "(+1, -1)"),
            CombinedMukandara::PosiZera => write!(f, "(+1, 0)"),
            CombinedMukandara::PosiPosi => write!(f, "(+1, +1)"),
        }
    }
}

/// Target for partial trace operation
#[derive(Debug, Clone, Copy)]
pub enum TraceTarget {
    First,
    Second,
}

/// Entanglement link to another qmstate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntanglementLink {
    /// Target qmstate identifier
    pub target_id: String,
    /// Entanglement strength (0.0 to 1.0)
    pub strength: f64,
    /// Type of entanglement
    pub link_type: EntanglementType,
}

/// Types of quantum entanglement
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EntanglementType {
    /// Bell-type entanglement (maximally entangled)
    Bell,
    /// Partial entanglement
    Partial,
    /// Cat-state entanglement (GHZ-like)
    CatState,
    /// Cluster state entanglement
    Cluster,
}

// ============================================================================
// INFINITISM - Handling Infinite Values
// ============================================================================

/// Extended number type supporting infinities and infinitesimals
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Infinitism {
    /// Negative infinity
    NegInfinity,
    /// Large negative finite number
    LargeNegative(f64),
    /// Standard finite number
    Finite(f64),
    /// Small positive finite number (infinitesimal)
    SmallPositive(f64),
    /// Exactly zero
    Zero,
    /// Small negative finite number (negative infinitesimal)
    SmallNegative(f64),
    /// Large positive finite number
    LargePositive(f64),
    /// Positive infinity
    PosInfinity,
    /// Undefined/indeterminate form
    Undefined,
    /// Complex infinity (infinite magnitude, undefined phase)
    ComplexInfinity,
}

impl Infinitism {
    /// Create from a standard f64, mapping infinities appropriately
    pub fn from_f64(value: f64) -> Self {
        if value.is_infinite() {
            if value.is_sign_negative() {
                Infinitism::NegInfinity
            } else {
                Infinitism::PosInfinity
            }
        } else if value.is_nan() {
            Infinitism::Undefined
        } else if value.abs() < 1e-300 {
            if value.is_sign_positive() {
                Infinitism::SmallPositive(value)
            } else {
                Infinitism::SmallNegative(value)
            }
        } else {
            Infinitism::Finite(value)
        }
    }

    /// Convert to a representative f64 (may lose information)
    pub fn to_f64(&self) -> f64 {
        match self {
            Infinitism::NegInfinity => f64::NEG_INFINITY,
            Infinitism::LargeNegative(v) => *v,
            Infinitism::Finite(v) => *v,
            Infinitism::SmallPositive(v) => *v,
            Infinitism::Zero => 0.0,
            Infinitism::SmallNegative(v) => *v,
            Infinitism::LargePositive(v) => *v,
            Infinitism::PosInfinity => f64::POS_INFINITY,
            Infinitism::Undefined | Infinitism::ComplexInfinity => f64::NAN,
        }
    }

    /// Check if this represents infinity
    pub fn is_infinite(&self) -> bool {
        matches!(self, Infinitism::NegInfinity | Infinitism::PosInfinity | Infinitism::ComplexInfinity)
    }

    /// Check if this is finite
    pub fn is_finite(&self) -> bool {
        !self.is_infinite() && !matches!(self, Infinitism::Undefined)
    }

    /// Check if this is zero
    pub fn is_zero(&self) -> bool {
        matches!(self, Infinitism::Zero)
    }

    /// Check if this is infinitesimal
    pub fn is_infinitesimal(&self) -> bool {
        matches!(self, Infinitism::SmallPositive(_) | Infinitism::SmallNegative(_))
    }

    /// Get the sign
    pub fn sign(&self) -> MukandaraState {
        match self {
            Infinitism::NegInfinity | Infinitism::LargeNegative(_) | Infinitism::SmallNegative(_) => MukandaraState::Nhaika,
            Infinitism::Zero => MukandaraState::Zera,
            Infinitism::PosInfinity | Infinitism::LargePositive(_) | Infinitism::SmallPositive(_) => MukandaraState::Posi,
            Infinitism::Undefined | Infinitism::ComplexInfinity => MukandaraState::Zera,
        }
    }

    /// Add two infinitism values
    pub fn add(&self, other: &Infinitism) -> Infinitism {
        use Infinitism::*;

        match (self, other) {
            // Infinity + anything = infinity (with sign)
            (PosInfinity, _) | (_, PosInfinity) => PosInfinity,
            (NegInfinity, _) | (_, NegInfinity) => NegInfinity,

            // Finite + infinity
            (PosInfinity, _) | (_, PosInfinity) => PosInfinity,
            (NegInfinity, _) | (_, NegInfinity) => NegInfinity,

            // Undefined + anything = undefined
            (Undefined, _) | (_, Undefined) => Undefined,
            (ComplexInfinity, _) | (_, ComplexInfinity) => ComplexInfinity,

            // Zero cases
            (Zero, _) => other.clone(),
            (_, Zero) => self.clone(),

            // Finite + finite
            (Finite(a), Finite(b)) => Infinitism::from_f64(a + b),

            // Handle infinitesimals
            (SmallPositive(a), SmallPositive(b)) | (LargePositive(a), LargePositive(b)) => Infinitism::from_f64(a + b),
            (SmallNegative(a), SmallNegative(b)) | (LargeNegative(a), LargeNegative(b)) => Infinitism::from_f64(a + b),

            // Mixed sign
            (SmallPositive(a), SmallNegative(b)) | (SmallNegative(a), SmallPositive(b)) => Infinitism::from_f64(a + b),
            (LargePositive(a), LargeNegative(b)) | (LargeNegative(a), LargePositive(b)) => Infinitism::from_f64(a + b),

            _ => Undefined,
        }
    }

    /// Multiply two infinitism values
    pub fn multiply(&self, other: &Infinitism) -> Infinitism {
        use Infinitism::*;

        match (self, other) {
            // Zero * anything = zero (unless infinity)
            (Zero, _) | (_, Zero) => Zero,

            // Infinity * infinity = infinity (with sign)
            (PosInfinity, PosInfinity) | (NegInfinity, NegInfinity) => PosInfinity,
            (PosInfinity, NegInfinity) | (NegInfinity, PosInfinity) => NegInfinity,

            // Infinity * anything = infinity
            (PosInfinity, _) | (_, PosInfinity) => PosInfinity,
            (NegInfinity, _) | (_, NegInfinity) => NegInfinity,

            // Undefined
            (Undefined, _) | (_, Undefined) => Undefined,
            (ComplexInfinity, ComplexInfinity) => ComplexInfinity,
            (ComplexInfinity, _) | (_, ComplexInfinity) => ComplexInfinity,

            // Finite * finite
            (Finite(a), Finite(b)) => Infinitism::from_f64(a * b),

            // Handle infinitesimals
            (SmallPositive(a), SmallPositive(b)) => Infinitism::from_f64(a * b),
            (SmallNegative(a), SmallNegative(b)) => Infinitism::from_f64(a * b),
            (SmallPositive(a), SmallNegative(b)) => Infinitism::from_f64(a * b),
            (SmallNegative(a), SmallPositive(b)) => Infinitism::from_f64(a * b),

            // Large * small = finite
            (LargePositive(a), SmallPositive(b)) | (SmallPositive(a), LargePositive(b))
            | (LargeNegative(a), SmallNegative(b)) | (SmallNegative(a), LargeNegative(b))
            | (LargePositive(a), SmallNegative(b)) | (SmallPositive(a), LargeNegative(b))
            | (LargeNegative(a), SmallPositive(b)) | (SmallNegative(a), LargePositive(b))
                => Infinitism::from_f64(a * b),

            _ => Undefined,
        }
    }

    /// Divide two infinitism values
    pub fn divide(&self, other: &Infinitism) -> Infinitism {
        use Infinitism::*;

        match (self, other) {
            // Zero / anything = zero (unless 0/0)
            (Zero, _) if !other.is_zero() => Zero,

            // Anything / infinity = zero
            (_, PosInfinity) | (_, NegInfinity) => Zero,

            // Anything / zero = infinity
            (_, Zero) => ComplexInfinity,

            // Infinity / infinity = undefined
            (PosInfinity, PosInfinity) | (NegInfinity, NegInfinity) => Undefined,
            (PosInfinity, NegInfinity) | (NegInfinity, PosInfinity) => Undefined,

            // Undefined
            (Undefined, _) | (_, Undefined) => Undefined,
            (ComplexInfinity, _) | (_, ComplexInfinity) => ComplexInfinity,

            // Finite / finite
            (Finite(a), Finite(b)) if *b != 0.0 => Infinitism::from_f64(a / b),

            // Handle infinitesimals
            (SmallPositive(a), SmallPositive(b)) => Infinitism::from_f64(a / b),
            (SmallNegative(a), SmallNegative(b)) => Infinitism::from_f64(a / b),
            (SmallPositive(a), SmallNegative(b)) => Infinitism::from_f64(a / b),
            (SmallNegative(a), SmallPositive(b)) => Infinitism::from_f64(a / b),

            _ => Undefined,
        }
    }

    /// Reciprocal
    pub fn reciprocal(&self) -> Infinitism {
        use Infinitism::*;

        match self {
            Zero => ComplexInfinity,
            SmallPositive(v) => LargePositive(1.0 / v),
            SmallNegative(v) => LargeNegative(1.0 / v),
            LargePositive(v) => SmallPositive(1.0 / v),
            LargeNegative(v) => SmallNegative(1.0 / v),
            Finite(v) if *v != 0.0 => Infinitism::from_f64(1.0 / v),
            PosInfinity => Zero,
            NegInfinity => Zero,
            _ => Undefined,
        }
    }

    /// Get magnitude order (log scale)
    pub fn order(&self) -> i32 {
        use Infinitism::*;

        match self {
            NegInfinity | PosInfinity | ComplexInfinity => i32::MAX,
            LargeNegative(v) | LargePositive(v) => (v.abs().log10().floor() as i32).max(10),
            SmallNegative(v) | SmallPositive(v) => (v.abs().log10().floor() as i32).min(-10),
            Zero => i32::MIN,
            Finite(v) => (v.abs().log10().floor() as i32),
            Undefined => 0,
        }
    }
}

impl Clone for Infinitism {
    fn clone(&self) -> Self {
        match self {
            Infinitism::LargeNegative(v) => Infinitism::LargeNegative(*v),
            Infinitism::Finite(v) => Infinitism::Finite(*v),
            Infinitism::SmallPositive(v) => Infinitism::SmallPositive(*v),
            Infinitism::SmallNegative(v) => Infinitism::SmallNegative(*v),
            Infinitism::LargePositive(v) => Infinitism::LargePositive(*v),
            other => *other,
        }
    }
}

impl Default for Infinitism {
    fn default() -> Self {
        Infinitism::Zero
    }
}

impl fmt::Display for Infinitism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Infinitism::NegInfinity => write!(f, "-∞"),
            Infinitism::LargeNegative(v) => write!(f, "-L({:e})", v),
            Infinitism::Finite(v) => write!(f, "{:e}", v),
            Infinitism::SmallPositive(v) => write!(f, "+ε({:e})", v),
            Infinitism::Zero => write!(f, "0"),
            Infinitism::SmallNegative(v) => write!(f, "-ε({:e})", v),
            Infinitism::LargePositive(v) => write!(f, "+L({:e})", v),
            Infinitism::PosInfinity => write!(f, "+∞"),
            Infinitism::Undefined => write!(f, "undef"),
            Infinitism::ComplexInfinity => write!(f, "∞̃"),
        }
    }
}

// ============================================================================
// MUKANDARA TIME - Sub-Planck Temporal Resolution
// ============================================================================

/// The smallest unit of time in the Mukandara framework.
///
/// Mukandara Time (mts) is designed to provide temporal resolution
/// beyond Planck time (5.39e-44 seconds), enabling true infinitesimal
/// time operations for high-precision computational modeling.
///
/// 1 mts = 10^-100 seconds (configurable precision)
pub type MukandaraTime = Infinitism;

/// Time direction for temporal operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeDirection {
    /// Forward in time
    Forward,
    /// Backward in time
    Backward,
    /// Stationary
    Stationary,
    /// Cyclical
    Cyclical,
}

/// Represents a point in Mukandara Time
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimePoint {
    /// The time value in mts units
    pub value: MukandaraTime,
    /// Direction of time flow
    pub direction: TimeDirection,
    /// Uncertainty in time measurement
    pub uncertainty: Option<Infinitism>,
}

impl TimePoint {
    /// Create a new time point
    pub fn new(value: MukandaraTime) -> Self {
        TimePoint {
            value,
            direction: TimeDirection::Forward,
            uncertainty: None,
        }
    }

    /// Create from standard f64 seconds
    pub fn from_seconds(seconds: f64) -> Self {
        TimePoint::new(Infinitism::from_f64(seconds))
    }

    /// Create from Planck time units
    pub fn from_planck_units(units: Infinitism) -> Self {
        let planck_time = Infinitism::SmallPositive(5.39e-44_f64);
        TimePoint::new(planck_time.multiply(&units))
    }

    /// Create now (current moment)
    pub fn now() -> Self {
        TimePoint::from_seconds(0.0)
    }

    /// Create at the beginning of time
    pub fn origin() -> Self {
        TimePoint::new(Infinitism::Zero)
    }

    /// Create at infinity
    pub fn infinite(direction: TimeDirection) -> Self {
        TimePoint {
            value: if direction == TimeDirection::Forward {
                Infinitism::PosInfinity
            } else {
                Infinitism::NegInfinity
            },
            direction,
            uncertainty: None,
        }
    }

    /// Add duration to get a new time point
    pub fn add_duration(&self, duration: &MukandaraTime) -> TimePoint {
        TimePoint {
            value: self.value.add(duration),
            direction: self.direction,
            uncertainty: self.uncertainty,
        }
    }

    /// Subtract duration
    pub fn subtract_duration(&self, duration: &MukandaraTime) -> TimePoint {
        TimePoint {
            value: self.value.add(&duration.multiply(&Infinitism::from_f64(-1.0))),
            direction: self.direction,
            uncertainty: self.uncertainty,
        }
    }

    /// Get the interval between two time points
    pub fn interval_to(&self, other: &TimePoint) -> MukandaraTime {
        other.value.add(&self.value.multiply(&Infinitism::from_f64(-1.0)))
    }

    /// Check if this is before another time point
    pub fn is_before(&self, other: &TimePoint) -> bool {
        let interval = self.interval_to(other);
        interval.sign() == MukandaraState::Nhaika
    }

    /// Check if this is after another time point
    pub fn is_after(&self, other: &TimePoint) -> bool {
        let interval = self.interval_to(other);
        interval.sign() == MukandaraState::Posi
    }

    /// Convert to seconds (may lose precision)
    pub fn to_seconds(&self) -> f64 {
        self.value.to_f64()
    }

    /// Get duration until another time point
    pub fn duration_until(&self, other: &TimePoint) -> Option<MukandaraTime> {
        if self.is_before(other) {
            Some(self.interval_to(other))
        } else {
            None
        }
    }
}

impl Default for TimePoint {
    fn default() -> Self {
        TimePoint::now()
    }
}

impl fmt::Display for TimePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t={} ({:?})", self.value, self.direction)
    }
}

// ============================================================================
// FCO EXPRESSION LANGUAGE
// ============================================================================

/// The fundamental unit of FCO computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FcoUnit {
    /// Unique identifier
    pub id: u64,
    /// The state value
    pub state: QuantumMukandaraState,
    /// Time of creation
    pub created_at: TimePoint,
    /// Optional metadata
    pub metadata: FcoMetadata,
}

/// Metadata for FCO units
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FcoMetadata {
    /// Type identifier
    pub type_id: Option<String>,
    /// Semantic meaning
    pub meaning: Option<String>,
    /// Source location
    pub source: Option<String>,
    /// Custom attributes
    pub attributes: std::collections::HashMap<String, String>,
}

impl FcoMetadata {
    pub fn new() -> Self {
        FcoMetadata::default()
    }

    pub fn with_type(mut self, type_id: &str) -> Self {
        self.type_id = Some(type_id.to_string());
        self
    }

    pub fn with_meaning(mut self, meaning: &str) -> Self {
        self.meaning = Some(meaning.to_string());
        self
    }

    pub fn with_attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
}

/// Operations that can be performed on FCO units
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FcoOperation {
    /// Identity operation
    Identity,
    /// Negation
    Negate,
    /// Complement
    Complement,
    /// AND
    And,
    /// OR
    Or,
    /// XOR
    Xor,
    /// NAND
    Nand,
    /// NOR
    Nor,
    /// Transfer (copy)
    Transfer,
    /// Measure (collapse quantum state)
    Measure,
    /// Entangle
    Entangle,
    /// Project to specific state
    Project(MukandaraState),
}

/// Result of an FCO operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FcoResult {
    /// Output FCO unit
    pub output: FcoUnit,
    /// Operation that produced this result
    pub operation: FcoOperation,
    /// Execution time
    pub execution_time: MukandaraTime,
    /// Success flag
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

impl FcoResult {
    pub fn success(output: FcoUnit, op: FcoOperation, time: MukandaraTime) -> Self {
        FcoResult {
            output,
            operation: op,
            execution_time: time,
            success: true,
            error: None,
        }
    }

    pub fn failure(op: FcoOperation, error: &str) -> Self {
        FcoResult {
            output: FcoUnit::default(),
            operation: op,
            execution_time: Infinitism::Zero,
            success: false,
            error: Some(error.to_string()),
        }
    }
}

impl Default for FcoUnit {
    fn default() -> Self {
        FcoUnit {
            id: 0,
            state: QuantumMukandaraState::default(),
            created_at: TimePoint::now(),
            metadata: FcoMetadata::new(),
        }
    }
}

// ============================================================================
// FCO ENGINE - Core Execution Engine
// ============================================================================

/// The FCO engine provides the core computational foundation
#[derive(Debug)]
pub struct FcoEngine {
    /// Current execution context
    context: FcoContext,
    /// Operation history
    history: Vec<FcoResult>,
    /// Statistics
    stats: FcoStats,
}

/// FCO execution context
#[derive(Debug, Clone, Default)]
pub struct FcoContext {
    /// Current time
    pub time: TimePoint,
    /// Active state
    pub state: QuantumMukandaraState,
    /// Local variables
    pub variables: std::collections::HashMap<String, FcoUnit>,
    /// Call stack depth
    pub stack_depth: usize,
}

/// FCO statistics
#[derive(Debug, Clone, Default)]
pub struct FcoStats {
    /// Total operations performed
    pub total_operations: u64,
    /// Operations by type
    pub operations_by_type: std::collections::HashMap<FcoOperation, u64>,
    /// Total execution time
    pub total_execution_time: MukandaraTime,
    /// Errors encountered
    pub errors: u64,
}

impl FcoEngine {
    /// Create a new FCO engine
    pub fn new() -> Self {
        FcoEngine {
            context: FcoContext::default(),
            history: Vec::new(),
            stats: FcoStats::default(),
        }
    }

    /// Initialize with a specific state
    pub fn with_state(state: QuantumMukandaraState) -> Self {
        let mut engine = FcoEngine::new();
        engine.context.state = state;
        engine
    }

    /// Execute an operation
    pub fn execute(&mut self, operation: FcoOperation, input: Option<&FcoUnit>) -> FcoResult {
        let start_time = TimePoint::now();

        let result = match operation {
            FcoOperation::Identity => self.execute_identity(input),
            FcoOperation::Negate => self.execute_negate(input),
            FcoOperation::Complement => self.execute_complement(input),
            FcoOperation::And => self.execute_binary_op(input, |a, b| a.and(&b)),
            FcoOperation::Or => self.execute_binary_op(input, |a, b| a.or(&b)),
            FcoOperation::Xor => self.execute_binary_op(input, |a, b| a.xor(&b)),
            FcoOperation::Nand => self.execute_binary_op(input, |a, b| a.and(&b).not()),
            FcoOperation::Nor => self.execute_binary_op(input, |a, b| a.or(&b).not()),
            FcoOperation::Transfer => self.execute_transfer(input),
            FcoOperation::Measure => self.execute_measure(input),
            FcoOperation::Entangle => self.execute_entangle(input),
            FcoOperation::Project(state) => self.execute_project(input, state),
        };

        let end_time = TimePoint::now();
        let execution_time = start_time.interval_to(&end_time);

        // Update statistics
        self.stats.total_operations += 1;
        *self.stats.operations_by_type.entry(operation).or_insert(0) += 1;
        self.stats.total_execution_time = self.stats.total_execution_time.add(&execution_time);
        if !result.success {
            self.stats.errors += 1;
        }

        // Record in history
        let mut result_with_time = result;
        result_with_time.execution_time = execution_time;
        self.history.push(result_with_time.clone());

        result_with_time
    }

    fn execute_identity(&mut self, input: Option<&FcoUnit>) -> FcoResult {
        let state = input.map(|u| u.state.clone()).unwrap_or(self.context.state.clone());
        let unit = FcoUnit {
            id: self.generate_id(),
            state,
            created_at: TimePoint::now(),
            metadata: FcoMetadata::new(),
        };
        FcoResult::success(unit, FcoOperation::Identity, Infinitism::Zero)
    }

    fn execute_negate(&mut self, input: Option<&FcoUnit>) -> FcoResult {
        let state = input.map(|u| u.state.clone()).unwrap_or(self.context.state.clone());
        let measured = state.measure();
        let negated = measured.not();
        let new_state = QuantumMukandaraState::pure(negated);
        let unit = FcoUnit {
            id: self.generate_id(),
            state: new_state,
            created_at: TimePoint::now(),
            metadata: FcoMetadata::new().with_type("negated"),
        };
        FcoResult::success(unit, FcoOperation::Negate, Infinitism::SmallPositive(1e-50))
    }

    fn execute_complement(&mut self, input: Option<&FcoUnit>) -> FcoResult {
        let state = input.map(|u| u.state.clone()).unwrap_or(self.context.state.clone());
        let measured = state.measure();
        let complemented = measured.opposite();
        let new_state = QuantumMukandaraState::pure(complemented);
        let unit = FcoUnit {
            id: self.generate_id(),
            state: new_state,
            created_at: TimePoint::now(),
            metadata: FcoMetadata::new().with_type("complement"),
        };
        FcoResult::success(unit, FcoOperation::Complement, Infinitism::SmallPositive(1e-50))
    }

    fn execute_binary_op<F>(&mut self, input: Option<&FcoUnit>, op: F) -> FcoResult
    where
        F: Fn(MukandaraState, MukandaraState) -> MukandaraState,
    {
        // For simplicity, use context state and optional input
        let state1 = input.map(|u| u.state.clone()).unwrap_or(self.context.state.clone());
        let state2 = self.context.state.clone();

        let m1 = state1.measure();
        let m2 = state2.measure();
        let result = op(m1, m2);
        let new_state = QuantumMukandaraState::pure(result);

        let unit = FcoUnit {
            id: self.generate_id(),
            state: new_state,
            created_at: TimePoint::now(),
            metadata: FcoMetadata::new(),
        };
        FcoResult::success(unit, FcoOperation::And, Infinitism::SmallPositive(1e-50))
    }

    fn execute_transfer(&mut self, input: Option<&FcoUnit>) -> FcoResult {
        if let Some(unit) = input {
            let transferred = FcoUnit {
                id: self.generate_id(),
                state: unit.state.clone(),
                created_at: TimePoint::now(),
                metadata: unit.metadata.clone(),
            };
            FcoResult::success(transferred, FcoOperation::Transfer, Infinitism::SmallPositive(1e-50))
        } else {
            FcoResult::failure(FcoOperation::Transfer, "No input provided")
        }
    }

    fn execute_measure(&mut self, input: Option<&FcoUnit>) -> FcoResult {
        let state = input.map(|u| u.state.clone()).unwrap_or(self.context.state.clone());
        let measured = state.measure();
        let new_state = QuantumMukandaraState::pure(measured);

        // Update context
        self.context.state = new_state.clone();

        let unit = FcoUnit {
            id: self.generate_id(),
            state: new_state,
            created_at: TimePoint::now(),
            metadata: FcoMetadata::new()
                .with_type("measured")
                .with_meaning(measured.shona_name()),
        };
        FcoResult::success(unit, FcoOperation::Measure, Infinitism::SmallPositive(1e-50))
    }

    fn execute_entangle(&mut self, input: Option<&FcoUnit>) -> FcoResult {
        let mut state = input.map(|u| u.state.clone()).unwrap_or(self.context.state.clone());
        let mut other = QuantumMukandaraState::uniform();

        state.entangle_with(&mut other, 1.0);

        let unit = FcoUnit {
            id: self.generate_id(),
            state: state.clone(),
            created_at: TimePoint::now(),
            metadata: FcoMetadata::new()
                .with_type("entangled")
                .with_meaning("Quantum entangled state"),
        };

        self.context.state = state;
        FcoResult::success(unit, FcoOperation::Entangle, Infinitism::SmallPositive(1e-50))
    }

    fn execute_project(&mut self, input: Option<&FcoUnit>, target: MukandaraState) -> FcoResult {
        let state = input.map(|u| u.state.clone()).unwrap_or(self.context.state.clone());
        let new_state = QuantumMukandaraState::pure(target);

        let unit = FcoUnit {
            id: self.generate_id(),
            state: new_state.clone(),
            created_at: TimePoint::now(),
            metadata: FcoMetadata::new()
                .with_type("projected")
                .with_meaning(target.shona_name()),
        };

        self.context.state = new_state;
        FcoResult::success(unit, FcoOperation::Project(target), Infinitism::SmallPositive(1e-50))
    }

    fn generate_id(&mut self) -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    /// Get the current context
    pub fn context(&self) -> &FcoContext {
        &self.context
    }

    /// Get statistics
    pub fn stats(&self) -> &FcoStats {
        &self.stats
    }

    /// Get operation history
    pub fn history(&self) -> &[FcoResult] {
        &self.history
    }

    /// Reset the engine
    pub fn reset(&mut self) {
        self.context = FcoContext::default();
        self.history.clear();
        self.stats = FcoStats::default();
    }

    /// Compile a sequence of operations
    pub fn compile_sequence(&self, operations: &[FcoOperation]) -> FcoProgram {
        FcoProgram {
            operations: operations.to_vec(),
            metadata: FcoMetadata::new(),
        }
    }

    /// Execute a program
    pub fn execute_program(&mut self, program: &FcoProgram) -> Vec<FcoResult> {
        let mut results = Vec::new();
        for op in &program.operations {
            let result = self.execute(*op, None);
            results.push(result);
        }
        results
    }
}

impl Default for FcoEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// A compiled FCO program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FcoProgram {
    /// Operations in sequence
    pub operations: Vec<FcoOperation>,
    /// Program metadata
    pub metadata: FcoMetadata,
}

impl FcoProgram {
    /// Create a new empty program
    pub fn new() -> Self {
        FcoProgram {
            operations: Vec::new(),
            metadata: FcoMetadata::new(),
        }
    }

    /// Add an operation
    pub fn add_operation(&mut self, op: FcoOperation) {
        self.operations.push(op);
    }

    /// Get the number of operations
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

impl Default for FcoProgram {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Errors that can occur in FCO operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FcoError {
    /// Invalid state value
    InvalidState(String),
    /// Invalid amplitude
    InvalidAmplitude(String),
    /// Division by zero
    DivisionByZero,
    /// Undefined operation
    UndefinedOperation(String),
    /// Stack overflow
    StackOverflow,
    /// Invalid program
    InvalidProgram(String),
    /// Execution error
    ExecutionError(String),
}

impl fmt::Display for FcoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FcoError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            FcoError::InvalidAmplitude(msg) => write!(f, "Invalid amplitude: {}", msg),
            FcoError::DivisionByZero => write!(f, "Division by zero"),
            FcoError::UndefinedOperation(msg) => write!(f, "Undefined operation: {}", msg),
            FcoError::StackOverflow => write!(f, "Stack overflow in FCO execution"),
            FcoError::InvalidProgram(msg) => write!(f, "Invalid program: {}", msg),
            FcoError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
        }
    }
}

impl std::error::Error for FcoError {}

// ============================================================================
// FCO TYPE SYSTEM
// ============================================================================

/// FCO type system for compile-time type checking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FcoType {
    /// Boolean-like ternary type
    TriBool,
    /// Integer type
    Int,
    /// Real number type
    Real,
    /// Complex number type
    Complex,
    /// Quantum state type
    QuantumState,
    /// Unit type
    Unit,
    /// Error type
    Error,
    /// Polymorphic type
    Poly,
}

impl FcoType {
    /// Check if two types are compatible
    pub fn compatible(&self, other: &FcoType) -> bool {
        matches!(
            (self, other),
            (FcoType::Poly, _) | (_, FcoType::Poly) | (FcoType::Error, _) | (_, FcoType::Error)
        ) || self == other
    }

    /// Get the result type of an operation
    pub fn result_type(&self, op: &FcoOperation, rhs: Option<&FcoType>) -> FcoType {
        match op {
            FcoOperation::Identity | FcoOperation::Transfer | FcoOperation::Measure | FcoOperation::Project(_) => {
                *self
            }
            FcoOperation::Negate | FcoOperation::Complement => *self,
            FcoOperation::And | FcoOperation::Or | FcoOperation::Xor | FcoOperation::Nand | FcoOperation::Nor => {
                FcoType::TriBool
            }
            FcoOperation::Entangle => FcoType::QuantumState,
        }
    }
}

// ============================================================================
// FCO COMPILER - Compile to FCO Programs
// ============================================================================

/// FCO compiler for translating to FCO programs
#[derive(Debug, Default)]
pub struct FcoCompiler {
    /// Current program under construction
    current_program: Option<FcoProgram>,
    /// Defined macros
    macros: std::collections::HashMap<String, Vec<FcoOperation>>,
}

impl FcoCompiler {
    /// Create a new compiler
    pub fn new() -> Self {
        FcoCompiler::default()
    }

    /// Start a new program
    pub fn start_program(&mut self) {
        self.current_program = Some(FcoProgram::new());
    }

    /// Add an operation to the current program
    pub fn add_operation(&mut self, op: FcoOperation) -> Result<(), FcoError> {
        match &mut self.current_program {
            Some(program) => {
                program.add_operation(op);
                Ok(())
            }
            None => Err(FcoError::InvalidProgram("No program started".into())),
        }
    }

    /// End the current program
    pub fn end_program(&mut self) -> Result<FcoProgram, FcoError> {
        self.current_program.take().ok_or_else(|| {
            FcoError::InvalidProgram("No program to end".into())
        })
    }

    /// Define a macro
    pub fn define_macro(&mut self, name: &str, operations: Vec<FcoOperation>) {
        self.macros.insert(name.to_string(), operations);
    }

    /// Expand a macro
    pub fn expand_macro(&mut self, name: &str) -> Result<Vec<FcoOperation>, FcoError> {
        self.macros.get(name).cloned().ok_or_else(|| {
            FcoError::UndefinedOperation(format!("Macro '{}' not found", name))
        })
    }

    /// Compile a high-level expression to FCO operations
    pub fn compile_expression(&mut self, _expr: &str) -> Result<Vec<FcoOperation>, FcoError> {
        // Simplified expression compiler
        // In a full implementation, this would parse the expression
        Ok(vec![FcoOperation::Identity])
    }

    /// Optimize a program
    pub fn optimize(&self, program: &FcoProgram) -> FcoProgram {
        // Remove consecutive Identity operations
        let mut optimized = Vec::new();
        let mut last_was_identity = false;

        for op in &program.operations {
            if matches!(op, FcoOperation::Identity) {
                if !last_was_identity {
                    optimized.push(*op);
                    last_was_identity = true;
                }
            } else {
                optimized.push(*op);
                last_was_identity = false;
            }
        }

        FcoProgram {
            operations: optimized,
            metadata: program.metadata.clone(),
        }
    }
}

// ============================================================================
// FCO TRANSFORMATIONS - Meta-level Operations
// ============================================================================

/// Transformation types for meta-compilation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FcoTransformation {
    /// Normalize to classical states
    ClassicalNormalize,
    /// Quantumize to superposition
    Quantumize,
    /// Entangle multiple states
    CreateEntanglement,
    /// Apply decoherence
    Decoherence(f64),
    /// Amplify to infinitism
    Amplify,
}

/// FCO transformation engine
pub struct FcoTransformer;

impl FcoTransformer {
    /// Transform a classical state to quantum
    pub fn to_quantum(state: MukandaraState) -> QuantumMukandaraState {
        QuantumMukandaraState::pure(state)
    }

    /// Transform quantum to classical by measurement
    pub fn to_classical(state: &QuantumMukandaraState) -> MukandaraState {
        state.measure()
    }

    /// Apply a transformation to an FCO unit
    pub fn transform(unit: &FcoUnit, transformation: FcoTransformation) -> FcoUnit {
        let mut new_state = unit.state.clone();

        match transformation {
            FcoTransformation::ClassicalNormalize => {
                let measured = new_state.measure();
                new_state = QuantumMukandaraState::pure(measured);
            }
            FcoTransformation::Quantumize => {
                if new_state.is_classical() {
                    new_state = QuantumMukandaraState::uniform();
                }
            }
            FcoTransformation::CreateEntanglement => {
                let mut other = QuantumMukandaraState::uniform();
                new_state.entangle_with(&mut other, 1.0);
            }
            FcoTransformation::Decoherence(amount) => {
                new_state.decohere(amount);
            }
            FcoTransformation::Amplify => {
                // Transition to infinitism-scale computation
            }
        }

        FcoUnit {
            id: unit.id,
            state: new_state,
            created_at: TimePoint::now(),
            metadata: unit.metadata.clone(),
        }
    }
}

// ============================================================================
// MUKANDARA STATE MACHINE
// ============================================================================

/// A state machine based on Mukandara states
#[derive(Debug)]
pub struct MukandaraStateMachine {
    /// Current state
    current: QuantumMukandaraState,
    /// Transition table: (current_state, input) -> new_state
    transitions: std::collections::HashMap<(MukandaraState, MukandaraState), MukandaraState>,
    /// Acceptance states
    accepting: Vec<MukandaraState>,
    /// Initial state
    initial: MukandaraState,
}

impl MukandaraStateMachine {
    /// Create a new state machine
    pub fn new(initial: MukandaraState) -> Self {
        MukandaraStateMachine {
            current: QuantumMukandaraState::pure(initial),
            transitions: std::collections::HashMap::new(),
            accepting: Vec::new(),
            initial,
        }
    }

    /// Add a transition
    pub fn add_transition(&mut self, from: MukandaraState, input: MukandaraState, to: MukandaraState) {
        self.transitions.insert((from, input), to);
    }

    /// Add an accepting state
    pub fn add_accepting(&mut self, state: MukandaraState) {
        self.accepting.push(state);
    }

    /// Step with a classical input
    pub fn step(&mut self, input: MukandaraState) {
        let current_classical = self.current.measure();
        if let Some(&next) = self.transitions.get(&(current_classical, input)) {
            self.current = QuantumMukandaraState::pure(next);
        }
    }

    /// Step with a quantum input
    pub fn quantum_step(&mut self, input: &QuantumMukandaraState) {
        // Measure the input
        let measured = input.measure();
        self.step(measured);
    }

    /// Check if current state is accepting
    pub fn is_accepting(&self) -> bool {
        let current = self.current.measure();
        self.accepting.contains(&current)
    }

    /// Get the current state
    pub fn current_state(&self) -> QuantumMukandaraState {
        self.current.clone()
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        self.current = QuantumMukandaraState::pure(self.initial);
    }
}

// ============================================================================
// FCO SERIALIZATION
// ============================================================================

/// Serialize FCO units to bytes
pub fn serialize_fco(unit: &FcoUnit) -> Result<Vec<u8>, FcoError> {
    serde_json::to_vec(unit).map_err(|e| FcoError::InvalidState(e.to_string()))
}

/// Deserialize FCO units from bytes
pub fn deserialize_fco(bytes: &[u8]) -> Result<FcoUnit, FcoError> {
    serde_json::from_slice(bytes).map_err(|e| FcoError::InvalidState(e.to_string()))
}

/// Serialize a program to bytes
pub fn serialize_program(program: &FcoProgram) -> Result<Vec<u8>, FcoError> {
    serde_json::to_vec(program).map_err(|e| FcoError::InvalidProgram(e.to_string()))
}

/// Deserialize a program from bytes
pub fn deserialize_program(bytes: &[u8]) -> Result<FcoProgram, FcoError> {
    serde_json::from_slice(bytes).map_err(|e| FcoError::InvalidProgram(e.to_string()))
}

// ============================================================================
// CONVENIENCE TRAIT IMPLEMENTATIONS
// ============================================================================

/// Extension trait for MukandaraState operations
pub trait MukandaraStateExt {
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
    fn to_usize(&self) -> usize;
}

impl MukandaraStateExt for MukandaraState {
    fn is_positive(&self) -> bool {
        matches!(self, MukandaraState::Posi)
    }

    fn is_negative(&self) -> bool {
        matches!(self, MukandaraState::Nhaika)
    }

    fn to_usize(&self) -> usize {
        match self {
            MukandaraState::Nhaika => 0,
            MukandaraState::Zera => 1,
            MukandaraState::Posi => 2,
        }
    }
}

/// Extension trait for creating common quantum states
pub trait QuantumMukandaraExt {
    /// Create |+⟩ state (equal superposition)
    fn plus_state() -> Self;
    /// Create |-⟩ state
    fn minus_state() -> Self;
    /// Create |i⟩ state
    fn i_state() -> Self;
    /// Create |-i⟩ state
    fn minus_i_state() -> Self;
}

impl QuantumMukandaraExt for QuantumMukandaraState {
    fn plus_state() -> Self {
        // (|0⟩ + |1⟩) / sqrt(2)
        let amp = Amplitude::magnitude(1.0 / 2.0_f64.sqrt());
        QuantumMukandaraState {
            components: vec![
                SuperpositionComponent::new(MukandaraState::Zera, amp),
                SuperpositionComponent::new(MukandaraState::Posi, amp),
            ],
            normalization: 1.0,
            entanglements: Vec::new(),
            coherence: 1.0,
        }
    }

    fn minus_state() -> Self {
        // (|0⟩ - |1⟩) / sqrt(2)
        let amp = Amplitude::magnitude(1.0 / 2.0_f64.sqrt());
        QuantumMukandaraState {
            components: vec![
                SuperpositionComponent::new(MukandaraState::Zera, amp),
                SuperpositionComponent::with_phase(MukandaraState::Posi, amp, std::f64::consts::PI),
            ],
            normalization: 1.0,
            entanglements: Vec::new(),
            coherence: 1.0,
        }
    }

    fn i_state() -> Self {
        // (|0⟩ + i|1⟩) / sqrt(2)
        let amp = Amplitude { re: 1.0 / 2.0_f64.sqrt(), im: 0.0 };
        let amp_i = Amplitude { re: 0.0, im: 1.0 / 2.0_f64.sqrt() };
        QuantumMukandaraState {
            components: vec![
                SuperpositionComponent::new(MukandaraState::Zera, amp),
                SuperpositionComponent::new(MukandaraState::Posi, amp_i),
            ],
            normalization: 1.0,
            entanglements: Vec::new(),
            coherence: 1.0,
        }
    }

    fn minus_i_state() -> Self {
        // (|0⟩ - i|1⟩) / sqrt(2)
        let amp = Amplitude { re: 1.0 / 2.0_f64.sqrt(), im: 0.0 };
        let amp_minus_i = Amplitude { re: 0.0, im: -1.0 / 2.0_f64.sqrt() };
        QuantumMukandaraState {
            components: vec![
                SuperpositionComponent::new(MukandaraState::Zera, amp),
                SuperpositionComponent::new(MukandaraState::Posi, amp_minus_i),
            ],
            normalization: 1.0,
            entanglements: Vec::new(),
            coherence: 1.0,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mukandara_state_operations() {
        // Test AND
        assert_eq!(MukandaraState::Posi.and(&MukandaraState::Posi), MukandaraState::Posi);
        assert_eq!(MukandaraState::Nhaika.and(&MukandaraState::Posi), MukandaraState::Nhaika);
        assert_eq!(MukandaraState::Zera.and(&MukandaraState::Posi), MukandaraState::Zera);

        // Test OR
        assert_eq!(MukandaraState::Nhaika.or(&MukandaraState::Nhaika), MukandaraState::Nhaika);
        assert_eq!(MukandaraState::Posi.or(&MukandaraState::Nhaika), MukandaraState::Posi);
        assert_eq!(MukandaraState::Zera.or(&MukandaraState::Zera), MukandaraState::Zera);

        // Test NOT
        assert_eq!(MukandaraState::Posi.not(), MukandaraState::Nhaika);
        assert_eq!(MukandaraState::Nhaika.not(), MukandaraState::Posi);
        assert_eq!(MukandaraState::Zera.not(), MukandaraState::Zera);
    }

    #[test]
    fn test_quantum_state() {
        // Test pure state
        let pure = QuantumMukandaraState::pure(MukandaraState::Posi);
        assert!(pure.is_classical());
        assert_eq!(pure.as_classical(), Some(MukandaraState::Posi));

        // Test uniform superposition
        let uniform = QuantumMukandaraState::uniform();
        assert!(!uniform.is_classical());

        // Test measurement collapses to classical
        let measured = uniform.measure();
        assert!(matches!(measured, MukandaraState::Nhaika | MukandaraState::Zera | MukandaraState::Posi));
    }

    #[test]
    fn test_infinitism() {
        // Test basic operations
        let inf = Infinitism::from_f64(f64::INFINITY);
        assert!(inf.is_infinite());

        let zero = Infinitism::from_f64(0.0);
        assert!(zero.is_zero());

        // Test operations
        let result = Infinitism::from_f64(1.0).add(&Infinitism::from_f64(2.0));
        assert_eq!(result.to_f64(), 3.0);
    }

    #[test]
    fn test_time_point() {
        let t1 = TimePoint::from_seconds(1.0);
        let t2 = TimePoint::from_seconds(2.0);

        assert!(t1.is_before(&t2));
        assert!(t2.is_after(&t1));

        let interval = t1.interval_to(&t2);
        assert_eq!(interval.to_f64(), 1.0);
    }

    #[test]
    fn test_fco_engine() {
        let mut engine = FcoEngine::new();

        // Test identity operation
        let result = engine.execute(FcoOperation::Identity, None);
        assert!(result.success);

        // Test measure operation
        let result = engine.execute(FcoOperation::Measure, None);
        assert!(result.success);
        assert!(result.output.state.is_classical());
    }
}
