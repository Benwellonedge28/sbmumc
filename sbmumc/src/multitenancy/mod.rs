//!
//! # SBMUMC Module 1577: Multi-Tenancy and Tenant Management
//!
//! Complete multi-tenant architecture with isolation, resource quotas,
//! tenant onboarding, and cross-tenant operations.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tenant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub plan: TenantPlan,
    pub status: TenantStatus,
    pub settings: TenantSettings,
    pub quotas: TenantQuotas,
    pub billing: BillingInfo,
    pub created_at: u64,
    pub updated_at: u64,
    pub metadata: HashMap<String, String>,
}

/// Tenant plans
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantPlan {
    Free,
    Starter,
    Professional,
    Enterprise,
    Custom(String),
}

/// Tenant status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantStatus {
    Trial,
    Active,
    Suspended,
    Terminated,
    Migration,
}

/// Tenant settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSettings {
    pub timezone: String,
    pub locale: String,
    pub theme: String,
    pub features: HashMap<String, bool>,
    pub branding: BrandingConfig,
    pub integrations: Vec<IntegrationConfig>,
}

/// Branding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingConfig {
    pub logo_url: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub custom_css: Option<String>,
}

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub integration_type: String,
    pub enabled: bool,
    pub config: HashMap<String, serde_json::Value>,
}

/// Resource quotas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantQuotas {
    pub max_users: u32,
    pub max_projects: u32,
    pub max_storage_mb: u64,
    pub max_api_calls: u64,
    pub max_bandwidth_mb: u64,
    pub max_concurrent_builds: u32,
}

/// Billing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInfo {
    pub billing_email: String,
    pub payment_method: PaymentMethod,
    pub cycle: BillingCycle,
    pub next_billing_date: Option<u64>,
    pub spent_this_cycle: f64,
}

/// Payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub method_type: PaymentType,
    pub last_four: Option<String>,
    pub expiry_month: Option<u32>,
    pub expiry_year: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentType {
    CreditCard,
    DebitCard,
    BankTransfer,
    PayPal,
    Custom(String),
}

/// Billing cycle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Annually,
}

/// Tenant user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantUser {
    pub id: String,
    pub tenant_id: String,
    pub user_id: String,
    pub role: TenantRole,
    pub permissions: Vec<String>,
    pub invited_by: Option<String>,
    pub joined_at: u64,
    pub last_active: Option<u64>,
}

/// Tenant roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantRole {
    Owner,
    Admin,
    Member,
    Guest,
    Custom(String),
}

/// Tenant project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantProject {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub description: String,
    pub visibility: ProjectVisibility,
    pub settings: ProjectSettings,
    pub created_at: u64,
    pub storage_used_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectVisibility {
    Private,
    Internal,
    Public,
}

/// Project settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub allow_guest_access: bool,
    pub require_approval: bool,
    pub auto_cleanup_days: Option<u32>,
    pub max_branch_size_mb: u64,
}

/// Resource usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub tenant_id: String,
    pub period: String,
    pub users_current: u32,
    pub projects_current: u32,
    pub storage_used_mb: u64,
    pub api_calls: u64,
    pub bandwidth_mb: u64,
    pub compute_hours: f64,
}

/// Tenant manager
pub struct TenantManager {
    tenants: Arc<RwLock<HashMap<String, Tenant>>>,
    users: Arc<RwLock<HashMap<String, TenantUser>>>,
    projects: Arc<RwLock<HashMap<String, TenantProject>>>,
    usage: Arc<RwLock<HashMap<String, ResourceUsage>>>,
}

impl TenantManager {
    pub fn new() -> Self {
        Self {
            tenants: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(HashMap::new())),
            projects: Arc::new(RwLock::new(HashMap::new())),
            usage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create tenant
    pub fn create_tenant(&self, name: String, slug: String, plan: TenantPlan, owner_email: String) -> Result<String, TenantError> {
        let tenant_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Check slug uniqueness
        {
            let tenants = self.tenants.read().unwrap();
            if tenants.values().any(|t| t.slug == slug) {
                return Err(TenantError::SlugAlreadyExists);
            }
        }

        let tenant = Tenant {
            id: tenant_id.clone(),
            name,
            slug,
            plan: plan.clone(),
            status: TenantStatus::Trial,
            settings: TenantSettings {
                timezone: "UTC".to_string(),
                locale: "en".to_string(),
                theme: "default".to_string(),
                features: HashMap::new(),
                branding: BrandingConfig {
                    logo_url: None,
                    primary_color: "#007bff".to_string(),
                    secondary_color: "#6c757d".to_string(),
                    custom_css: None,
                },
                integrations: vec![],
            },
            quotas: Self::get_default_quotas(&plan),
            billing: BillingInfo {
                billing_email: owner_email,
                payment_method: PaymentMethod {
                    method_type: PaymentType::CreditCard,
                    last_four: None,
                    expiry_month: None,
                    expiry_year: None,
                },
                cycle: BillingCycle::Monthly,
                next_billing_date: None,
                spent_this_cycle: 0.0,
            },
            created_at: timestamp,
            updated_at: timestamp,
            metadata: HashMap::new(),
        };

        let mut tenants = self.tenants.write().unwrap();
        tenants.insert(tenant_id.clone(), tenant);

        Ok(tenant_id)
    }

    fn get_default_quotas(plan: &TenantPlan) -> TenantQuotas {
        match plan {
            TenantPlan::Free => TenantQuotas {
                max_users: 3,
                max_projects: 2,
                max_storage_mb: 100,
                max_api_calls: 10000,
                max_bandwidth_mb: 1000,
                max_concurrent_builds: 1,
            },
            TenantPlan::Starter => TenantQuotas {
                max_users: 10,
                max_projects: 10,
                max_storage_mb: 1024,
                max_api_calls: 100000,
                max_bandwidth_mb: 10000,
                max_concurrent_builds: 3,
            },
            TenantPlan::Professional => TenantQuotas {
                max_users: 50,
                max_projects: 50,
                max_storage_mb: 10240,
                max_api_calls: 1000000,
                max_bandwidth_mb: 100000,
                max_concurrent_builds: 10,
            },
            TenantPlan::Enterprise => TenantQuotas {
                max_users: 500,
                max_projects: 500,
                max_storage_mb: 102400,
                max_api_calls: 10000000,
                max_bandwidth_mb: 1000000,
                max_concurrent_builds: 50,
            },
            TenantPlan::Custom(_) => TenantQuotas {
                max_users: 1000,
                max_projects: 1000,
                max_storage_mb: 1024000,
                max_api_calls: u64::MAX,
                max_bandwidth_mb: u64::MAX,
                max_concurrent_builds: 100,
            },
        }
    }

    /// Get tenant
    pub fn get_tenant(&self, tenant_id: &str) -> Option<Tenant> {
        let tenants = self.tenants.read().unwrap();
        tenants.get(tenant_id).cloned()
    }

    /// Update tenant
    pub fn update_tenant(&self, tenant_id: &str, updates: TenantUpdates) -> Result<(), TenantError> {
        let mut tenants = self.tenants.write().unwrap();

        if let Some(tenant) = tenants.get_mut(tenant_id) {
            if let Some(name) = updates.name {
                tenant.name = name;
            }
            if let Some(status) = updates.status {
                tenant.status = status;
            }
            if let Some(plan) = updates.plan {
                tenant.plan = plan;
                tenant.quotas = Self::get_default_quotas(&plan);
            }
            tenant.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(())
        } else {
            Err(TenantError::TenantNotFound)
        }
    }

    /// Add user to tenant
    pub fn add_user(&self, tenant_id: &str, user_id: String, role: TenantRole) -> Result<String, TenantError> {
        // Verify tenant exists
        {
            let tenants = self.tenants.read().unwrap();
            if !tenants.contains_key(tenant_id) {
                return Err(TenantError::TenantNotFound);
            }
        }

        // Check quota
        {
            let tenants = self.tenants.read().unwrap();
            let users = self.users.read().unwrap();
            if let Some(tenant) = tenants.get(tenant_id) {
                let current_users = users.values().filter(|u| u.tenant_id == tenant_id).count() as u32;
                if current_users >= tenant.quotas.max_users {
                    return Err(TenantError::QuotaExceeded("max_users".to_string()));
                }
            }
        }

        let user_id_str = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let tenant_user = TenantUser {
            id: user_id_str.clone(),
            tenant_id: tenant_id.to_string(),
            user_id,
            role: role.clone(),
            permissions: Self::get_role_permissions(&role),
            invited_by: None,
            joined_at: timestamp,
            last_active: None,
        };

        let mut users = self.users.write().unwrap();
        users.insert(user_id_str, tenant_user);

        Ok(user_id_str)
    }

    fn get_role_permissions(role: &TenantRole) -> Vec<String> {
        match role {
            TenantRole::Owner => vec![
                "tenant.manage".to_string(),
                "tenant.delete".to_string(),
                "users.manage".to_string(),
                "projects.manage".to_string(),
                "billing.manage".to_string(),
                "settings.manage".to_string(),
            ],
            TenantRole::Admin => vec![
                "users.manage".to_string(),
                "projects.manage".to_string(),
                "settings.manage".to_string(),
            ],
            TenantRole::Member => vec![
                "projects.view".to_string(),
                "projects.create".to_string(),
            ],
            TenantRole::Guest => vec![
                "projects.view".to_string(),
            ],
            TenantRole::Custom(name) => vec![format!("custom.{}", name)],
        }
    }

    /// Create project for tenant
    pub fn create_project(&self, tenant_id: &str, name: String, visibility: ProjectVisibility) -> Result<String, TenantError> {
        // Verify tenant
        {
            let tenants = self.tenants.read().unwrap();
            if !tenants.contains_key(tenant_id) {
                return Err(TenantError::TenantNotFound);
            }
        }

        // Check quota
        {
            let tenants = self.tenants.read().unwrap();
            let projects = self.projects.read().unwrap();
            if let Some(tenant) = tenants.get(tenant_id) {
                let current = projects.values().filter(|p| p.tenant_id == tenant_id).count() as u32;
                if current >= tenant.quotas.max_projects {
                    return Err(TenantError::QuotaExceeded("max_projects".to_string()));
                }
            }
        }

        let project_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let project = TenantProject {
            id: project_id.clone(),
            tenant_id: tenant_id.to_string(),
            name,
            description: String::new(),
            visibility,
            settings: ProjectSettings {
                allow_guest_access: false,
                require_approval: false,
                auto_cleanup_days: None,
                max_branch_size_mb: 500,
            },
            created_at: timestamp,
            storage_used_mb: 0,
        };

        let mut projects = self.projects.write().unwrap();
        projects.insert(project_id, project);

        Ok(project_id)
    }

    /// Get tenant usage
    pub fn get_usage(&self, tenant_id: &str) -> ResourceUsage {
        let tenants = self.tenants.read().unwrap();
        let users = self.users.read().unwrap();
        let projects = self.projects.read().unwrap();

        let tenant = tenants.get(tenant_id);
        let tenant_id_str = tenant_id.to_string();

        ResourceUsage {
            tenant_id: tenant_id_str.clone(),
            period: "current".to_string(),
            users_current: users.values().filter(|u| u.tenant_id == tenant_id_str).count() as u32,
            projects_current: projects.values().filter(|p| p.tenant_id == tenant_id_str).count() as u32,
            storage_used_mb: projects.values()
                .filter(|p| p.tenant_id == tenant_id_str)
                .map(|p| p.storage_used_mb)
                .sum(),
            api_calls: 0,
            bandwidth_mb: 0,
            compute_hours: 0.0,
        }
    }

    /// Check quota and enforce
    pub fn check_quota(&self, tenant_id: &str, resource: &str, amount: u64) -> Result<(), TenantError> {
        let tenants = self.tenants.read().unwrap();
        let usage = self.get_usage(tenant_id);

        if let Some(tenant) = tenants.get(tenant_id) {
            match resource {
                "users" => {
                    if (usage.users_current as u64 + amount) > tenant.quotas.max_users as u64 {
                        return Err(TenantError::QuotaExceeded("max_users".to_string()));
                    }
                }
                "projects" => {
                    if (usage.projects_current as u64 + amount) > tenant.quotas.max_projects as u64 {
                        return Err(TenantError::QuotaExceeded("max_projects".to_string()));
                    }
                }
                "storage" => {
                    if (usage.storage_used_mb as u64 + amount) > tenant.quotas.max_storage_mb {
                        return Err(TenantError::QuotaExceeded("max_storage_mb".to_string()));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Suspend tenant
    pub fn suspend(&self, tenant_id: &str, reason: String) -> Result<(), TenantError> {
        let mut tenants = self.tenants.write().unwrap();

        if let Some(tenant) = tenants.get_mut(tenant_id) {
            tenant.status = TenantStatus::Suspended;
            tenant.metadata.insert("suspension_reason".to_string(), reason);
            tenant.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(())
        } else {
            Err(TenantError::TenantNotFound)
        }
    }

    /// Delete tenant
    pub fn delete_tenant(&self, tenant_id: &str) -> Result<(), TenantError> {
        let mut tenants = self.tenants.write().unwrap();
        let mut users = self.users.write().unwrap();
        let mut projects = self.projects.write().unwrap();

        // Remove all users
        users.retain(|_, u| u.tenant_id != tenant_id);

        // Remove all projects
        projects.retain(|_, p| p.tenant_id != tenant_id);

        // Remove tenant
        if tenants.remove(tenant_id).is_some() {
            Ok(())
        } else {
            Err(TenantError::TenantNotFound)
        }
    }

    /// List all tenants
    pub fn list_tenants(&self, filters: Option<TenantFilters>) -> Vec<Tenant> {
        let tenants = self.tenants.read().unwrap();

        let mut result: Vec<Tenant> = tenants.values().cloned().collect();

        if let Some(f) = filters {
            if let Some(status) = f.status {
                result.retain(|t| t.status == status);
            }
            if let Some(plan) = f.plan {
                result.retain(|t| t.plan == plan);
            }
            if let Some(search) = f.search {
                let search_lower = search.to_lowercase();
                result.retain(|t| {
                    t.name.to_lowercase().contains(&search_lower) ||
                    t.slug.to_lowercase().contains(&search_lower)
                });
            }
        }

        result
    }

    /// Get users for tenant
    pub fn get_tenant_users(&self, tenant_id: &str) -> Vec<TenantUser> {
        let users = self.users.read().unwrap();
        users.values()
            .filter(|u| u.tenant_id == tenant_id)
            .cloned()
            .collect()
    }

    /// Get projects for tenant
    pub fn get_tenant_projects(&self, tenant_id: &str) -> Vec<TenantProject> {
        let projects = self.projects.read().unwrap();
        projects.values()
            .filter(|p| p.tenant_id == tenant_id)
            .cloned()
            .collect()
    }
}

/// Tenant updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantUpdates {
    pub name: Option<String>,
    pub status: Option<TenantStatus>,
    pub plan: Option<TenantPlan>,
    pub settings: Option<TenantSettings>,
}

/// Tenant filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantFilters {
    pub status: Option<TenantStatus>,
    pub plan: Option<TenantPlan>,
    pub search: Option<String>,
}

/// Tenant error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TenantError {
    TenantNotFound,
    SlugAlreadyExists,
    UserNotFound,
    ProjectNotFound,
    QuotaExceeded(String),
    PermissionDenied,
    InvalidOperation,
}

impl std::fmt::Display for TenantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TenantError::TenantNotFound => write!(f, "Tenant not found"),
            TenantError::SlugAlreadyExists => write!(f, "Slug already exists"),
            TenantError::UserNotFound => write!(f, "User not found in tenant"),
            TenantError::ProjectNotFound => write!(f, "Project not found"),
            TenantError::QuotaExceeded(resource) => write!(f, "Quota exceeded: {}", resource),
            TenantError::PermissionDenied => write!(f, "Permission denied"),
            TenantError::InvalidOperation => write!(f, "Invalid operation"),
        }
    }
}

impl std::error::Error for TenantError {}

// Re-export types
pub use Tenant;
pub use TenantUser;
pub use TenantProject;
pub use TenantPlan;
pub use TenantManager;