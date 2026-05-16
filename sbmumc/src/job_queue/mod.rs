//!
//! # SBMUMC Module 1573: Job Queue and Task Scheduling
//!
//! Distributed job queue with priority scheduling, retries,
//! dead letter queue, and worker management.

use std::collections::BinaryHeap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Job definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub job_type: String,
    pub payload: serde_json::Value,
    pub priority: JobPriority,
    pub scheduled_at: Option<u64>,
    pub max_retries: u32,
    pub timeout_secs: u32,
    pub created_at: u64,
    pub attempts: u32,
    pub status: JobStatus,
    pub metadata: HashMap<String, String>,
}

/// Job priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JobPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

/// Job status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    DeadLetter,
}

/// Job result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    pub job_id: String,
    pub success: bool,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub executed_at: u64,
    pub duration_ms: u64,
}

/// Dead letter job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterJob {
    pub job: Job,
    pub failure_reason: String,
    pub last_error: String,
    pub failed_at: u64,
    pub failure_count: u32,
}

/// Worker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    pub id: String,
    pub name: String,
    pub job_types: Vec<String>,
    pub concurrency: u32,
    pub poll_interval_ms: u64,
    pub heartbeat_interval_secs: u32,
}

/// Worker status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStatus {
    pub worker_id: String,
    pub status: WorkerState,
    pub current_jobs: Vec<String>,
    pub processed_count: u64,
    pub failed_count: u64,
    pub last_heartbeat: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerState {
    Idle,
    Busy,
    Paused,
    Offline,
}

/// Job queue
pub struct JobQueue {
    jobs: Arc<RwLock<BinaryHeap<QueuedJob>>>,
    running: Arc<RwLock<HashMap<String, Job>>>,
    completed: Arc<RwLock<HashMap<String, JobResult>>>,
    dead_letter: Arc<RwLock<Vec<DeadLetterJob>>>,
    workers: Arc<RwLock<HashMap<String, WorkerStatus>>>,
}

#[derive(Clone)]
struct QueuedJob {
    job: Job,
    priority: JobPriority,
    queue_time: u64,
}

impl PartialEq for QueuedJob {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.queue_time == other.queue_time
    }
}

impl Eq for QueuedJob {}

impl PartialOrd for QueuedJob {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedJob {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first (lower number)
        let priority_cmp = self.priority.cmp(&other.priority);
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp.reverse();
        }
        // Earlier queue time first
        self.queue_time.cmp(&other.queue_time)
    }
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(BinaryHeap::new())),
            running: Arc::new(RwLock::new(HashMap::new())),
            completed: Arc::new(RwLock::new(HashMap::new())),
            dead_letter: Arc::new(RwLock::new(Vec::new())),
            workers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Enqueue a job
    pub fn enqueue(&self, job: Job) -> String {
        let job_id = job.id.clone();
        let queue_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let queued = QueuedJob {
            job: job.clone(),
            priority: job.priority.clone(),
            queue_time,
        };

        let mut jobs = self.jobs.write().unwrap();
        jobs.push(queued);

        job_id
    }

    /// Dequeue next job
    pub fn dequeue(&self, worker_id: &str, job_types: &[String]) -> Option<Job> {
        let mut jobs = self.jobs.write().unwrap();

        // Find a job matching worker's job types
        let mut found_index = None;
        for (i, queued) in jobs.iter().enumerate() {
            if job_types.contains(&queued.job.job_type) {
                found_index = Some(i);
                break;
            }
        }

        if let Some(index) = found_index {
            // Remove the job from heap (inefficient but simple)
            let mut temp: BinaryHeap<QueuedJob> = BinaryHeap::new();
            let mut selected = None;

            while let Some(item) = jobs.pop() {
                if item.job.id == jobs.iter().nth(index).map(|j| j.job.id.clone()).unwrap_or_default() {
                    selected = Some(item);
                } else {
                    temp.push(item);
                }
            }

            // Restore remaining jobs
            while let Some(item) = temp.pop() {
                jobs.push(item);
            }

            if let Some(queued) = selected {
                let mut running = self.running.write().unwrap();
                running.insert(queued.job.id.clone(), queued.job.clone());
                return Some(queued.job);
            }
        }

        None
    }

    /// Complete job
    pub fn complete(&self, job_id: &str, result: JobResult) -> Result<(), QueueError> {
        let mut running = self.running.write().unwrap();
        let job = running.remove(job_id).ok_or(QueueError::JobNotFound)?;

        let mut completed = self.completed.write().unwrap();
        completed.insert(job_id.to_string(), result);

        Ok(())
    }

    /// Fail job
    pub fn fail(&self, job_id: &str, error: String) -> Result<JobStatus, QueueError> {
        let mut running = self.running.write().unwrap();

        if let Some(job) = running.remove(job_id) {
            let mut failed_job = job.clone();
            failed_job.attempts += 1;

            if failed_job.attempts >= failed_job.max_retries {
                // Move to dead letter queue
                let mut dead_letter = self.dead_letter.write().unwrap();
                dead_letter.push(DeadLetterJob {
                    job: failed_job.clone(),
                    failure_reason: "Max retries exceeded".to_string(),
                    last_error: error,
                    failed_at: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                    failure_count: failed_job.attempts,
                });

                Ok(JobStatus::DeadLetter)
            } else {
                // Re-queue for retry
                let mut jobs = self.jobs.write().unwrap();
                jobs.push(QueuedJob {
                    job: failed_job,
                    priority: JobPriority::Normal, // Lower priority for retries
                    queue_time: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                });

                Ok(JobStatus::Failed)
            }
        } else {
            Err(QueueError::JobNotFound)
        }
    }

    /// Cancel job
    pub fn cancel(&self, job_id: &str) -> Result<(), QueueError> {
        let mut running = self.running.write().unwrap();

        if let Some(mut job) = running.remove(job_id) {
            job.status = JobStatus::Cancelled;
            return Ok(());
        }

        // Check queued jobs
        let mut jobs = self.jobs.write().unwrap();
        let mut filtered: BinaryHeap<QueuedJob> = BinaryHeap::new();

        while let Some(queued) = jobs.pop() {
            if queued.job.id != job_id {
                filtered.push(queued);
            }
        }

        *jobs = filtered;

        Ok(())
    }

    /// Get job status
    pub fn get_status(&self, job_id: &str) -> Option<JobStatus> {
        let running = self.running.read().unwrap();
        if let Some(job) = running.get(job_id) {
            return Some(job.status.clone());
        }

        let completed = self.completed.read().unwrap();
        if completed.contains_key(job_id) {
            return Some(JobStatus::Completed);
        }

        let dead_letter = self.dead_letter.read().unwrap();
        if dead_letter.iter().any(|d| d.job.id == job_id) {
            return Some(JobStatus::DeadLetter);
        }

        None
    }

    /// Get dead letter jobs
    pub fn get_dead_letter(&self, limit: usize) -> Vec<DeadLetterJob> {
        let dead_letter = self.dead_letter.read().unwrap();
        dead_letter.iter().take(limit).cloned().collect()
    }

    /// Replay dead letter job
    pub fn replay_dead_letter(&self, job_id: &str) -> Result<String, QueueError> {
        let mut dead_letter = self.dead_letter.write().unwrap();

        if let Some(pos) = dead_letter.iter().position(|d| d.job.id == job_id) {
            let dl_job = dead_letter.remove(pos);
            let mut job = dl_job.job.clone();
            job.attempts = 0;
            job.status = JobStatus::Queued;

            drop(dead_letter);
            Ok(self.enqueue(job))
        } else {
            Err(QueueError::JobNotFound)
        }
    }

    /// Register worker
    pub fn register_worker(&self, config: WorkerConfig) {
        let mut workers = self.workers.write().unwrap();
        workers.insert(config.id.clone(), WorkerStatus {
            worker_id: config.id,
            status: WorkerState::Idle,
            current_jobs: vec![],
            processed_count: 0,
            failed_count: 0,
            last_heartbeat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        });
    }

    /// Heartbeat worker
    pub fn heartbeat(&self, worker_id: &str) -> Result<(), QueueError> {
        let mut workers = self.workers.write().unwrap();

        if let Some(status) = workers.get_mut(worker_id) {
            status.last_heartbeat = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(())
        } else {
            Err(QueueError::WorkerNotFound)
        }
    }

    /// Get queue stats
    pub fn get_stats(&self) -> QueueStats {
        let jobs = self.jobs.read().unwrap();
        let running = self.running.read().unwrap();
        let completed = self.completed.read().unwrap();
        let dead_letter = self.dead_letter.read().unwrap();
        let workers = self.workers.read().unwrap();

        QueueStats {
            queued: jobs.len(),
            running: running.len(),
            completed: completed.len(),
            dead_letter: dead_letter.len(),
            workers_online: workers.values().filter(|w| w.status != WorkerState::Offline).count(),
            workers_busy: workers.values().filter(|w| w.status == WorkerState::Busy).count(),
        }
    }

    /// Get job result
    pub fn get_result(&self, job_id: &str) -> Option<JobResult> {
        let completed = self.completed.read().unwrap();
        completed.get(job_id).cloned()
    }

    /// Clear completed jobs older than
    pub fn cleanup(&self, older_than_secs: u64) -> usize {
        let cutoff = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 - (older_than_secs * 1000);

        let mut completed = self.completed.write().unwrap();
        let initial = completed.len();

        completed.retain(|_, result| result.executed_at >= cutoff);

        initial - completed.len()
    }
}

/// Queue statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    pub queued: usize,
    pub running: usize,
    pub completed: usize,
    pub dead_letter: usize,
    pub workers_online: usize,
    pub workers_busy: usize,
}

/// Queue error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueError {
    JobNotFound,
    WorkerNotFound,
    QueueFull,
    InvalidJob,
}

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueError::JobNotFound => write!(f, "Job not found"),
            QueueError::WorkerNotFound => write!(f, "Worker not found"),
            QueueError::QueueFull => write!(f, "Queue is full"),
            QueueError::InvalidJob => write!(f, "Invalid job"),
        }
    }
}

impl std::error::Error for QueueError {}

// Re-export types
pub use Job;
pub use JobPriority;
pub use JobResult;
pub use DeadLetterJob;
pub use JobQueue;