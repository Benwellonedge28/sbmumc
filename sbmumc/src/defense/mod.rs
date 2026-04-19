//!
//! Defense Module - Pillar II: Counter-Intelligence & Defense (Files 38-54)
//!
//! This module implements defensive capabilities:
//! - Brand rotation and public mask management
//! - Agency invoicing for forensic access fees
//! - Intrusion detection and countermeasures
//! - Revenue generation through defensive mechanisms

use crate::core::{Result, EntityId, SbmumcError};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug, warn};
use serde::{Serialize, Deserialize};

/// Defense Engine - Pillar II
pub struct DefenseEngine {
    /// Brand manager
    brand_manager: Arc<BrandManager>,

    /// Agency invoicing
    invoicing: Arc<AgencyInvoicing>,

    /// Counter-intelligence
    counter_intel: Arc<CounterIntelligence>,

    /// Attack detection
    attack_detection: Arc<AttackDetection>,

    /// Countermeasures
    countermeasures: Arc<Countermeasures>,

    /// Configuration
    config: DefenseConfig,
}

/// Brand Manager
pub struct BrandManager {
    /// Available brands
    brands: RwLock<Vec<Brand>>,

    /// Active brand
    active_brand: RwLock<String>,

    /// Brand history
    history: RwLock<Vec<BrandSwitch>>,

    /// Perception tracker
    perception: RwLock<PerceptionTracker>,
}

/// Brand definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    pub id: String,
    pub name: String,
    pub tagline: String,
    pub description: String,
    pub public_face: PublicFace,
    pub appearance: BrandAppearance,
    pub compliance_mode: ComplianceMode,
    pub jurisdictions: Vec<String>,
    pub reputation_score: f64,
}

/// Public face
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicFace {
    pub category: BrandCategory,
    pub use_case: Vec<String>,
    pub target_audience: Vec<String>,
    pub marketing_message: String,
    pub legal_standing: String,
}

/// Brand category
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BrandCategory {
    SafetyUtility,
    CommunicationPlatform,
    EnterpriseSoftware,
    HealthcareTech,
    FinancialServices,
    Logistics,
    IoT,
    Custom,
}

/// Brand appearance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandAppearance {
    pub logo_url: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub typography: String,
    pub iconography: String,
    pub tone_of_voice: String,
}

/// Compliance mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ComplianceMode {
    FullCompliance,
    RegionalCompliance,
    MinimalCompliance,
    Stealth,
}

/// Brand switch record
#[derive(Debug, Clone)]
pub struct BrandSwitch {
    pub from_brand: String,
    pub to_brand: String,
    pub reason: SwitchReason,
    pub timestamp: u64,
    pub automatic: bool,
}

/// Switch reason
#[derive(Debug, Clone, Copy)]
pub enum SwitchReason {
    BlacklistRisk,
    LegalPressure,
    Scheduled,
    Emergency,
    Optimization,
}

/// Perception tracker
#[derive(Debug, Clone)]
pub struct PerceptionTracker {
    pub social_sentiment: f64,
    pub media_coverage: HashMap<String, f64>,
    pub threat_level: ThreatLevel,
    pub last_assessment: u64,
}

/// Threat level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    Minimal,
    Low,
    Medium,
    High,
    Critical,
}

/// Agency Invoicing
pub struct AgencyInvoicing {
    /// Active invoices
    invoices: RwLock<HashMap<String, Invoice>>,

    /// Agency registry
    agencies: RwLock<HashMap<String, AgencyInfo>>,

    /// Payment tracking
    payments: RwLock<HashMap<String, PaymentRecord>>,

    /// Billing rules
    rules: RwLock<Vec<BillingRule>>,

    /// Revenue tracker
    revenue: RwLock<RevenueTracker>,
}

/// Invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String,
    pub agency_id: String,
    pub agency_name: String,
    pub service_type: ServiceType,
    pub description: String,
    pub amount: InvoiceAmount,
    pub issued_at: u64,
    pub due_date: u64,
    pub status: InvoiceStatus,
    pub legal_basis: String,
    pub compliance_notes: String,
}

/// Service type
#[derive(Debug, Clone, Copy)]
pub enum ServiceType {
    ForensicAccess,
    DataIntegrity,
    CybersecurityConsultation,
    TechnicalCooperation,
    ResearchPartnership,
}

/// Invoice amount
#[derive(Debug, Clone)]
pub struct InvoiceAmount {
    pub value: f64,
    pub currency: String,
    pub breakdown: Vec<LineItem>,
}

/// Line item
#[derive(Debug, Clone)]
pub struct LineItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
}

/// Invoice status
#[derive(Debug, Clone, Copy)]
pub enum InvoiceStatus {
    Pending,
    Sent,
    Viewed,
    Disputed,
    Paid,
    Overdue,
    WrittenOff,
}

/// Agency info
#[derive(Debug, Clone)]
pub struct AgencyInfo {
    pub id: String,
    pub name: String,
    pub agency_type: AgencyType,
    pub jurisdiction: String,
    pub contact_info: ContactInfo,
    pub billing_history: Vec<String>,
    pub risk_score: f64,
}

/// Agency type
#[derive(Debug, Clone, Copy)]
pub enum AgencyType {
    LawEnforcement,
    Intelligence,
    Regulatory,
    Judicial,
    Military,
    Private,
}

/// Contact info
#[derive(Debug, Clone)]
pub struct ContactInfo {
    pub name: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub case_number: Option<String>,
}

/// Payment record
#[derive(Debug, Clone)]
pub struct PaymentRecord {
    pub invoice_id: String,
    pub amount_paid: f64,
    pub currency: String,
    pub method: PaymentMethod,
    pub transaction_id: Option<String>,
    pub paid_at: u64,
    pub notes: String,
}

/// Payment method
#[derive(Debug, Clone, Copy)]
pub enum PaymentMethod {
    WireTransfer,
    Crypto,
    Escrow,
    Barter,
    Unpaid,
}

/// Billing rule
#[derive(Debug, Clone)]
pub struct BillingRule {
    pub id: String,
    pub service_type: ServiceType,
    pub agency_type: AgencyType,
    pub base_rate: f64,
    pub currency: String,
    pub minimum: f64,
    pub maximum: Option<f64>,
    pub formula: BillingFormula,
}

/// Billing formula
#[derive(Debug, Clone)]
pub struct BillingFormula {
    pub formula_type: FormulaType,
    pub base_value: f64,
    pub multiplier: f64,
    pub cap: Option<f64>,
}

/// Formula type
#[derive(Debug, Clone, Copy)]
pub enum FormulaType {
    Flat,
    Hourly,
    DataVolume,
    Complexity,
    Custom,
}

/// Revenue tracker
#[derive(Debug, Clone)]
pub struct RevenueTracker {
    pub total_revenue: f64,
    pub currency: String,
    pub by_service_type: HashMap<ServiceType, f64>,
    pub by_agency: HashMap<String, f64>,
    pub pending_amount: f64,
    pub received_amount: f64,
}

/// Counter-Intelligence
pub struct CounterIntelligence {
    /// Monitoring targets
    targets: RwLock<HashMap<String, MonitorTarget>>,

    /// Active operations
    operations: RwLock<HashMap<String, IntelOperation>>,

    /// Threat intelligence
    threat_intel: Arc<ThreatIntelligence>,

    /// Deception systems
    deception: Arc<DeceptionSystem>,
}

/// Monitor target
#[derive(Debug, Clone)]
pub struct MonitorTarget {
    pub id: String,
    pub target_type: TargetType,
    pub name: String,
    pub indicators: Vec<Indicator>,
    pub risk_level: RiskLevel,
    pub last_activity: u64,
    pub notes: String,
}

/// Target type
#[derive(Debug, Clone, Copy)]
pub enum TargetType {
    Agency,
    ThreatActor,
    Competitor,
    Media,
    Unknown,
}

/// Indicator
#[derive(Debug, Clone)]
pub struct Indicator {
    pub indicator_type: IndicatorType,
    pub value: String,
    pub confidence: f64,
    pub first_seen: u64,
    pub last_seen: u64,
}

/// Indicator type
#[derive(Debug, Clone, Copy)]
pub enum IndicatorType {
    IPAddress,
    Domain,
    Email,
    Hash,
    Pattern,
    Behavior,
}

/// Risk level
#[derive(Debug, Clone, Copy)]
pub enum RiskLevel {
    Negligible,
    Low,
    Medium,
    High,
    Critical,
}

/// Intel operation
#[derive(Debug, Clone)]
pub struct IntelOperation {
    pub id: String,
    pub operation_type: OperationType,
    pub target_id: String,
    pub status: OperationStatus,
    pub started_at: u64,
    pub objectives: Vec<String>,
    pub findings: Vec<Finding>,
}

/// Operation type
#[derive(Debug, Clone, Copy)]
pub enum OperationType {
    PassiveMonitoring,
    ActiveReconnaissance,
    Attribution,
    AttributionDenied,
    Honeypot,
}

/// Operation status
#[derive(Debug, Clone, Copy)]
pub enum OperationStatus {
    Planning,
    Active,
    Paused,
    Completed,
    Cancelled,
}

/// Finding
#[derive(Debug, Clone)]
pub struct Finding {
    pub finding_id: String,
    pub description: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub timestamp: u64,
}

/// Threat Intelligence
pub struct ThreatIntelligence {
    /// Known threat actors
    actors: RwLock<HashMap<String, ThreatActor>>,

    /// Known attack patterns
    patterns: RwLock<Vec<AttackPattern>>,

    /// IOC database
    iocs: RwLock<HashMap<String, IOC>>>,

    /// MITRE ATT&CK mapping
    mitre_mapping: RwLock<HashMap<String, String>>,

    /// Configuration
    config: ThreatIntelConfig,
}

/// Threat actor
#[derive(Debug, Clone)]
pub struct ThreatActor {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub attribution: Attribution,
    pub capabilities: Vec<String>,
    pub intent: String,
    pub active_since: u64,
    pub last_activity: u64,
}

/// Attribution
#[derive(Debug, Clone)]
pub struct Attribution {
    pub confidence: f64,
    pub country: Option<String>,
    pub organization: Option<String>,
    pub individuals: Vec<String>,
    pub funding: Option<String>,
}

/// Attack pattern
#[derive(Debug, Clone)]
pub struct AttackPattern {
    pub id: String,
    pub name: String,
    pub mitre_id: Option<String>,
    pub description: String,
    pub indicators: Vec<String>,
    pub severity: Severity,
    pub defenses: Vec<String>,
}

/// Severity
#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// IOC (Indicator of Compromise)
#[derive(Debug, Clone)]
pub struct IOC {
    pub id: String,
    pub indicator: String,
    pub ioc_type: IOCType,
    pub threat_actor: Option<String>,
    pub first_seen: u64,
    pub last_seen: u64,
    pub confidence: f64,
    pub action_required: bool,
}

/// IOC type
#[derive(Debug, Clone, Copy)]
pub enum IOCType {
    Malware,
    C2,
    Phishing,
    Exploit,
    Infrastructure,
    Tool,
}

/// Threat intel config
#[derive(Debug, Clone)]
pub struct ThreatIntelConfig {
    pub auto_update: bool,
    pub update_interval_hours: u32,
    pub share_with_mesh: bool,
    pub attribution_threshold: f64,
}

/// Deception System
pub struct DeceptionSystem {
    /// Honeypots
    honeypots: RwLock<HashMap<String, Honeypot>>>,

    /// Decoys
    decoys: RwLock<Vec<Decoy>>,

    /// Canary tokens
    canary_tokens: RwLock<HashMap<String, CanaryToken>>>,

    /// Deception campaigns
    campaigns: RwLock<Vec<DeceptionCampaign>>,

    /// Configuration
    config: DeceptionConfig,
}

/// Honeypot
#[derive(Debug, Clone)]
pub struct Honeypot {
    pub id: String,
    pub name: String,
    pub honeypot_type: HoneypotType,
    pub exposure_level: ExposureLevel,
    pub capture_rate: f64,
    pub intelligence_gathered: Vec<String>,
    pub status: HoneypotStatus,
}

/// Honeypot type
#[derive(Debug, Clone, Copy)]
pub enum HoneypotType {
    Low,
    Medium,
    High,
    Research,
}

/// Exposure level
#[derive(Debug, Clone, Copy)]
pub enum ExposureLevel {
    Hidden,
    Partial,
    Full,
}

/// Honeypot status
#[derive(Debug, Clone, Copy)]
pub enum HoneypotStatus {
    Active,
    Triggered,
    Dormant,
    Compromised,
}

/// Decoy
#[derive(Debug, Clone)]
pub struct Decoy {
    pub id: String,
    pub decoy_type: DecoyType,
    pub authenticity: f64,
    pub deployment_locations: Vec<String>,
}

/// Decoy type
#[derive(Debug, Clone, Copy)]
pub enum DecoyType {
    FakeFile,
    FakeCredential,
    FakeEndpoint,
    FakeService,
}

/// Canary token
#[derive(Debug, Clone)]
pub struct CanaryToken {
    pub id: String,
    pub token_type: TokenType,
    pub token_value: String,
    pub triggered: bool,
    pub triggered_at: Option<u64>,
    pub triggered_by: Option<String>,
    pub location: String,
}

/// Token type
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Document,
    URL,
    Email,
    AWSKey,
    APIKey,
    Password,
}

/// Deception campaign
#[derive(Debug, Clone)]
pub struct DeceptionCampaign {
    pub id: String,
    pub name: String,
    pub objective: String,
    pub resources: Vec<String>,
    pub status: CampaignStatus,
    pub effectiveness_score: f64,
}

/// Campaign status
#[derive(Debug, Clone, Copy)]
pub enum CampaignStatus {
    Planning,
    Active,
    Evaluated,
    Completed,
}

/// Deception config
#[derive(Debug, Clone)]
pub struct DeceptionConfig {
    pub enable_honeypots: bool,
    pub enable_decoys: bool,
    pub enable_canary_tokens: bool,
    pub deployment_strategy: String,
    pub refresh_interval_hours: u32,
}

/// Attack Detection
pub struct AttackDetection {
    /// Detection rules
    rules: RwLock<Vec<DetectionRule>>,

    /// Active alerts
    alerts: RwLock<HashMap<String, Alert>>,

    /// Detection models
    models: RwLock<HashMap<String, DetectionModel>>>,

    /// Configuration
    config: DetectionConfig,
}

/// Detection rule
#[derive(Debug, Clone)]
pub struct DetectionRule {
    pub id: String,
    pub name: String,
    pub rule_type: RuleType,
    pub condition: String,
    pub severity: Severity,
    pub enabled: bool,
    pub last_triggered: Option<u64>,
}

/// Rule type
#[derive(Debug, Clone, Copy)]
pub enum RuleType {
    Signature,
    Anomaly,
    Behavior,
    ML,
}

/// Detection model
#[derive(Debug, Clone)]
pub struct DetectionModel {
    pub id: String,
    pub model_type: ModelType,
    pub trained_on: u64,
    pub accuracy: f64,
    pub false_positive_rate: f64,
}

/// Model type
#[derive(Debug, Clone, Copy)]
pub enum ModelType {
    NeuralNetwork,
    RandomForest,
    SVM,
    Ensemble,
}

/// Detection config
#[derive(Debug, Clone)]
pub struct DetectionConfig {
    pub sensitivity: f64,
    pub auto_response: bool,
    pub alert_threshold: u32,
    pub investigation_timeout_minutes: u32,
}

/// Alert
#[derive(Debug, Clone)]
pub struct Alert {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub status: AlertStatus,
    pub source: String,
    pub indicators: Vec<String>,
    pub created_at: u64,
    pub assigned_to: Option<String>,
    pub resolution: Option<String>,
}

/// Alert status
#[derive(Debug, Clone, Copy)]
pub enum AlertStatus {
    New,
    Acknowledged,
    Investigating,
    Contained,
    Resolved,
    FalsePositive,
}

/// Countermeasures
pub struct Countermeasures {
    /// Active countermeasures
    active: RwLock<Vec<Countermeasure>>,

    /// Countermeasure templates
    templates: RwLock<HashMap<String, CountermeasureTemplate>>>,

    /// Response playbooks
    playbooks: RwLock<Vec<ResponsePlaybook>>,

    /// Configuration
    config: CountermeasureConfig,
}

/// Countermeasure
#[derive(Debug, Clone)]
pub struct Countermeasure {
    pub id: String,
    pub name: String,
    pub cm_type: CountermeasureType,
    pub target: String,
    pub status: CountermeasureStatus,
    pub deployed_at: u64,
    pub effectiveness: f64,
}

/// Countermeasure type
#[derive(Debug, Clone, Copy)]
pub enum CountermeasureType {
    Block,
    Redirect,
    SlowDown,
    Misdirect,
    Exhaust,
    Evict,
}

/// Countermeasure status
#[derive(Debug, Clone, Copy)]
pub enum CountermeasureStatus {
    Ready,
    Deploying,
    Active,
    Exhausted,
    Failed,
}

/// Countermeasure template
#[derive(Debug, Clone)]
pub struct CountermeasureTemplate {
    pub id: String,
    pub name: String,
    pub cm_type: CountermeasureType,
    pub parameters: HashMap<String, String>,
    pub cooldown_seconds: u64,
}

/// Response playbook
#[derive(Debug, Clone)]
pub struct ResponsePlaybook {
    pub id: String,
    pub name: String,
    pub trigger_condition: String,
    pub steps: Vec<PlaybookStep>,
    pub automated: bool,
    pub approval_required: bool,
}

/// Playbook step
#[derive(Debug, Clone)]
pub struct PlaybookStep {
    pub order: u32,
    pub action: String,
    pub target: String,
    pub timeout_seconds: u64,
}

/// Countermeasure config
#[derive(Debug, Clone)]
pub struct CountermeasureConfig {
    pub auto_deploy: bool,
    pub escalation_threshold: u32,
    pub max_concurrent: u32,
    pub legal_review_required: bool,
}

/// Defense Configuration
#[derive(Debug, Clone)]
pub struct DefenseConfig {
    pub brand_rotation_enabled: bool,
    pub agency_invoicing_enabled: bool,
    pub counter_intel_enabled: bool,
    pub auto_defense_enabled: bool,
    pub revenue_target_monthly: f64,
}

impl DefenseEngine {
    /// Create a new defense engine
    pub async fn new(config: DefenseConfig) -> Result<Self> {
        info!("Initializing Defense Engine - Pillar II");

        let brand_manager = Arc::new(BrandManager {
            brands: RwLock::new(vec![
                Brand {
                    id: "guardian".to_string(),
                    name: "GUARDIAN".to_string(),
                    tagline: "Safety Utility for the Digital Age".to_string(),
                    description: "Enterprise-grade safety and connectivity platform".to_string(),
                    public_face: PublicFace {
                        category: BrandCategory::SafetyUtility,
                        use_case: vec!["Personal Safety".to_string(), "Secure Communication".to_string()],
                        target_audience: vec!["Consumers".to_string(), "Small Business".to_string()],
                        marketing_message: "Protection you can trust".to_string(),
                        legal_standing: "Registered utility service".to_string(),
                    },
                    appearance: BrandAppearance {
                        logo_url: None,
                        primary_color: "#8B5CF6".to_string(),
                        secondary_color: "#EC4899".to_string(),
                        typography: "Inter".to_string(),
                        iconography: "Shield".to_string(),
                        tone_of_voice: "Professional".to_string(),
                    },
                    compliance_mode: ComplianceMode::FullCompliance,
                    jurisdictions: vec!["US".to_string(), "EU".to_string(), "UK".to_string()],
                    reputation_score: 0.85,
                },
            ]),
            active_brand: RwLock::new("guardian".to_string()),
            history: RwLock::new(Vec::new()),
            perception: RwLock::new(PerceptionTracker {
                social_sentiment: 0.75,
                media_coverage: HashMap::new(),
                threat_level: ThreatLevel::Low,
                last_assessment: current_timestamp(),
            }),
        });

        let invoicing = Arc::new(AgencyInvoicing {
            invoices: RwLock::new(HashMap::new()),
            agencies: RwLock::new(HashMap::new()),
            payments: RwLock::new(HashMap::new()),
            rules: RwLock::new(vec![
                BillingRule {
                    id: "forensic_access".to_string(),
                    service_type: ServiceType::ForensicAccess,
                    agency_type: AgencyType::LawEnforcement,
                    base_rate: 5000.0,
                    currency: "USD".to_string(),
                    minimum: 1000.0,
                    maximum: Some(100000.0),
                    formula: BillingFormula {
                        formula_type: FormulaType::Complexity,
                        base_value: 5000.0,
                        multiplier: 1.5,
                        cap: Some(100000.0),
                    },
                },
            ]),
            revenue: RwLock::new(RevenueTracker {
                total_revenue: 0.0,
                currency: "USD".to_string(),
                by_service_type: HashMap::new(),
                by_agency: HashMap::new(),
                pending_amount: 0.0,
                received_amount: 0.0,
            }),
        });

        let counter_intel = Arc::new(CounterIntelligence {
            targets: RwLock::new(HashMap::new()),
            operations: RwLock::new(HashMap::new()),
            threat_intel: Arc::new(ThreatIntelligence {
                actors: RwLock::new(HashMap::new()),
                patterns: RwLock::new(Vec::new()),
                iocs: RwLock::new(HashMap::new()),
                mitre_mapping: RwLock::new(HashMap::new()),
                config: ThreatIntelConfig {
                    auto_update: true,
                    update_interval_hours: 24,
                    share_with_mesh: true,
                    attribution_threshold: 0.7,
                },
            }),
            deception: Arc::new(DeceptionSystem {
                honeypots: RwLock::new(HashMap::new()),
                decoys: RwLock::new(Vec::new()),
                canary_tokens: RwLock::new(HashMap::new()),
                campaigns: RwLock::new(Vec::new()),
                config: DeceptionConfig {
                    enable_honeypots: true,
                    enable_decoys: true,
                    enable_canary_tokens: true,
                    deployment_strategy: "Distributed".to_string(),
                    refresh_interval_hours: 72,
                },
            }),
        });

        let attack_detection = Arc::new(AttackDetection {
            rules: RwLock::new(Vec::new()),
            alerts: RwLock::new(HashMap::new()),
            models: RwLock::new(HashMap::new()),
            config: DetectionConfig {
                sensitivity: 0.8,
                auto_response: true,
                alert_threshold: 3,
                investigation_timeout_minutes: 30,
            },
        });

        let countermeasures = Arc::new(Countermeasures {
            active: RwLock::new(Vec::new()),
            templates: RwLock::new(HashMap::new()),
            playbooks: RwLock::new(Vec::new()),
            config: CountermeasureConfig {
                auto_deploy: true,
                escalation_threshold: 5,
                max_concurrent: 10,
                legal_review_required: false,
            },
        });

        let engine = Self {
            brand_manager,
            invoicing,
            counter_intel,
            attack_detection,
            countermeasures,
            config,
        };

        // Initialize detection rules
        engine.initialize_detection_rules();

        info!("Defense Engine initialized");
        Ok(engine)
    }

    /// Initialize detection rules
    fn initialize_detection_rules(&self) {
        let rules = vec![
            DetectionRule {
                id: "port_scan".to_string(),
                name: "Port Scan Detection".to_string(),
                rule_type: RuleType::Signature,
                condition: "multiple_ports < 60s".to_string(),
                severity: Severity::Medium,
                enabled: true,
                last_triggered: None,
            },
            DetectionRule {
                id: "brute_force".to_string(),
                name: "Brute Force Detection".to_string(),
                rule_type: RuleType::Behavior,
                condition: "failed_auth > 10 < 5m".to_string(),
                severity: Severity::High,
                enabled: true,
                last_triggered: None,
            },
            DetectionRule {
                id: "data_exfil".to_string(),
                name: "Data Exfiltration".to_string(),
                rule_type: RuleType::Anomaly,
                condition: "volume > threshold * 3".to_string(),
                severity: Severity::Critical,
                enabled: true,
                last_triggered: None,
            },
        ];

        *self.attack_detection.rules.write() = rules;
    }

    /// Issue invoice to agency
    pub async fn issue_invoice(&self, agency_id: &str, service: ServiceType, description: &str) -> Result<String> {
        debug!("Issuing invoice to agency: {}", agency_id);

        let invoice_id = EntityId::new().to_string();
        let invoice = Invoice {
            id: invoice_id.clone(),
            invoice_number: format!("INV-{}", invoice_id[..8].to_uppercase()),
            agency_id: agency_id.to_string(),
            agency_name: "Agency".to_string(),
            service_type: service,
            description: description.to_string(),
            amount: InvoiceAmount {
                value: 5000.0,
                currency: "USD".to_string(),
                breakdown: vec![
                    LineItem {
                        description: description.to_string(),
                        quantity: 1.0,
                        unit_price: 5000.0,
                        total: 5000.0,
                    },
                ],
            },
            issued_at: current_timestamp(),
            due_date: current_timestamp() + 30 * 24 * 3600,
            status: InvoiceStatus::Pending,
            legal_basis: "Consultancy Services Agreement".to_string(),
            compliance_notes: "Per Forensic Partnership Protocol".to_string(),
        };

        self.invoicing.invoices.write().insert(invoice_id.clone(), invoice);

        Ok(invoice_id)
    }

    /// Switch brand
    pub async fn switch_brand(&self, brand_id: &str, reason: SwitchReason) -> Result<()> {
        info!("Switching brand to: {} (reason: {:?})", brand_id, reason);

        let current = self.brand_manager.active_brand.read().clone();
        *self.brand_manager.active_brand.write() = brand_id.to_string();

        self.brand_manager.history.write().push(BrandSwitch {
            from_brand: current,
            to_brand: brand_id.to_string(),
            reason,
            timestamp: current_timestamp(),
            automatic: false,
        });

        Ok(())
    }

    /// Deploy countermeasure
    pub async fn deploy_countermeasure(&self, cm_type: CountermeasureType, target: &str) -> Result<String> {
        debug!("Deploying countermeasure: {:?} against {}", cm_type, target);

        let cm_id = EntityId::new().to_string();
        let cm = Countermeasure {
            id: cm_id.clone(),
            name: format!("{:?} against {}", cm_type, target),
            cm_type,
            target: target.to_string(),
            status: CountermeasureStatus::Active,
            deployed_at: current_timestamp(),
            effectiveness: 0.0,
        };

        self.countermeasures.active.write().push(cm);

        Ok(cm_id)
    }

    /// Get defense status
    pub fn get_status(&self) -> DefenseStatus {
        let brands = self.brand_manager.brands.read();
        let invoices = self.invoicing.invoices.read();
        let alerts = self.attack_detection.alerts.read();
        let countermeasures = self.countermeasures.active.read();

        DefenseStatus {
            active_brand: self.brand_manager.active_brand.read().clone(),
            brand_count: brands.len(),
            pending_invoices: invoices.values().filter(|i| i.status == InvoiceStatus::Pending).count(),
            active_alerts: alerts.len(),
            deployed_countermeasures: countermeasures.len(),
            threat_level: *self.brand_manager.perception.read().threat_level,
        }
    }
}

/// Defense status
#[derive(Debug, Clone)]
pub struct DefenseStatus {
    pub active_brand: String,
    pub brand_count: usize,
    pub pending_invoices: usize,
    pub active_alerts: usize,
    pub deployed_countermeasures: usize,
    pub threat_level: ThreatLevel,
}

/// Get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl Default for DefenseConfig {
    fn default() -> Self {
        Self {
            brand_rotation_enabled: true,
            agency_invoicing_enabled: true,
            counter_intel_enabled: true,
            auto_defense_enabled: true,
            revenue_target_monthly: 100_000.0,
        }
    }
}
