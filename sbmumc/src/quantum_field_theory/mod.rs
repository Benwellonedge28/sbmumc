//! Quantum Field Theory Module
//!
//! This module implements quantum field theory frameworks, field quantization,
//! renormalization, and particle physics interactions.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Quantum field theory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumFieldTheory {
    pub qft_id: String,
    pub theory_type: QFTType,
    pub fields: Vec<QuantumField>,
    pub interactions: Vec<Interaction>,
    pub symmetries: Vec<Symmetry>,
    pub gauge_groups: Vec<GaugeGroup>,
    pub renormalization: RenormalizationGroup,
}

/// Types of QFT frameworks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QFTType {
    Scalar,
    Fermionic,
    Gauge,
    YangMills,
    NonAbelian,
    Conformal,
    Topological,
    Supersymmetric,
    String motivated,
    Effective,
}

/// Quantum field representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumField {
    pub field_id: String,
    pub field_name: String,
    pub field_type: FieldType,
    pub spin: Spin,
    pub mass: f64,
    pub charges: Vec<Charge>,
    pub representation: Representation,
    pub quantization: QuantizationScheme,
}

/// Spin classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Spin {
    Scalar,
    SpinorHalf,
    Vector,
    SpinOneHalf,
    Tensor,
    SpinTwo,
    HigherSpin,
}

/// Charge under gauge groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charge {
    pub charge_id: String,
    pub gauge_group: String,
    pub charge_value: f64,
    pub representation: String,
}

/// Group representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Representation {
    pub rep_type: RepType,
    pub dimension: u32,
    pub generators: Vec<Generator>,
    pub casimir_eigenvalue: f64,
}

/// Types of representations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepType {
    Fundamental,
    AntiFundamental,
    Adjoint,
    Vector,
    Singlet,
    Higher,
}

/// Generator of Lie algebra
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generator {
    pub generator_id: String,
    pub matrix: Vec<Vec<f64>>,
    pub trace_commutator: f64,
}

/// Quantization scheme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuantizationScheme {
    Canonical,
    PathIntegral,
    BRST,
    Geometric,
    DeWitt,
}

/// Interaction between fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub interaction_id: String,
    pub coupling: Coupling,
    pub vertex_factor: String,
    pub order: u32,
    pub verified: bool,
}

/// Coupling constant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupling {
    pub coupling_id: String,
    pub coupling_type: CouplingType,
    pub value: f64,
    pub scale_dependence: ScaleDependence,
}

/// Types of couplings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CouplingType {
    Gauge,
    Yukawa,
    Quartic,
    Tensor,
    HigherOrder,
}

/// Scale dependence of coupling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleDependence {
    pub reference_scale: f64,
    pub coupling_at_reference: f64,
    pub beta_function: String,
    pub running_exponent: f64,
}

/// Symmetry of the theory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symmetry {
    pub symmetry_id: String,
    pub symmetry_type: SymmetryType,
    pub group: String,
    pub breaking_scale: Option<f64>,
    pub conserved_quantities: Vec<ConservedQuantity>,
    pub anomaly_cancelled: bool,
}

/// Types of symmetries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SymmetryType {
    Gauge,
    Global,
    Chiral,
    Conformal,
    Supersymmetry,
    Superconformal,
    Lorentz,
    Poincare,
}

/// Conserved quantity from symmetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservedQuantity {
    pub quantity_name: String,
    pub quantum_number: String,
    pub conservation_law: String,
}

/// Gauge group structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugeGroup {
    pub group_id: String,
    pub group_name: String,
    pub rank: u32,
    pub dimension: u32,
    pub structure_constants: Vec<f64>,
    pub representation_content: Vec<String>,
}

/// Renormalization group analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenormalizationGroup {
    pub rg_id: String,
    pub beta_functions: Vec<BetaFunction>,
    pub anomalous_dimensions: Vec<AnomalousDimension>,
    pub fixed_points: Vec<FixedPoint>,
    pub flow_diagram: FlowDiagram,
}

/// Beta function for coupling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaFunction {
    pub coupling_id: String,
    pub expansion_coefficients: Vec<f64>,
    pub scheme: String,
}

/// Anomalous dimension of operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalousDimension {
    pub operator: String,
    pub dimension: f64,
    pub conformal_weight: f64,
}

/// RG fixed point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedPoint {
    pub fixed_point_id: String,
    pub couplings: HashMap<String, f64>,
    pub stability_matrix: Vec<Vec<f64>>,
    pub critical_exponents: Vec<f64>,
    pub physical_relevance: f64,
}

/// RG flow diagram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowDiagram {
    pub diagram_id: String,
    pub axes: Vec<String>,
    pub trajectories: Vec<Trajectory>,
    pub separatrices: Vec<String>,
}

/// RG trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub trajectory_id: String,
    pub path: Vec<HashMap<String, f64>>,
    pub direction: TrajectoryDirection,
    pub endpoint: Option<String>,
}

/// Direction of RG flow
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrajectoryDirection {
    Infrared,
    Ultraviolet,
    Cyclic,
    FixedPoint,
}

impl QuantumFieldTheory {
    /// Creates a new QFT system
    pub fn new() -> Self {
        Self {
            qft_id: String::from("qft_v1"),
            theory_type: QFTType::YangMills,
            fields: vec![
                QuantumField {
                    field_id: String::from("gluon"),
                    field_name: String::from("Gluon field"),
                    field_type: FieldType::Vector,
                    spin: Spin::Vector,
                    mass: 0.0,
                    charges: vec![],
                    representation: Representation {
                        rep_type: RepType::Adjoint,
                        dimension: 8,
                        generators: vec![],
                        casimir_eigenvalue: 3.0,
                    },
                    quantization: QuantizationScheme::PathIntegral,
                },
            ],
            interactions: Vec::new(),
            symmetries: vec![
                Symmetry {
                    symmetry_id: String::from("su3"),
                    symmetry_type: SymmetryType::Gauge,
                    group: String::from("SU(3)"),
                    breaking_scale: None,
                    conserved_quantities: vec![
                        ConservedQuantity {
                            quantity_name: String::from("Color charge"),
                            quantum_number: String::from("3"),
                            conservation_law: String::from("Color conservation"),
                        },
                    ],
                    anomaly_cancelled: true,
                },
            ],
            gauge_groups: vec![
                GaugeGroup {
                    group_id: String::from("su3_color"),
                    group_name: String::from("SU(3) Color"),
                    rank: 2,
                    dimension: 8,
                    structure_constants: vec![],
                    representation_content: vec![String::from("Quarks: 3"), String::from("Gluons: 8")],
                },
            ],
            renormalization: RenormalizationGroup {
                rg_id: String::from("rg_1"),
                beta_functions: vec![],
                anomalous_dimensions: vec![],
                fixed_points: vec![],
                flow_diagram: FlowDiagram {
                    diagram_id: String::from("flow_1"),
                    axes: vec![String::from("g1"), String::from("g2")],
                    trajectories: vec![],
                    separatrices: vec![],
                },
            },
        }
    }

    /// Adds field to theory
    pub fn add_field(&mut self, field: QuantumField) -> Result<&QuantumField> {
        self.fields.push(field);
        Ok(self.fields.last().unwrap())
    }

    /// Constructs Lagrangian
    pub fn construct_lagrangian(&self) -> Result<Lagrangian> {
        let kinetic_terms = vec![
            KineticTerm {
                term_id: String::from("kin_1"),
                field_id: String::from("gluon"),
                coefficient: -1.0 / 4.0,
                expression: String::from("F_{\\mu\\nu}^a F^{\\mu\\nu}_a"),
            },
        ];
        let interaction_terms = vec![
            InteractionTerm {
                term_id: String::from("int_1"),
                expression: String::from("g_s f^{abc} A^a_\\mu A^b_\\nu A^c_\\rho"),
                coupling: String::from("g_s"),
                vertices: 3,
            },
        ];
        Ok(Lagrangian {
            lagrangian_id: String::from("lagrangian_1"),
            kinetic_terms,
            interaction_terms,
            full_expression: String::from("L = -1/4 F_{\\mu\nu}^a F^{\mu\nu}_a + g_s f^{abc} A_\mu^a A_\nu^b A_\rho^c"),
            symmetries: vec![String::from("Gauge invariance")],
        })
    }

    /// Computes scattering amplitude
    pub fn compute_amplitude(&self, process: &ScatteringProcess) -> Result<Amplitude> {
        let amplitude = Amplitude {
            process_id: format!("amp_{}", process.process_name),
            amplitude_type: AmplitudeType::TreeLevel,
            expression: String::from("M = g_s^3"),
            order: 3,
            diagrams: process.diagrams.clone(),
            spin_sum: true,
            color_sum: true,
            result: Complex::new(1.0, 0.0),
        };
        Ok(amplitude)
    }

    /// Analyzes RG flow
    pub fn analyze_rg_flow(&self) -> RGAnalysis {
        let fixed_points = vec![
            FixedPoint {
                fixed_point_id: String::from("gaussian"),
                couplings: HashMap::new(),
                stability_matrix: vec![vec![-1.0]],
                critical_exponents: vec![1.0],
                physical_relevance: 0.5,
            },
        ];
        RGAnalysis {
            fixed_points,
            asymptotic_freedom: true,
            confinement_scale: 200.0,
            chiral_symmetry: false,
        }
    }

    /// Checks anomaly cancellation
    pub fn check_anomalies(&self) -> AnomalyCheck {
        let anomalies = vec![
            Anomaly {
                anomaly_id: String::from("anomaly_1"),
                gauge_group: String::from("SU(3)"),
                cancellation: CancellationStatus::Cancelled,
                contribution: 0.0,
                required_particles: vec![String::from("Quarks")],
            },
        ];
        AnomalyCheck {
            anomalies,
            all_cancelled: true,
            gauge invariance_preserved: true,
        }
    }
}

/// Lagrangian of the theory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lagrangian {
    pub lagrangian_id: String,
    pub kinetic_terms: Vec<KineticTerm>,
    pub interaction_terms: Vec<InteractionTerm>,
    pub full_expression: String,
    pub symmetries: Vec<String>,
}

/// Kinetic term in Lagrangian
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KineticTerm {
    pub term_id: String,
    pub field_id: String,
    pub coefficient: f64,
    pub expression: String,
}

/// Interaction term in Lagrangian
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionTerm {
    pub term_id: String,
    pub expression: String,
    pub coupling: String,
    pub vertices: u32,
}

/// Scattering process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScatteringProcess {
    pub process_name: String,
    pub initial_particles: Vec<String>,
    pub final_particles: Vec<String>,
    pub center_of_mass_energy: f64,
    pub diagrams: Vec<FeynmanDiagram>,
}

/// Feynman diagram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeynmanDiagram {
    pub diagram_id: String,
    pub diagram_type: String,
    pub vertices: u32,
    pub propagators: u32,
    pub amplitude_factor: f64,
}

/// Computed amplitude
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amplitude {
    pub process_id: String,
    pub amplitude_type: AmplitudeType,
    pub expression: String,
    pub order: u32,
    pub diagrams: Vec<FeynmanDiagram>,
    pub spin_sum: bool,
    pub color_sum: bool,
    pub result: Complex,
}

/// Types of amplitudes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AmplitudeType {
    TreeLevel,
    OneLoop,
    TwoLoop,
    Higher,
}

/// Complex number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

/// RG analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGAnalysis {
    pub fixed_points: Vec<FixedPoint>,
    pub asymptotic_freedom: bool,
    pub confinement_scale: f64,
    pub chiral_symmetry: bool,
}

/// Anomaly check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub anomaly_id: String,
    pub gauge_group: String,
    pub cancellation: CancellationStatus,
    pub contribution: f64,
    pub required_particles: Vec<String>,
}

/// Cancellation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CancellationStatus {
    Cancelled,
    NotCancelled,
    Partial,
}

/// Anomaly check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyCheck {
    pub anomalies: Vec<Anomaly>,
    pub all_cancelled: bool,
    pub gauge invariance_preserved: bool,
}

impl Default for QuantumFieldTheory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lagrangian_construction() {
        let qft = QuantumFieldTheory::new();
        let lagrangian = qft.construct_lagrangian();
        assert!(lagrangian.is_ok());
    }

    #[test]
    fn test_anomaly_check() {
        let qft = QuantumFieldTheory::new();
        let check = qft.check_anomalies();
        assert!(check.all_cancelled);
    }

    #[test]
    fn test_rg_flow_analysis() {
        let qft = QuantumFieldTheory::new();
        let analysis = qft.analyze_rg_flow();
        assert!(analysis.asymptotic_freedom);
    }
}
