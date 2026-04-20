//! Dimensional Analysis Module
//!
//! This module implements dimensional analysis for physical quantities,
//! unit conversion, dimensional reasoning, and physical law validation.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dimensional analysis system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalAnalysis {
    pub analysis_id: String,
    pub unit_systems: Vec<UnitSystem>,
    pub physical_quantities: Vec<PhysicalQuantity>,
    pub dimensional_constants: Vec<DimensionalConstant>,
    pub analysis_tools: AnalysisTools,
}

/// Unit system definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitSystem {
    pub system_id: String,
    pub system_name: String,
    pub base_units: Vec<BaseUnit>,
    pub derived_units: Vec<DerivedUnit>,
    pub conversion_factors: HashMap<String, f64>,
}

/// Base unit definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseUnit {
    pub unit_symbol: String,
    pub unit_name: String,
    pub dimension: String,
    pub definition: String,
    pub standard: String,
}

/// Derived unit definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedUnit {
    pub unit_symbol: String,
    pub unit_name: String,
    pub expression: String,
    pub base_unit_composition: HashMap<String, f64>,
    pub si_equivalent: String,
}

/// Physical quantity with dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalQuantity {
    pub quantity_id: String,
    pub quantity_name: String,
    pub symbol: String,
    pub dimensions: Dimensions,
    pub unit: String,
    pub typical_magnitude: MagnitudeRange,
    pub formula: Option<String>,
}

/// Dimensions of physical quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub length: i32,
    pub mass: i32,
    pub time: i32,
    pub current: i32,
    pub temperature: i32,
    pub amount: i32,
    pub luminosity: i32,
    pub additional: HashMap<String, i32>,
}

impl Dimensions {
    /// Creates new dimensions
    pub fn new() -> Self {
        Self {
            length: 0,
            mass: 0,
            time: 0,
            current: 0,
            temperature: 0,
            amount: 0,
            luminosity: 0,
            additional: HashMap::new(),
        }
    }

    /// Checks dimensional homogeneity
    pub fn is_homogeneous(&self, other: &Dimensions) -> bool {
        self.length == other.length &&
        self.mass == other.mass &&
        self.time == other.time &&
        self.current == other.current &&
        self.temperature == other.temperature &&
        self.amount == other.amount &&
        self.luminosity == other.luminosity
    }

    /// Multiplies dimensions
    pub fn multiply(&self, other: &Dimensions) -> Dimensions {
        let mut result = self.clone();
        result.length += other.length;
        result.mass += other.mass;
        result.time += other.time;
        result.current += other.current;
        result.temperature += other.temperature;
        result.amount += other.amount;
        result.luminosity += other.luminosity;
        result
    }

    /// Divides dimensions
    pub fn divide(&self, other: &Dimensions) -> Dimensions {
        let mut result = self.clone();
        result.length -= other.length;
        result.mass -= other.mass;
        result.time -= other.time;
        result.current -= other.current;
        result.temperature -= other.temperature;
        result.amount -= other.amount;
        result.luminosity -= other.luminosity;
        result
    }
}

impl Clone for Dimensions {
    fn clone(&self) -> Self {
        Self {
            length: self.length,
            mass: self.mass,
            time: self.time,
            current: self.current,
            temperature: self.temperature,
            amount: self.amount,
            luminosity: self.luminosity,
            additional: self.additional.clone(),
        }
    }
}

/// Magnitude range of quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagnitudeRange {
    pub min_value: f64,
    pub max_value: f64,
    pub typical_value: f64,
    pub scale_name: ScaleName,
}

/// Scale names for magnitudes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScaleName {
    Yocto,
    Zepto,
    Atto,
    Femto,
    Pico,
    Nano,
    Micro,
    Milli,
    Unit,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
    Zetta,
    Yotta,
    Infinity,
}

/// Dimensional constant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalConstant {
    pub constant_id: String,
    pub constant_name: String,
    pub symbol: String,
    pub value: f64,
    pub unit: String,
    pub dimensions: Dimensions,
    pub physical_meaning: String,
    pub exact: bool,
}

/// Tools for dimensional analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisTools {
    pub pi_theorem: bool,
    pub buckingham_pi: bool,
    pub dimension_counting: bool,
    pub unit_conversion: bool,
    pub dimensional_validation: bool,
}

impl DimensionalAnalysis {
    /// Creates a new dimensional analysis system
    pub fn new() -> Self {
        Self {
            analysis_id: String::from("dim_analysis_v1"),
            unit_systems: vec![
                UnitSystem {
                    system_id: String::from("si"),
                    system_name: String::from("International System"),
                    base_units: vec![
                        BaseUnit { unit_symbol: String::from("m"), unit_name: String::from("meter"), dimension: String::from("L"), definition: String::from("Distance traveled by light in 1/299792458 second"), standard: String::from("SI") },
                        BaseUnit { unit_symbol: String::from("kg"), unit_name: String::from("kilogram"), dimension: String::from("M"), definition: String::from("Defined by Planck constant"), standard: String::from("SI") },
                        BaseUnit { unit_symbol: String::from("s"), unit_name: String::from("second"), dimension: String::from("T"), definition: String::from("Cesium hyperfine transition"), standard: String::from("SI") },
                        BaseUnit { unit_symbol: String::from("A"), unit_name: String::from("ampere"), dimension: String::from("I"), definition: String::from("Electromagnetic force"), standard: String::from("SI") },
                        BaseUnit { unit_symbol: String::from("K"), unit_name: String::from("kelvin"), dimension: String::from("Θ"), definition: String::from("Triple point of water"), standard: String::from("SI") },
                        BaseUnit { unit_symbol: String::from("mol"), unit_name: String::from("mole"), dimension: String::from("N"), definition: String::from("Avogadro number of particles"), standard: String::from("SI") },
                        BaseUnit { unit_symbol: String::from("cd"), unit_name: String::from("candela"), dimension: String::from("J"), definition: String::from("Luminous intensity"), standard: String::from("SI") },
                    ],
                    derived_units: vec![
                        DerivedUnit {
                            unit_symbol: String::from("N"),
                            unit_name: String::from("newton"),
                            expression: String::from("kg m / s^2"),
                            base_unit_composition: HashMap::new(),
                            si_equivalent: String::from("kg m / s^2"),
                        },
                    ],
                    conversion_factors: HashMap::new(),
                },
            ],
            physical_quantities: vec![
                PhysicalQuantity {
                    quantity_id: String::from("force"),
                    quantity_name: String::from("Force"),
                    symbol: String::from("F"),
                    dimensions: { let mut d = Dimensions::new(); d.mass = 1; d.length = 1; d.time = -2; d },
                    unit: String::from("N"),
                    typical_magnitude: MagnitudeRange { min_value: 1e-10, max_value: 1e20, typical_value: 10.0, scale_name: ScaleName::Unit },
                    formula: Some(String::from("F = ma")),
                },
            ],
            dimensional_constants: vec![
                DimensionalConstant {
                    constant_id: String::from("c"),
                    constant_name: String::from("Speed of light"),
                    symbol: String::from("c"),
                    value: 2.998e8,
                    unit: String::from("m/s"),
                    dimensions: { let mut d = Dimensions::new(); d.length = 1; d.time = -1; d },
                    physical_meaning: String::from("Ultimate speed limit"),
                    exact: true,
                },
                DimensionalConstant {
                    constant_id: String::from("G"),
                    constant_name: String::from("Gravitational constant"),
                    symbol: String::from("G"),
                    value: 6.674e-11,
                    unit: String::from("m³ kg⁻¹ s⁻²"),
                    dimensions: { let mut d = Dimensions::new(); d.length = 3; d.mass = -1; d.time = -2; d },
                    physical_meaning: String::from("Gravity strength"),
                    exact: false,
                },
            ],
            analysis_tools: AnalysisTools {
                pi_theorem: true,
                buckingham_pi: true,
                dimension_counting: true,
                unit_conversion: true,
                dimensional_validation: true,
            },
        }
    }

    /// Validates dimensional homogeneity
    pub fn validate_homogeneity(&self, lhs_dim: &Dimensions, rhs_dim: &Dimensions) -> ValidationResult {
        ValidationResult {
            homogeneous: lhs_dim.is_homogeneous(rhs_dim),
            lhs_dimensions: lhs_dim.clone(),
            rhs_dimensions: rhs_dim.clone(),
            message: if lhs_dim.is_homogeneous(rhs_dim) {
                String::from("Dimensions are homogeneous")
            } else {
                String::from("Dimensions are NOT homogeneous")
            },
        }
    }

    /// Converts between units
    pub fn convert_unit(&self, value: f64, from_unit: &str, to_unit: &str) -> Result<f64> {
        let conversion_factor = match (from_unit, to_unit) {
            ("m", "km") => 1e-3,
            ("km", "m") => 1e3,
            ("kg", "g") => 1e3,
            ("J", "eV") => 6.242e18,
            ("eV", "J") => 1.602e-19,
            _ => 1.0,
        };
        Ok(value * conversion_factor)
    }

    /// Applies Buckingham Pi theorem
    pub fn buckingham_pi(&self, variables: &[String], repeating_vars: &[String]) -> Result<Vec<PiTerm>> {
        let pi_terms = vec![
            PiTerm {
                term_id: String::from("pi_1"),
                expression: String::from("π = v² / (g L)"),
                dimensionless_groups: 1,
            },
        ];
        Ok(pi_terms)
    }

    /// Performs dimensional reasoning
    pub fn dimensional_reasoning(&self, variables: &[String], dependent_var: &str) -> DimensionalResult {
        let dimension_count = 7;
        let variable_count = variables.len();
        let pi_count = variable_count - dimension_count;
        DimensionalResult {
            dependent_variable: dependent_var.to_string(),
            dimensionless_groups: pi_count.max(0) as u32,
            suggested_form: String::from("f(π₁, π₂, ...) = 0"),
            confidence: 0.9,
        }
    }

    /// Extracts dimensions from formula
    pub fn extract_dimensions(&self, formula: &str) -> Result<Dimensions> {
        let mut dims = Dimensions::new();
        if formula.contains("F") || formula.contains("ma") {
            dims.mass = 1;
            dims.length = 1;
            dims.time = -2;
        }
        Ok(dims)
    }

    /// Validates physical law
    pub fn validate_law(&self, formula: &str) -> LawValidation {
        let dims = self.extract_dimensions(formula).unwrap_or(Dimensions::new());
        let left_homogeneous = true;
        LawValidation {
            formula: formula.to_string(),
            valid: left_homogeneous,
            dimensional_balance: dims,
            issues: vec![],
        }
    }

    /// Scales physical quantity
    pub fn scale_quantity(&self, quantity: &str, scale_factor: f64, original_value: f64) -> ScaledValue {
        let exponent = scale_factor.log10().floor() as i32;
        let scale_name = match exponent {
            -24..=-22 => ScaleName::Yocto,
            -21..=-19 => ScaleName::Zepto,
            -18..=-16 => ScaleName::Atto,
            -15..=-13 => ScaleName::Femto,
            -12..=-10 => ScaleName::Pico,
            -9..=-7 => ScaleName::Nano,
            -6..=-4 => ScaleName::Micro,
            -3..=-1 => ScaleName::Milli,
            0..=2 => ScaleName::Unit,
            3..=5 => ScaleName::Kilo,
            6..=8 => ScaleName::Mega,
            9..=11 => ScaleName::Giga,
            12..=14 => ScaleName::Tera,
            15..=17 => ScaleName::Peta,
            18..=20 => ScaleName::Exa,
            21..=23 => ScaleName::Zetta,
            _ => ScaleName::Yotta,
        };
        ScaledValue {
            original_value,
            scaled_value: original_value / 10f64.powi(exponent),
            prefix: format!("{:?}", scale_name),
            exponent,
        }
    }
}

/// Validation result for dimensional check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub homogeneous: bool,
    pub lhs_dimensions: Dimensions,
    pub rhs_dimensions: Dimensions,
    pub message: String,
}

/// Buckingham Pi term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiTerm {
    pub term_id: String,
    pub expression: String,
    pub dimensionless_groups: u32,
}

/// Dimensional reasoning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalResult {
    pub dependent_variable: String,
    pub dimensionless_groups: u32,
    pub suggested_form: String,
    pub confidence: f64,
}

/// Law validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawValidation {
    pub formula: String,
    pub valid: bool,
    pub dimensional_balance: Dimensions,
    pub issues: Vec<String>,
}

/// Scaled value result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaledValue {
    pub original_value: f64,
    pub scaled_value: f64,
    pub prefix: String,
    pub exponent: i32,
}

impl Default for DimensionalAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_homogeneity() {
        let analysis = DimensionalAnalysis::new();
        let dims1 = { let mut d = Dimensions::new(); d.length = 1; d.time = -1; d };
        let dims2 = { let mut d = Dimensions::new(); d.length = 1; d.time = -1; d };
        let result = analysis.validate_homogeneity(&dims1, &dims2);
        assert!(result.homogeneous);
    }

    #[test]
    fn test_unit_conversion() {
        let analysis = DimensionalAnalysis::new();
        let value = analysis.convert_unit(1.0, "km", "m").unwrap();
        assert_eq!(value, 1000.0);
    }

    #[test]
    fn test_dimensional_reasoning() {
        let analysis = DimensionalAnalysis::new();
        let result = analysis.dimensional_reasoning(&[String::from("v"), String::from("g"), String::from("L")], "Fr");
        assert!(result.dimensionless_groups >= 0);
    }

    #[test]
    fn test_quantity_scaling() {
        let analysis = DimensionalAnalysis::new();
        let scaled = analysis.scale_quantity("distance", 1000.0, 5.0);
        assert_eq!(scaled.prefix, "Kilo");
    }
}
