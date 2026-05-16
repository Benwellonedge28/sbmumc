//!
//! # SBMUMC Module 1572: Notification System
//!
//! Multi-channel notification service with templates, scheduling,
//! personalization, and delivery tracking.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Notification channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationChannel {
    Email,
    SMS,
    Push,
    Webhook,
    Slack,
    Discord,
    Teams,
    Custom(String),
}

/// Notification priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationPriority {
    Critical,
    High,
    Normal,
    Low,
}

/// Notification message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub channel: NotificationChannel,
    pub recipient: String,
    pub subject: Option<String>,
    pub body: String,
    pub priority: NotificationPriority,
    pub template_id: Option<String>,
    pub template_data: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
    pub scheduled_at: Option<u64>,
    pub created_at: u64,
    pub sent_at: Option<u64>,
    pub delivered_at: Option<u64>,
    pub status: NotificationStatus,
    pub retries: u32,
}

/// Notification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationStatus {
    Pending,
    Scheduled,
    Sending,
    Sent,
    Delivered,
    Failed,
    Cancelled,
}

/// Notification template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub channel: NotificationChannel,
    pub subject_template: Option<String>,
    pub body_template: String,
    pub variables: Vec<TemplateVariable>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Template variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub var_type: VariableType,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub user_id: String,
    pub channel_preferences: HashMap<NotificationChannel, ChannelPreference>,
    pub quiet_hours: Option<QuietHours>,
    pub aggregation: AggregationPreference,
}

/// Channel preference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPreference {
    pub enabled: bool,
    pub address: String,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuietHours {
    pub start: String, // HH:MM
    pub end: String,
    pub timezone: String,
    pub days: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationPreference {
    pub enabled: bool,
    pub batch_interval_mins: u32,
    pub max_batch_size: u32,
}

/// Delivery receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryReceipt {
    pub notification_id: String,
    pub status: NotificationStatus,
    pub timestamp: u64,
    pub details: Option<String>,
    pub error_code: Option<String>,
}

/// Notification service
pub struct NotificationService {
    notifications: Arc<RwLock<HashMap<String, Notification>>>,
    templates: Arc<RwLock<HashMap<String, NotificationTemplate>>>,
    preferences: Arc<RwLock<HashMap<String, NotificationPreferences>>>,
    channels: Arc<RwLock<HashMap<NotificationChannel, ChannelHandler>>>,
    queue: Arc<RwLock<Vec<Notification>>>,
}

impl NotificationService {
    pub fn new() -> Self {
        let service = Self {
            notifications: Arc::new(RwLock::new(HashMap::new())),
            templates: Arc::new(RwLock::new(HashMap::new())),
            preferences: Arc::new(RwLock::new(HashMap::new())),
            channels: Arc::new(RwLock::new(HashMap::new())),
            queue: Arc::new(RwLock::new(Vec::new())),
        };

        service.init_default_templates();
        service
    }

    /// Register channel handler
    pub fn register_channel(&self, channel: NotificationChannel, handler: ChannelHandler) {
        let mut channels = self.channels.write().unwrap();
        channels.insert(channel, handler);
    }

    /// Create template
    pub fn create_template(&self, template: NotificationTemplate) -> String {
        let mut templates = self.templates.write().unwrap();
        templates.insert(template.id.clone(), template.clone());
        template.id
    }

    /// Send notification
    pub async fn send(&self, notification: Notification) -> Result<String, NotificationError> {
        let notification_id = notification.id.clone();

        // Check user preferences
        if let Some(prefs) = self.get_preferences(&notification.recipient) {
            if let Some(channel_pref) = prefs.channel_preferences.get(&notification.channel) {
                if !channel_pref.enabled {
                    return Err(NotificationError::ChannelDisabled);
                }

                // Check quiet hours
                if let Some(quiet) = &prefs.quiet_hours {
                    if self.is_in_quiet_hours(quiet) {
                        let mut notif = notification.clone();
                        notif.status = NotificationStatus::Scheduled;
                        self.schedule_notification(notif)?;
                        return Ok(notification_id);
                    }
                }
            }
        }

        // Queue for sending
        let mut notif = notification.clone();
        notif.status = NotificationStatus::Pending;

        {
            let mut notifications = self.notifications.write().unwrap();
            notifications.insert(notification_id.clone(), notif.clone());
        }

        // Process immediately
        self.process_notification(&mut notif).await?;

        Ok(notification_id)
    }

    async fn process_notification(&self, notification: &mut Notification) -> Result<(), NotificationError> {
        notification.status = NotificationStatus::Sending;

        // Get channel handler
        let channels = self.channels.read().unwrap();
        let handler = channels.get(&notification.channel)
            .ok_or(NotificationError::ChannelNotSupported)?;

        // Send via channel
        let result = handler.send(notification).await;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        match result {
            Ok(()) => {
                notification.status = NotificationStatus::Sent;
                notification.sent_at = Some(timestamp);
            }
            Err(e) => {
                notification.status = NotificationStatus::Failed;
                notification.retries += 1;

                if notification.retries < 3 {
                    // Re-queue for retry
                    let mut queue = self.queue.write().unwrap();
                    queue.push(notification.clone());
                }

                return Err(e);
            }
        }

        Ok(())
    }

    /// Send using template
    pub async fn send_with_template(
        &self,
        template_id: &str,
        recipient: String,
        channel: NotificationChannel,
        template_data: HashMap<String, serde_json::Value>,
    ) -> Result<String, NotificationError> {
        let templates = self.templates.read().unwrap();
        let template = templates.get(template_id)
            .ok_or(NotificationError::TemplateNotFound)?
            .clone();
        drop(templates);

        let body = self.render_template(&template.body_template, &template_data);
        let subject = template.subject_template
            .as_ref()
            .map(|s| self.render_template(s, &template_data));

        let notification = Notification {
            id: Uuid::new_v4().to_string(),
            channel: template.channel.clone(),
            recipient,
            subject,
            body,
            priority: NotificationPriority::Normal,
            template_id: Some(template_id.to_string()),
            template_data,
            metadata: HashMap::new(),
            scheduled_at: None,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            sent_at: None,
            delivered_at: None,
            status: NotificationStatus::Pending,
            retries: 0,
        };

        self.send(notification).await
    }

    fn render_template(&self, template: &str, data: &HashMap<String, serde_json::Value>) -> String {
        let mut result = template.to_string();

        for (key, value) in data {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &value_str);
        }

        result
    }

    /// Schedule notification
    pub fn schedule_notification(&self, notification: Notification) -> Result<String, NotificationError> {
        let notification_id = notification.id.clone();

        let mut notifications = self.notifications.write().unwrap();
        notifications.insert(notification_id.clone(), notification.clone());

        let mut queue = self.queue.write().unwrap();
        queue.push(notification);

        Ok(notification_id)
    }

    /// Cancel notification
    pub fn cancel(&self, notification_id: &str) -> Result<(), NotificationError> {
        let mut notifications = self.notifications.write().unwrap();

        if let Some(notif) = notifications.get_mut(notification_id) {
            if notif.status == NotificationStatus::Pending || notif.status == NotificationStatus::Scheduled {
                notif.status = NotificationStatus::Cancelled;
                Ok(())
            } else {
                Err(NotificationError::CannotCancel)
            }
        } else {
            Err(NotificationError::NotFound)
        }
    }

    /// Get notification status
    pub fn get_status(&self, notification_id: &str) -> Result<NotificationStatus, NotificationError> {
        let notifications = self.notifications.read().unwrap();

        notifications
            .get(notification_id)
            .map(|n| n.status.clone())
            .ok_or(NotificationError::NotFound)
    }

    /// Set user preferences
    pub fn set_preferences(&self, preferences: NotificationPreferences) {
        let mut prefs = self.preferences.write().unwrap();
        prefs.insert(preferences.user_id.clone(), preferences);
    }

    /// Get user preferences
    pub fn get_preferences(&self, user_id: &str) -> Option<NotificationPreferences> {
        let prefs = self.preferences.read().unwrap();
        prefs.get(user_id).cloned()
    }

    /// Get notification history
    pub fn get_history(&self, recipient: &str, limit: usize) -> Vec<Notification> {
        let notifications = self.notifications.read().unwrap();

        notifications
            .values()
            .filter(|n| n.recipient == recipient)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    fn is_in_quiet_hours(&self, quiet: &QuietHours) -> bool {
        let now = SystemTime::now();
        let offset = chrono::offset::Utc::now;
        let current_time = format!("{:02}:{:02}", 0, 0); // Simplified

        current_time >= quiet.start && current_time <= quiet.end
    }

    fn init_default_templates(&self) {
        let templates = vec![
            NotificationTemplate {
                id: "welcome_email".to_string(),
                name: "Welcome Email".to_string(),
                channel: NotificationChannel::Email,
                subject_template: Some("Welcome to {{company_name}}!".to_string()),
                body_template: "Hello {{user_name}},\n\nWelcome to {{company_name}}! We're excited to have you.\n\nBest regards,\n{{company_name}} Team".to_string(),
                variables: vec![
                    TemplateVariable {
                        name: "user_name".to_string(),
                        var_type: VariableType::String,
                        required: true,
                        default_value: None,
                    },
                    TemplateVariable {
                        name: "company_name".to_string(),
                        var_type: VariableType::String,
                        required: false,
                        default_value: Some("SBMUMC".to_string()),
                    },
                ],
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                updated_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            },
            NotificationTemplate {
                id: "password_reset".to_string(),
                name: "Password Reset".to_string(),
                channel: NotificationChannel::Email,
                subject_template: Some("Reset your password".to_string()),
                body_template: "Hello,\n\nClick the link below to reset your password:\n\n{{reset_link}}\n\nThis link expires in {{expiry_hours}} hours.\n\nIf you didn't request this, please ignore this email.".to_string(),
                variables: vec![
                    TemplateVariable {
                        name: "reset_link".to_string(),
                        var_type: VariableType::String,
                        required: true,
                        default_value: None,
                    },
                    TemplateVariable {
                        name: "expiry_hours".to_string(),
                        var_type: VariableType::Number,
                        required: false,
                        default_value: Some("24".to_string()),
                    },
                ],
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                updated_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            },
        ];

        let mut template_store = self.templates.write().unwrap();
        for template in templates {
            template_store.insert(template.id.clone(), template);
        }
    }

    /// Process queue (called by background worker)
    pub async fn process_queue(&self) {
        let mut queue = self.queue.write().unwrap();
        let notifications: Vec<Notification> = queue.drain(..).collect();
        drop(queue);

        for mut notification in notifications {
            if notification.status == NotificationStatus::Pending {
                self.process_notification(&mut notification).await.ok();
            }
        }
    }
}

/// Channel handler trait
#[async_trait::async_trait]
pub trait ChannelHandler: Send + Sync {
    async fn send(&self, notification: &Notification) -> Result<(), NotificationError>;
    async fn verify(&self, address: &str) -> Result<bool, NotificationError>;
}

/// Email channel handler
pub struct EmailChannelHandler {
    smtp_config: SmtpConfig,
}

impl EmailChannelHandler {
    pub fn new(smtp_config: SmtpConfig) -> Self {
        Self { smtp_config }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_tls: bool,
}

#[async_trait::async_trait]
impl ChannelHandler for EmailChannelHandler {
    async fn send(&self, notification: &Notification) -> Result<(), NotificationError> {
        // In production, use lettre or similar to send email
        println!("Sending email to {}: {}", notification.recipient, notification.body);
        Ok(())
    }

    async fn verify(&self, address: &str) -> Result<bool, NotificationError> {
        Ok(address.contains('@'))
    }
}

/// Slack channel handler
pub struct SlackChannelHandler {
    webhook_url: String,
}

impl SlackChannelHandler {
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url }
    }
}

#[async_trait::async_trait]
impl ChannelHandler for SlackChannelHandler {
    async fn send(&self, notification: &Notification) -> Result<(), NotificationError> {
        // In production, use slack SDK to send message
        println!("Sending Slack message to {}: {}", notification.recipient, notification.body);
        Ok(())
    }

    async fn verify(&self, address: &str) -> Result<bool, NotificationError> {
        Ok(!address.is_empty())
    }
}

/// Notification error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationError {
    NotFound,
    TemplateNotFound,
    ChannelNotSupported,
    ChannelDisabled,
    CannotCancel,
    SendFailed(String),
    InvalidAddress,
}

impl std::fmt::Display for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationError::NotFound => write!(f, "Notification not found"),
            NotificationError::TemplateNotFound => write!(f, "Template not found"),
            NotificationError::ChannelNotSupported => write!(f, "Channel not supported"),
            NotificationError::ChannelDisabled => write!(f, "Channel disabled for user"),
            NotificationError::CannotCancel => write!(f, "Cannot cancel notification"),
            NotificationError::SendFailed(msg) => write!(f, "Send failed: {}", msg),
            NotificationError::InvalidAddress => write!(f, "Invalid notification address"),
        }
    }
}

impl std::error::Error for NotificationError {}

// Re-export types
pub use Notification;
pub use NotificationTemplate;
pub use NotificationChannel;
pub use NotificationService;