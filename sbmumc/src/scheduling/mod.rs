//!
//! # SBMUMC Module 1583: Task Scheduling and Cron Management
//!
//! Advanced task scheduling with cron expressions, one-time tasks,
//! periodic jobs, and task dependencies.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Scheduled task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub task_type: TaskType,
    pub schedule: TaskSchedule,
    pub handler: TaskHandler,
    pub config: TaskConfig,
    pub retry_policy: RetryPolicy,
    pub notifications: NotificationConfig,
    pub created_at: u64,
    pub updated_at: u64,
    pub last_run: Option<u64>,
    pub next_run: Option<u64>,
    pub run_count: u32,
}

/// Task types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    Periodic,
    OneTime,
    Cron,
    Delayed,
    Dependent,
}

/// Task schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSchedule {
    pub schedule_type: ScheduleType,
    pub cron_expression: Option<String>,
    pub interval_secs: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScheduleType {
    Interval,
    Cron,
    OneTime,
    Manual,
}

/// Task handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHandler {
    pub handler_type: HandlerType,
    pub command: Option<String>,
    pub script: Option<String>,
    pub function_name: Option<String>,
    pub webhook_url: Option<String>,
}

/// Handler types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HandlerType {
    Command,
    Script,
    HTTP,
    Function,
    Queue,
}

/// Task configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    pub timeout_secs: u32,
    pub max_concurrent: u32,
    pub allow_overlap: bool,
    pub catch_errors: bool,
    pub environment: HashMap<String, String>,
}

/// Retry policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub enabled: bool,
    pub max_attempts: u32,
    pub initial_delay_secs: u64,
    pub max_delay_secs: u64,
    pub backoff_multiplier: f64,
    pub retry_on_errors: Vec<String>,
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub on_success: bool,
    pub on_failure: bool,
    pub on_timeout: bool,
    pub channels: Vec<NotificationChannel>,
    pub recipients: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationChannel {
    Email,
    Slack,
    Webhook,
    SMS,
}

/// Task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecution {
    pub id: String,
    pub task_id: String,
    pub status: ExecutionState,
    pub started_at: u64,
    pub completed_at: Option<u64>,
    pub output: Option<String>,
    pub error: Option<String>,
    pub exit_code: Option<i32>,
    pub attempt: u32,
}

/// Execution state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionState {
    Pending,
    Running,
    Success,
    Failed,
    Timeout,
    Cancelled,
    Skipped,
}

/// Task dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    pub task_id: String,
    pub depends_on: Vec<String>,
    pub wait_for_completion: bool,
}

/// Task group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskGroup {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tasks: Vec<String>,
    pub execution_mode: GroupExecutionMode,
    pub created_at: u64,
}

/// Group execution mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupExecutionMode {
    Sequential,
    Parallel,
    FanOut,
    FanIn,
}

/// Task scheduler
pub struct TaskScheduler {
    tasks: Arc<RwLock<HashMap<String, ScheduledTask>>>,
    executions: Arc<RwLock<HashMap<String, TaskExecution>>>,
    groups: Arc<RwLock<HashMap<String, TaskGroup>>>,
    dependencies: Arc<RwLock<HashMap<String, TaskDependency>>>,
    cron_parser: Arc<RwLock<CronParser>>,
    running_tasks: Arc<RwLock<HashSet<String>>>,
}

#[derive(Debug, Clone)]
struct CronParser;

impl CronParser {
    fn parse(&self, expression: &str) -> Result<CronSchedule, CronError> {
        // Simplified cron parsing
        let parts: Vec<&str> = expression.split_whitespace().collect();

        if parts.len() < 5 {
            return Err(CronError::InvalidExpression);
        }

        Ok(CronSchedule {
            minute: parts[0].to_string(),
            hour: parts[1].to_string(),
            day_of_month: parts[2].to_string(),
            month: parts[3].to_string(),
            day_of_week: parts[4].to_string(),
        })
    }

    fn next_run(&self, expression: &str, from: u64) -> Option<u64> {
        // Simplified next run calculation
        if let Ok(_schedule) = self.parse(expression) {
            Some(from + 3600) // Add 1 hour
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct CronSchedule {
    pub minute: String,
    pub hour: String,
    pub day_of_month: String,
    pub month: String,
    pub day_of_week: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CronError {
    InvalidExpression,
    InvalidField,
}

impl std::fmt::Display for CronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CronError::InvalidExpression => write!(f, "Invalid cron expression"),
            CronError::InvalidField => write!(f, "Invalid cron field"),
        }
    }
}

impl std::error::Error for CronError {}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            executions: Arc::new(RwLock::new(HashMap::new())),
            groups: Arc::new(RwLock::new(HashMap::new())),
            dependencies: Arc::new(RwLock::new(HashMap::new())),
            cron_parser: Arc::new(RwLock::new(CronParser)),
            running_tasks: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Create task
    pub fn create_task(&self, task: ScheduledTask) -> Result<String, SchedulerError> {
        // Validate schedule
        match task.schedule.schedule_type {
            ScheduleType::Cron => {
                if task.schedule.cron_expression.is_none() {
                    return Err(SchedulerError::MissingCronExpression);
                }
            }
            ScheduleType::Interval => {
                if task.schedule.interval_secs.is_none() {
                    return Err(SchedulerError::MissingInterval);
                }
            }
            _ => {}
        }

        let mut tasks = self.tasks.write().unwrap();
        tasks.insert(task.id.clone(), task.clone());

        // Calculate next run time
        if let Some(next) = self.calculate_next_run(&task) {
            let mut t = task;
            t.next_run = Some(next);
            tasks.insert(task.id.clone(), t);
        }

        Ok(task.id)
    }

    /// Update task
    pub fn update_task(&self, task_id: &str, updates: TaskUpdates) -> Result<(), SchedulerError> {
        let mut tasks = self.tasks.write().unwrap();

        if let Some(task) = tasks.get_mut(task_id) {
            if let Some(name) = updates.name {
                task.name = name;
            }
            if let Some(schedule) = updates.schedule {
                task.schedule = schedule;
            }
            if let Some(config) = updates.config {
                task.config = config;
            }
            task.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            // Recalculate next run
            if let Some(next) = self.calculate_next_run(task) {
                task.next_run = Some(next);
            }

            Ok(())
        } else {
            Err(SchedulerError::TaskNotFound)
        }
    }

    /// Delete task
    pub fn delete_task(&self, task_id: &str) -> Result<(), SchedulerError> {
        let mut tasks = self.tasks.write().unwrap();
        let mut deps = self.dependencies.write().unwrap();

        // Check if task is running
        {
            let running = self.running_tasks.read().unwrap();
            if running.contains(task_id) {
                return Err(SchedulerError::TaskRunning);
            }
        }

        // Remove task
        tasks.remove(task_id);

        // Clean up dependencies
        deps.retain(|_, d| !d.depends_on.contains(&task_id.to_string()));

        Ok(())
    }

    /// Execute task immediately
    pub async fn execute_task(&self, task_id: &str) -> Result<String, SchedulerError> {
        let tasks = self.tasks.read().unwrap();
        let task = tasks.get(task_id)
            .ok_or(SchedulerError::TaskNotFound)?
            .clone();
        drop(tasks);

        // Check dependencies
        let deps = self.dependencies.read().unwrap();
        if let Some(dep) = deps.get(task_id) {
            for dep_id in &dep.depends_on {
                let execs = self.executions.read().unwrap();
                let last_exec = execs.values()
                    .filter(|e| e.task_id == *dep_id && e.status == ExecutionState::Success)
                    .last();

                if last_exec.is_none() {
                    return Err(SchedulerError::DependencyNotMet);
                }
            }
        }

        // Create execution
        let execution_id = Uuid::new_v4().to_string();
        let execution = TaskExecution {
            id: execution_id.clone(),
            task_id: task_id.to_string(),
            status: ExecutionState::Pending,
            started_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            completed_at: None,
            output: None,
            error: None,
            exit_code: None,
            attempt: 1,
        };

        {
            let mut execs = self.executions.write().unwrap();
            execs.insert(execution_id.clone(), execution);
        }

        // Mark as running
        {
            let mut running = self.running_tasks.write().unwrap();
            running.insert(task_id.to_string());
        }

        // Execute (simulated)
        let result = self.run_task(&task).await;

        // Mark as complete
        {
            let mut running = self.running_tasks.write().unwrap();
            running.remove(task_id);
        }

        // Update execution
        {
            let mut execs = self.executions.write().unwrap();
            if let Some(exec) = execs.get_mut(&execution_id) {
                exec.status = if result.is_ok() { ExecutionState::Success } else { ExecutionState::Failed };
                exec.completed_at = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64
                );
                if let Err(e) = &result {
                    exec.error = Some(e.to_string());
                }
            }
        }

        // Update task stats
        {
            let mut tasks = self.tasks.write().unwrap();
            if let Some(t) = tasks.get_mut(task_id) {
                t.last_run = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64
                );
                t.run_count += 1;
            }
        }

        Ok(execution_id)
    }

    async fn run_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        match task.handler.handler_type {
            HandlerType::Command => {
                // Execute command
                Ok(())
            }
            HandlerType::Script => {
                // Execute script
                Ok(())
            }
            HandlerType::HTTP => {
                // Call webhook
                Ok(())
            }
            HandlerType::Function => {
                // Call function
                Ok(())
            }
            HandlerType::Queue => {
                // Enqueue to job queue
                Ok(())
            }
        }
    }

    /// Cancel execution
    pub fn cancel_execution(&self, execution_id: &str) -> Result<(), SchedulerError> {
        let mut execs = self.executions.write().unwrap();

        if let Some(exec) = execs.get_mut(execution_id) {
            if exec.status == ExecutionState::Running {
                exec.status = ExecutionState::Cancelled;
                exec.completed_at = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64
                );
                Ok(())
            } else {
                Err(SchedulerError::InvalidState)
            }
        } else {
            Err(SchedulerError::ExecutionNotFound)
        }
    }

    /// Get due tasks
    pub fn get_due_tasks(&self) -> Vec<ScheduledTask> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let tasks = self.tasks.read().unwrap();

        tasks.values()
            .filter(|t| {
                if let Some(next) = t.next_run {
                    next <= now
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }

    /// Create task group
    pub fn create_group(&self, group: TaskGroup) -> String {
        let mut groups = self.groups.write().unwrap();
        groups.insert(group.id.clone(), group.clone());
        group.id
    }

    /// Execute group
    pub async fn execute_group(&self, group_id: &str) -> Result<GroupExecutionResult, SchedulerError> {
        let groups = self.groups.read().unwrap();
        let group = groups.get(group_id)
            .ok_or(SchedulerError::GroupNotFound)?
            .clone();
        drop(groups);

        let mut results = Vec::new();

        match group.execution_mode {
            GroupExecutionMode::Sequential => {
                for task_id in &group.tasks {
                    let result = self.execute_task(task_id).await;
                    results.push(GroupTaskResult {
                        task_id: task_id.clone(),
                        success: result.is_ok(),
                        execution_id: result.ok(),
                    });
                }
            }
            GroupExecutionMode::Parallel => {
                // Would spawn tasks in parallel
                for task_id in &group.tasks {
                    results.push(GroupTaskResult {
                        task_id: task_id.clone(),
                        success: true,
                        execution_id: None,
                    });
                }
            }
            _ => {}
        }

        Ok(GroupExecutionResult {
            group_id: group_id.to_string(),
            task_results: results,
            success: results.iter().all(|r| r.success),
        })
    }

    /// Get execution
    pub fn get_execution(&self, execution_id: &str) -> Option<TaskExecution> {
        let execs = self.executions.read().unwrap();
        execs.get(execution_id).cloned()
    }

    /// Get task executions
    pub fn get_task_executions(&self, task_id: &str, limit: usize) -> Vec<TaskExecution> {
        let execs = self.executions.read().unwrap();
        execs.values()
            .filter(|e| e.task_id == task_id)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// List tasks
    pub fn list_tasks(&self) -> Vec<ScheduledTask> {
        let tasks = self.tasks.read().unwrap();
        tasks.values().cloned().collect()
    }

    fn calculate_next_run(&self, task: &ScheduledTask) -> Option<u64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        match task.schedule.schedule_type {
            ScheduleType::Interval => {
                task.schedule.interval_secs.map(|interval| now + interval * 1000)
            }
            ScheduleType::Cron => {
                task.schedule.cron_expression.as_ref().and_then(|expr| {
                    let parser = self.cron_parser.read().unwrap();
                    parser.next_run(expr, now)
                })
            }
            ScheduleType::OneTime => {
                task.schedule.start_time
            }
            ScheduleType::Manual => None,
        }
    }

    /// Set dependency
    pub fn set_dependency(&self, task_id: String, depends_on: Vec<String>) {
        let mut deps = self.dependencies.write().unwrap();
        deps.insert(task_id, TaskDependency {
            task_id,
            depends_on,
            wait_for_completion: true,
        });
    }
}

/// Task updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskUpdates {
    pub name: Option<String>,
    pub schedule: Option<TaskSchedule>,
    pub config: Option<TaskConfig>,
    pub enabled: Option<bool>,
}

/// Group execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupExecutionResult {
    pub group_id: String,
    pub task_results: Vec<GroupTaskResult>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTaskResult {
    pub task_id: String,
    pub success: bool,
    pub execution_id: Option<String>,
}

/// Scheduler error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulerError {
    TaskNotFound,
    TaskRunning,
    ExecutionNotFound,
    GroupNotFound,
    DependencyNotMet,
    MissingCronExpression,
    MissingInterval,
    InvalidState,
    ExecutionError(String),
}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchedulerError::TaskNotFound => write!(f, "Task not found"),
            SchedulerError::TaskRunning => write!(f, "Task is currently running"),
            SchedulerError::ExecutionNotFound => write!(f, "Execution not found"),
            SchedulerError::GroupNotFound => write!(f, "Task group not found"),
            SchedulerError::DependencyNotMet => write!(f, "Task dependency not met"),
            SchedulerError::MissingCronExpression => write!(f, "Cron expression required"),
            SchedulerError::MissingInterval => write!(f, "Interval required"),
            SchedulerError::InvalidState => write!(f, "Invalid task state"),
            SchedulerError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
        }
    }
}

impl std::error::Error for SchedulerError {}

// Re-export types
pub use ScheduledTask;
pub use TaskExecution;
pub use TaskGroup;
pub use TaskScheduler;