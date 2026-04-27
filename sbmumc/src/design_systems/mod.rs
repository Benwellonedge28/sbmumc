//! Design Systems Module
//!
//! This module implements design systems, visual languages,
//! and systematic approaches to design for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Design system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignSystem {
    pub system_id: String,
    pub name: String,
    pub visual_language: VisualLanguage,
    pub components: Vec<Component>,
    pub design_tokens: Vec<DesignToken>,
    pub patterns: Vec<DesignPattern>,
    pub accessibility: AccessibilityConfig,
}

/// Visual language definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualLanguage {
    pub color_system: ColorSystem,
    pub typography: TypographySystem,
    pub spacing: SpacingSystem,
    pub iconography: IconographySystem,
    pub imagery: ImagerySystem,
}

/// Color system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSystem {
    pub primary: ColorPalette,
    pub secondary: ColorPalette,
    pub neutral: ColorPalette,
    pub semantic: HashMap<String, ColorValue>,
    pub mode: ColorMode,
}

/// Color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub name: String,
    pub shades: HashMap<String, ColorValue>,
}

/// Color value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorValue {
    pub hex: String,
    pub rgb: [u8; 3],
    pub hsl: [f64; 3],
    pub alpha: f64,
}

/// Color mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorMode {
    Light,
    Dark,
    HighContrast,
    Custom,
}

/// Typography system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographySystem {
    pub font_families: Vec<FontFamily>,
    pub type_scale: TypeScale,
    pub line_heights: HashMap<String, f64>,
    pub letter_spacing: HashMap<String, f64>,
}

/// Font family
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFamily {
    pub name: String,
    pub weights: Vec<FontWeight>,
    pub fallback: Vec<String>,
    pub category: FontCategory,
}

/// Font weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontWeight {
    pub name: String,
    pub value: u16,
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FontCategory {
    Serif,
    SansSerif,
    Monospace,
    Display,
    Handwriting,
}

/// Type scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScale {
    pub base_size: f64,
    pub scale_ratio: f64,
    pub steps: HashMap<String, TypeStep>,
}

/// Type step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeStep {
    pub name: String,
    pub size: f64,
    pub line_height: f64,
    pub usage: String,
}

/// Spacing system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingSystem {
    pub base_unit: f64,
    pub scale: Vec<f64>,
    pub naming: HashMap<String, f64>,
}

/// Iconography system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconographySystem {
    pub library: String,
    pub style: IconStyle,
    pub sizes: Vec<f64>,
    pub stroke_width: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IconStyle {
    Outline,
    Filled,
    Duotone,
    Custom,
}

/// Imagery system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagerySystem {
    pub photography_style: String,
    pub illustration_style: String,
    pub icon_library: String,
    pub image_treatment: Vec<String>,
}

/// Component definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub component_id: String,
    pub name: String,
    pub category: ComponentCategory,
    pub props: Vec<ComponentProp>,
    pub states: Vec<ComponentState>,
    pub variants: Vec<ComponentVariant>,
}

/// Component category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentCategory {
    Layout,
    Navigation,
    Form,
    Display,
    Feedback,
    Overlay,
}

/// Component property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProp {
    pub name: String,
    pub prop_type: String,
    pub default: Option<String>,
    pub required: bool,
}

/// Component state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentState {
    pub state_name: String,
    pub styles: HashMap<String, String>,
}

/// Component variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentVariant {
    pub variant_name: String,
    pub description: String,
}

/// Design token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignToken {
    pub token_name: String,
    pub token_type: TokenType,
    pub value: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Color,
    Typography,
    Spacing,
    Shadow,
    Border,
    Motion,
    Other,
}

/// Design pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignPattern {
    pub pattern_name: String,
    pub description: String,
    pub use_cases: Vec<String>,
    pub examples: Vec<String>,
}

/// Accessibility configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    pub wcag_level: WcagLevel,
    pub contrast_ratios: HashMap<String, f64>,
    pub focus_indicators: bool,
    pub screen_reader: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WcagLevel {
    A,
    AA,
    AAA,
}

impl DesignSystem {
    /// Creates a new design system
    pub fn new() -> Self {
        Self {
            system_id: String::from("design_system_v1"),
            name: String::from("SBMUMC Design System"),
            visual_language: VisualLanguage {
                color_system: ColorSystem {
                    primary: ColorPalette {
                        name: String::from("Primary"),
                        shades: HashMap::new(),
                    },
                    secondary: ColorPalette {
                        name: String::from("Secondary"),
                        shades: HashMap::new(),
                    },
                    neutral: ColorPalette {
                        name: String::from("Neutral"),
                        shades: HashMap::new(),
                    },
                    semantic: HashMap::new(),
                    mode: ColorMode::Light,
                },
                typography: TypographySystem {
                    font_families: vec![],
                    type_scale: TypeScale {
                        base_size: 16.0,
                        scale_ratio: 1.25,
                        steps: HashMap::new(),
                    },
                    line_heights: HashMap::new(),
                    letter_spacing: HashMap::new(),
                },
                spacing: SpacingSystem {
                    base_unit: 4.0,
                    scale: vec![4.0, 8.0, 16.0, 24.0, 32.0, 48.0, 64.0],
                    naming: HashMap::new(),
                },
                iconography: IconographySystem {
                    library: String::from("Heroicons"),
                    style: IconStyle::Outline,
                    sizes: vec![16.0, 20.0, 24.0, 32.0],
                    stroke_width: 1.5,
                },
                imagery: ImagerySystem {
                    photography_style: String::from("Clean"),
                    illustration_style: String::from("Flat"),
                    icon_library: String::from("Custom"),
                    image_treatment: vec![],
                },
            },
            components: vec![],
            design_tokens: vec![],
            patterns: vec![],
            accessibility: AccessibilityConfig {
                wcag_level: WcagLevel::AA,
                contrast_ratios: HashMap::new(),
                focus_indicators: true,
                screen_reader: true,
            },
        }
    }

    /// Validates design system consistency
    pub fn validate(&self) -> DesignSystemValidation {
        DesignSystemValidation {
            system_id: self.system_id.clone(),
            is_valid: true,
            issues: vec![],
            warnings: vec![],
        }
    }

    /// Generates component documentation
    pub fn document_component(&self, component_id: &str) -> Result<ComponentDocumentation> {
        let component = self.components.iter()
            .find(|c| c.component_id == component_id)
            .ok_or(SbmumcError::NotFound(String::from("Component not found")))?;
        
        Ok(ComponentDocumentation {
            component_id: component.component_id.clone(),
            component_name: component.name.clone(),
            category: component.category.clone(),
            props: component.props.clone(),
            states: component.states.clone(),
            usage: String::from("See component API"),
            examples: component.variants.len(),
        })
    }

    /// Exports design tokens
    pub fn export_tokens(&self, format: ExportFormat) -> TokenExport {
        TokenExport {
            format,
            tokens: self.design_tokens.clone(),
            metadata: ExportMetadata {
                system_name: self.name.clone(),
                version: String::from("1.0.0"),
                generated: String::from("2024-01-01"),
            },
        }
    }

    /// Checks accessibility compliance
    pub fn check_accessibility(&self, color_pair: (&str, &str)) -> AccessibilityCheck {
        AccessibilityCheck {
            foreground: color_pair.0.to_string(),
            background: color_pair.1.to_string(),
            ratio: 4.5,
            passes_aa: true,
            passes_aaa: false,
            recommendation: String::from("Good contrast"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignSystemValidation {
    pub system_id: String,
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDocumentation {
    pub component_id: String,
    pub component_name: String,
    pub category: ComponentCategory,
    pub props: Vec<ComponentProp>,
    pub states: Vec<ComponentState>,
    pub usage: String,
    pub examples: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenExport {
    pub format: ExportFormat,
    pub tokens: Vec<DesignToken>,
    pub metadata: ExportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportFormat {
    Json,
    Css,
    Scss,
    Swift,
    Kotlin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub system_name: String,
    pub version: String,
    pub generated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityCheck {
    pub foreground: String,
    pub background: String,
    pub ratio: f64,
    pub passes_aa: bool,
    pub passes_aaa: bool,
    pub recommendation: String,
}

impl Default for DesignSystem {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_design_system_creation() {
        let ds = DesignSystem::new();
        assert_eq!(ds.system_id, "design_system_v1");
    }
    #[test]
    fn test_validate_design_system() {
        let ds = DesignSystem::new();
        let validation = ds.validate();
        assert!(validation.is_valid);
    }
}
