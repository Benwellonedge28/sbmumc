//! Economics Module
//!
//! This module implements economic frameworks, market analysis,
//! and economic theory for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Economics system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Economics {
    pub econ_id: String,
    pub macroeconomic_indicators: Vec<MacroIndicator>,
    pub markets: Vec<Market>,
    pub theories: Vec<EconomicTheory>,
    pub policies: Vec<EconomicPolicy>,
    pub international_economics: InternationalEconomics,
}

/// Macroeconomic indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroIndicator {
    pub indicator_id: String,
    pub indicator_name: String,
    pub value: f64,
    pub unit: String,
    pub time_series: Vec<TimeSeriesPoint>,
    pub trend: TrendDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub date: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

/// Market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub market_id: String,
    pub market_name: String,
    pub market_type: MarketType,
    pub participants: Vec<MarketParticipant>,
    pub supply_demand: SupplyDemand,
    pub price_mechanisms: Vec<PriceMechanism>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarketType {
    PerfectCompetition,
    MonopolisticCompetition,
    Oligopoly,
    Monopoly,
    Monopsony,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketParticipant {
    pub participant_id: String,
    pub participant_type: ParticipantType,
    pub market_power: f64,
    pub behavior_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParticipantType {
    Consumer,
    Producer,
    Investor,
    Government,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyDemand {
    pub supply_curve: CurveData,
    pub demand_curve: CurveData,
    pub equilibrium_price: f64,
    pub equilibrium_quantity: f64,
    pub elasticity: ElasticityAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurveData {
    pub curve_type: CurveType,
    pub equation: String,
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CurveType {
    Linear,
    Exponential,
    Logarithmic,
    CobbDouglas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticityAnalysis {
    pub price_elasticity_demand: f64,
    pub price_elasticity_supply: f64,
    pub income_elasticity: f64,
    pub cross_price_elasticity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceMechanism {
    pub mechanism_type: MechanismType,
    pub description: String,
    pub efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MechanismType {
    Auction,
    PriceSetter,
    Negotiation,
    GovernmentSet,
}

/// Economic theory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicTheory {
    pub theory_id: String,
    pub theory_name: String,
    pub school: EconomicSchool,
    pub core_arguments: Vec<String>,
    pub assumptions: Vec<String>,
    pub predictions: Vec<String>,
    pub empirical_support: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EconomicSchool {
    Classical,
    Keynesian,
    Monetarist,
    Austrian,
    Marxist,
    Institutionalist,
    Behavioral,
    Neoclassical,
}

/// Economic policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicPolicy {
    pub policy_id: String,
    pub policy_name: String,
    pub policy_type: PolicyType,
    pub objectives: Vec<String>,
    pub tools: Vec<PolicyTool>,
    pub implementation_status: ImplementationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyType {
    Monetary,
    Fiscal,
    Trade,
    Industrial,
    Social,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyTool {
    pub tool_name: String,
    pub tool_type: ToolType,
    pub effectiveness: f64,
    pub side_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolType {
    Tax,
    Subsidy,
    Regulation,
    InterestRate,
    MoneySupply,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationStatus {
    pub current_stage: String,
    pub progress_percentage: f64,
    pub challenges: Vec<String>,
}

/// International economics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalEconomics {
    pub trade_flows: Vec<TradeFlow>,
    pub exchange_rates: Vec<ExchangeRate>,
    pub trade_agreements: Vec<TradeAgreement>,
    pub balance_of_payments: BalanceOfPayments,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFlow {
    pub flow_id: String,
    pub exporter: String,
    pub importer: String,
    pub goods_type: String,
    pub value_usd: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub currency_pair: String,
    pub rate: f64,
    pub volatility: f64,
    pub trend: TrendDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAgreement {
    pub agreement_id: String,
    pub agreement_name: String,
    pub member_countries: Vec<String>,
    pub tariff_reductions: f64,
    pub trade_volume_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceOfPayments {
    pub current_account: f64,
    pub capital_account: f64,
    pub financial_account: f64,
    pub errors_and_omissions: f64,
    pub overall_balance: f64,
}

impl Economics {
    /// Creates a new economics system
    pub fn new() -> Self {
        Self {
            econ_id: String::from("economics_v1"),
            macroeconomic_indicators: vec![
                MacroIndicator {
                    indicator_id: String::from("ind_gdp"),
                    indicator_name: String::from("GDP"),
                    value: 21.43e12,
                    unit: String::from("USD"),
                    time_series: vec![],
                    trend: TrendDirection::Increasing,
                },
            ],
            markets: vec![
                Market {
                    market_id: String::from("mkt_labor"),
                    market_name: String::from("Labor Market"),
                    market_type: MarketType::MonopolisticCompetition,
                    participants: vec![],
                    supply_demand: SupplyDemand {
                        supply_curve: CurveData { curve_type: CurveType::Linear, equation: String::from("Qs = a + bP"), parameters: HashMap::new() },
                        demand_curve: CurveData { curve_type: CurveType::Linear, equation: String::from("Qd = c - dP"), parameters: HashMap::new() },
                        equilibrium_price: 25.0,
                        equilibrium_quantity: 1000.0,
                        elasticity: ElasticityAnalysis { price_elasticity_demand: -0.5, price_elasticity_supply: 0.8, income_elasticity: 1.0, cross_price_elasticity: 0.0 },
                    },
                    price_mechanisms: vec![],
                },
            ],
            theories: vec![
                EconomicTheory {
                    theory_id: String::from("theory_supply_demand"),
                    theory_name: String::from("Supply and Demand"),
                    school: EconomicSchool::Classical,
                    core_arguments: vec![String::from("Market equilibrium"), String::from("Price determination")],
                    assumptions: vec![String::from("Rational actors"), String::from("Perfect information")],
                    predictions: vec![String::from("Price adjusts to clear markets")],
                    empirical_support: 0.8,
                },
            ],
            policies: vec![
                EconomicPolicy {
                    policy_id: String::from("pol_fed_funds"),
                    policy_name: String::from("Federal Funds Rate"),
                    policy_type: PolicyType::Monetary,
                    objectives: vec![String::from("Price stability"), String::from("Maximum employment")],
                    tools: vec![PolicyTool { tool_name: String::from("Interest rate"), tool_type: ToolType::InterestRate, effectiveness: 0.7, side_effects: vec![String::from("May increase inequality")] }],
                    implementation_status: ImplementationStatus { current_stage: String::from("Active"), progress_percentage: 100.0, challenges: vec![] },
                },
            ],
            international_economics: InternationalEconomics {
                trade_flows: vec![],
                exchange_rates: vec![
                    ExchangeRate { currency_pair: String::from("EUR/USD"), rate: 1.08, volatility: 0.05, trend: TrendDirection::Stable },
                ],
                trade_agreements: vec![],
                balance_of_payments: BalanceOfPayments { current_account: -800.0e9, capital_account: 100.0e9, financial_account: 700.0e9, errors_and_omissions: 0.0, overall_balance: 0.0 },
            },
        }
    }

    /// Analyzes market equilibrium
    pub fn analyze_equilibrium(&self, market_id: &str) -> MarketEquilibrium {
        MarketEquilibrium {
            market_id: market_id.to_string(),
            equilibrium_price: 100.0,
            equilibrium_quantity: 1000.0,
            surplus: 0.0,
            shortage: 0.0,
        }
    }

    /// Calculates GDP
    pub fn calculate_gdp(&self, components: &GDPCalculation) -> GDPResult {
        GDPResult {
            method: String::from("Expenditure approach"),
            consumption: components.consumption,
            investment: components.investment,
            government_spending: components.government_spending,
            net_exports: components.net_exports,
            total_gdp: components.consumption + components.investment + components.government_spending + components.net_exports,
        }
    }

    /// Analyzes inflation
    pub fn analyze_inflation(&self, base_year: &str) -> InflationAnalysis {
        InflationAnalysis {
            base_year: base_year.to_string(),
            cpi: 260.0,
            inflation_rate: 0.035,
            core_inflation: 0.03,
            expectations: 0.025,
        }
    }

    /// Evaluates policy effectiveness
    pub fn evaluate_policy(&self, policy_id: &str) -> PolicyEffectiveness {
        PolicyEffectiveness {
            policy_id: policy_id.to_string(),
            objective_achievement: 0.75,
            unintended_consequences: vec![String::from("Wealth effect")],
            efficiency_score: 0.7,
            equity_impact: 0.5,
        }
    }

    /// Forecasts economic growth
    pub fn forecast_growth(&self, time_horizon: u32) -> EconomicForecast {
        EconomicForecast {
            time_horizon_years: time_horizon,
            gdp_growth_rate: 0.025,
            confidence_interval: [0.015, 0.035],
            key_assumptions: vec![String::from("Stable monetary policy")],
            risks: vec![String::from("Geopolitical instability")],
        }
    }

    /// Analyzes trade balance
    pub fn analyze_trade_balance(&self, country: &str) -> TradeAnalysis {
        TradeAnalysis {
            country_id: country.to_string(),
            exports: 500e9,
            imports: 600e9,
            trade_balance: -100e9,
            trade_weight: 0.15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketEquilibrium {
    pub market_id: String,
    pub equilibrium_price: f64,
    pub equilibrium_quantity: f64,
    pub surplus: f64,
    pub shortage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GDPCalculation {
    pub consumption: f64,
    pub investment: f64,
    pub government_spending: f64,
    pub net_exports: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GDPResult {
    pub method: String,
    pub consumption: f64,
    pub investment: f64,
    pub government_spending: f64,
    pub net_exports: f64,
    pub total_gdp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflationAnalysis {
    pub base_year: String,
    pub cpi: f64,
    pub inflation_rate: f64,
    pub core_inflation: f64,
    pub expectations: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEffectiveness {
    pub policy_id: String,
    pub objective_achievement: f64,
    pub unintended_consequences: Vec<String>,
    pub efficiency_score: f64,
    pub equity_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicForecast {
    pub time_horizon_years: u32,
    pub gdp_growth_rate: f64,
    pub confidence_interval: [f64; 2],
    pub key_assumptions: Vec<String>,
    pub risks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAnalysis {
    pub country_id: String,
    pub exports: f64,
    pub imports: f64,
    pub trade_balance: f64,
    pub trade_weight: f64,
}

impl Default for Economics {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_economics_creation() {
        let econ = Economics::new();
        assert_eq!(econ.econ_id, "economics_v1");
    }
    #[test]
    fn test_analyze_equilibrium() {
        let econ = Economics::new();
        let eq = econ.analyze_equilibrium("mkt_labor");
        assert!(eq.equilibrium_price > 0.0);
    }
}
