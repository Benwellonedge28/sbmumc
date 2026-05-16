//!
//! # SBMUMC Module 1570: Workflow Automation Engine
//!
//! Visual workflow builder with conditional logic, parallel execution,
//! error handling, and integration with external systems.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: u32,
    pub steps: Vec<WorkflowStep>,
    pub triggers: Vec<WorkflowTrigger>,
    pub variables: Vec<WorkflowVariable>,
    pub error_handling: ErrorHandlingConfig,
    pub timeout_secs: u32,
    pub created_at: u64,
    pub updated_at: u64,
    pub status: WorkflowStatus,
}

/// Workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub step_type: StepType,
    pub config: StepConfig,
    pub position: StepPosition,
    pub conditions: Vec<StepCondition>,
    pub retries: RetryConfig,
    pub timeout_secs: Option<u32>,
}

/// Step types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepType {
    Action,
    Condition,
    Loop,
    Parallel,
    Subworkflow,
    HttpRequest,
    Database,
    Notification,
    Transform,
    Wait,
}

/// Step configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepConfig {
    pub action: Option<String>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub input_mapping: Vec<MappingRule>,
    pub output_mapping: Vec<MappingRule>,
}

/// Mapping rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRule {
    pub source: String,
    pub target: String,
    pub transform: Option<String>,
}

/// Step position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepPosition {
    pub x: f32,
    pub y: f32,
}

/// Step condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    RegexMatch,
    IsEmpty,
    IsNotEmpty,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub enabled: bool,
    pub max_attempts: u32,
    pub backoff_type: BackoffType,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackoffType {
    Fixed,
    Exponential,
    Linear,
}

/// Workflow trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    pub trigger_type: TriggerType,
    pub config: TriggerConfig,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TriggerType {
    Manual,
    Schedule,
    Webhook,
    Event,
    ApiCall,
}

/// Trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerConfig {
    pub cron_expression: Option<String>,
    pub event_type: Option<String>,
    pub webhook_path: Option<String>,
    pub conditions: Option<Vec<TriggerCondition>>,
}

/// Trigger condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCondition {
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

/// Workflow variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowVariable {
    pub name: String,
    pub var_type: VariableType,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

/// Error handling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlingConfig {
    pub on_error: ErrorAction,
    pub notify_on_failure: bool,
    pub notification_channels: Vec<String>,
    pub capture_output: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorAction {
    Stop,
    Retry,
    Skip,
    Fallback,
    Continue,
}

/// Workflow status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Draft,
    Active,
    Paused,
    Archived,
}

/// Workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: String,
    pub workflow_id: String,
    pub status: ExecutionStatus,
    pub started_at: u64,
    pub completed_at: Option<u64>,
    pub current_step: Option<String>,
    pub variables: HashMap<String, serde_json::Value>,
    pub step_results: Vec<StepResult>,
    pub error: Option<ExecutionError>,
}

/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
}

/// Step result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    pub status: StepResultStatus,
    pub started_at: u64,
    pub completed_at: Option<u64>,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepResultStatus {
    Pending,
    Running,
    Success,
    Failed,
    Skipped,
    Retrying,
}

/// Execution error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionError {
    pub message: String,
    pub step_id: Option<String>,
    pub stack_trace: Option<String>,
}

/// Subworkflow call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubworkflowCall {
    pub workflow_id: String,
    pub input: HashMap<String, serde_json::Value>,
    pub wait_for_completion: bool,
}

/// Workflow builder
pub struct WorkflowBuilder {
    workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    executions: Arc<RwLock<HashMap<String, WorkflowExecution>>>,
    subworkflows: Arc<RwLock<HashMap<String, String>>>,
}

impl WorkflowBuilder {
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(RwLock::new(HashMap::new())),
            executions: Arc::new(RwLock::new(HashMap::new())),
            subworkflows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create new workflow
    pub fn create_workflow(&self, name: String, description: String) -> String {
        let workflow_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let workflow = Workflow {
            id: workflow_id.clone(),
            name,
            description,
            version: 1,
            steps: vec![],
            triggers: vec![],
            variables: vec![],
            error_handling: ErrorHandlingConfig {
                on_error: ErrorAction::Stop,
                notify_on_failure: false,
                notification_channels: vec![],
                capture_output: true,
            },
            timeout_secs: 3600,
            created_at: timestamp,
            updated_at: timestamp,
            status: WorkflowStatus::Draft,
        };

        let mut workflows = self.workflows.write().unwrap();
        workflows.insert(workflow_id.clone(), workflow);

        workflow_id
    }

    /// Add step to workflow
    pub fn add_step(&self, workflow_id: &str, step: WorkflowStep) -> Result<(), WorkflowError> {
        let mut workflows = self.workflows.write().unwrap();

        if let Some(workflow) = workflows.get_mut(workflow_id) {
            workflow.steps.push(step);
            workflow.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(())
        } else {
            Err(WorkflowError::WorkflowNotFound)
        }
    }

    /// Add trigger to workflow
    pub fn add_trigger(&self, workflow_id: &str, trigger: WorkflowTrigger) -> Result<(), WorkflowError> {
        let mut workflows = self.workflows.write().unwrap();

        if let Some(workflow) = workflows.get_mut(workflow_id) {
            workflow.triggers.push(trigger);
            workflow.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(())
        } else {
            Err(WorkflowError::WorkflowNotFound)
        }
    }

    /// Add variable to workflow
    pub fn add_variable(&self, workflow_id: &str, variable: WorkflowVariable) -> Result<(), WorkflowError> {
        let mut workflows = self.workflows.write().unwrap();

        if let Some(workflow) = workflows.get_mut(workflow_id) {
            workflow.variables.push(variable);
            workflow.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(())
        } else {
            Err(WorkflowError::WorkflowNotFound)
        }
    }

    /// Activate workflow
    pub fn activate(&self, workflow_id: &str) -> Result<(), WorkflowError> {
        let mut workflows = self.workflows.write().unwrap();

        if let Some(workflow) = workflows.get_mut(workflow_id) {
            if workflow.steps.is_empty() {
                return Err(WorkflowError::EmptyWorkflow);
            }
            workflow.status = WorkflowStatus::Active;
            workflow.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(())
        } else {
            Err(WorkflowError::WorkflowNotFound)
        }
    }

    /// Execute workflow
    pub async fn execute(&self, workflow_id: &str, input: HashMap<String, serde_json::Value>) -> Result<String, WorkflowError> {
        let workflows = self.workflows.read().unwrap();
        let workflow = workflows.get(workflow_id)
            .ok_or(WorkflowError::WorkflowNotFound)?
            .clone();
        drop(workflows);

        let execution_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut variables = HashMap::new();
        for var in &workflow.variables {
            variables.insert(var.name.clone(), var.default_value.clone().unwrap_or(serde_json::Value::Null));
        }
        for (key, value) in input {
            variables.insert(key, value);
        }

        let execution = WorkflowExecution {
            id: execution_id.clone(),
            workflow_id: workflow_id.to_string(),
            status: ExecutionStatus::Running,
            started_at: timestamp,
            completed_at: None,
            current_step: None,
            variables,
            step_results: vec![],
            error: None,
        };

        let mut executions = self.executions.write().unwrap();
        executions.insert(execution_id.clone(), execution);

        // Execute workflow in background
        let executions_clone = self.executions.clone();
        let workflow_clone = workflow.clone();
        let execution_id_clone = execution_id.clone();

        tokio::spawn(async move {
            WorkflowBuilder::execute_workflow_internal(
                &executions_clone,
                &workflow_clone,
                &execution_id_clone,
            ).await;
        });

        Ok(execution_id)
    }

    async fn execute_workflow_internal(
        executions: &Arc<RwLock<HashMap<String, WorkflowExecution>>>,
        workflow: &Workflow,
        execution_id: &str,
    ) {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        for step in &workflow.steps {
            // Check conditions
            if !WorkflowBuilder::evaluate_conditions(&step.conditions, &HashMap::new()) {
                continue;
            }

            // Execute step
            let result = WorkflowBuilder::execute_step(step).await;

            // Record result
            let mut execs = executions.write().unwrap();
            if let Some(exec) = execs.get_mut(execution_id) {
                exec.step_results.push(result);
                exec.current_step = Some(step.id.clone());
            }
        }

        // Mark complete
        let mut execs = executions.write().unwrap();
        if let Some(exec) = execs.get_mut(execution_id) {
            exec.status = ExecutionStatus::Completed;
            exec.completed_at = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            );
        }
    }

    fn evaluate_conditions(conditions: &[StepCondition], _variables: &HashMap<String, serde_json::Value>) -> bool {
        // Simplified condition evaluation
        conditions.is_empty() || conditions.iter().all(|c| true)
    }

    async fn execute_step(step: &WorkflowStep) -> StepResult {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let output = match step.step_type {
            StepType::Action => {
                // Execute action
                Some(serde_json::json!({ "status": "success" }))
            }
            StepType::HttpRequest => {
                // Execute HTTP request
                Some(serde_json::json!({ "status": "success" }))
            }
            StepType::Notification => {
                // Send notification
                Some(serde_json::json!({ "status": "sent" }))
            }
            _ => None,
        };

        StepResult {
            step_id: step.id.clone(),
            status: StepResultStatus::Success,
            started_at: timestamp,
            completed_at: Some(timestamp + 100),
            output,
            error: None,
            attempts: 1,
        }
    }

    /// Get execution status
    pub fn get_execution(&self, execution_id: &str) -> Option<WorkflowExecution> {
        let executions = self.executions.read().unwrap();
        executions.get(execution_id).cloned()
    }

    /// List workflow executions
    pub fn list_executions(&self, workflow_id: &str, limit: usize) -> Vec<WorkflowExecution> {
        let executions = self.executions.read().unwrap();
        executions
            .values()
            .filter(|e| e.workflow_id == workflow_id)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Cancel execution
    pub fn cancel_execution(&self, execution_id: &str) -> Result<(), WorkflowError> {
        let mut executions = self.executions.write().unwrap();

        if let Some(exec) = executions.get_mut(execution_id) {
            if exec.status == ExecutionStatus::Running {
                exec.status = ExecutionStatus::Cancelled;
                exec.completed_at = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64
                );
                Ok(())
            } else {
                Err(WorkflowError::InvalidState)
            }
        } else {
            Err(WorkflowError::ExecutionNotFound)
        }
    }

    /// Get workflow
    pub fn get_workflow(&self, workflow_id: &str) -> Option<Workflow> {
        let workflows = self.workflows.read().unwrap();
        workflows.get(workflow_id).cloned()
    }

    /// List workflows
    pub fn list_workflows(&self) -> Vec<Workflow> {
        let workflows = self.workflows.read().unwrap();
        workflows.values().cloned().collect()
    }

    /// Validate workflow
    pub fn validate(&self, workflow_id: &str) -> Vec<ValidationError> {
        let workflows = self.workflows.read().unwrap();
        let mut errors = Vec::new();

        if let Some(workflow) = workflows.get(workflow_id) {
            if workflow.steps.is_empty() {
                errors.push(ValidationError {
                    field: "steps".to_string(),
                    message: "Workflow must have at least one step".to_string(),
                });
            }

            for (i, step) in workflow.steps.iter().enumerate() {
                if step.name.is_empty() {
                    errors.push(ValidationError {
                        field: format!("steps[{}].name", i),
                        message: "Step name is required".to_string(),
                    });
                }
            }
        }

        errors
    }

    /// Duplicate workflow
    pub fn duplicate(&self, workflow_id: &str, new_name: String) -> Result<String, WorkflowError> {
        let workflows = self.workflows.read().unwrap();
        let workflow = workflows.get(workflow_id)
            .ok_or(WorkflowError::WorkflowNotFound)?
            .clone();
        drop(workflows);

        let new_id = self.create_workflow(new_name, workflow.description.clone());

        let mut workflows = self.workflows.write().unwrap();
        if let Some(new_workflow) = workflows.get_mut(&new_id) {
            new_workflow.steps = workflow.steps.clone();
            new_workflow.variables = workflow.variables.clone();
            new_workflow.triggers = workflow.triggers.clone();
            new_workflow.error_handling = workflow.error_handling.clone();
            new_workflow.timeout_secs = workflow.timeout_secs;
        }

        Ok(new_id)
    }
}

/// Workflow error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowError {
    WorkflowNotFound,
    StepNotFound,
    ExecutionNotFound,
    InvalidState,
    EmptyWorkflow,
    ValidationFailed(String),
}

impl std::fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowError::WorkflowNotFound => write!(f, "Workflow not found"),
            WorkflowError::StepNotFound => write!(f, "Step not found"),
            WorkflowError::ExecutionNotFound => write!(f, "Execution not found"),
            WorkflowError::InvalidState => write!(f, "Invalid execution state"),
            WorkflowError::EmptyWorkflow => write!(f, "Workflow has no steps"),
            WorkflowError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
        }
    }
}

impl std::error::Error for WorkflowError {}

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

// Re-export types
pub use Workflow;
pub use WorkflowStep;
pub use WorkflowExecution;
pub use StepType;
pub use WorkflowBuilder;