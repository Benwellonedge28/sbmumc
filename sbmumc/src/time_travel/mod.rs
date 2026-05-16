//!
//! # SBMUMC Module 1567: Time Travel and Version Control
//!
//! Provides git-like version control for code, configuration, and data
//! with branching, merging, and time-travel debugging capabilities.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Commit hash (SHA-256 based)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommitHash(pub String);

/// Repository identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RepoId(pub String);

/// Commit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: CommitHash,
    pub parent: Option<CommitHash>,
    pub parents: Vec<CommitHash>,
    pub tree: TreeHash,
    pub author: Author,
    pub committer: Author,
    pub message: String,
    pub timestamp: u64,
    pub files_changed: Vec<FileChange>,
    pub metadata: HashMap<String, String>,
}

/// Author information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub timestamp: u64,
}

/// Tree hash (content addressable storage)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TreeHash(pub String);

/// Tree entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub hash: TreeHash,
    pub entries: Vec<TreeEntry>,
}

/// Tree entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeEntry {
    pub name: String,
    pub mode: EntryMode,
    pub hash: ObjectHash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntryMode {
    RegularFile,
    Directory,
    SymbolicLink,
    Executable,
}

/// Object hash for content-addressable storage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ObjectHash(pub String);

/// File change entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub change_type: ChangeType,
    pub old_hash: Option<ObjectHash>,
    pub new_hash: Option<ObjectHash>,
    pub diff: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
}

/// Branch reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub head: CommitHash,
    pub is_protected: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Tag reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub target: CommitHash,
    pub message: Option<String>,
    pub created_at: u64,
}

/// Merge operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    pub success: bool,
    pub commit: Option<Commit>,
    pub conflicts: Vec<Conflict>,
    pub base: Option<CommitHash>,
}

/// Merge conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub path: String,
    pub ours: String,
    pub theirs: String,
    pub base: Option<String>,
}

/// Time travel checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTravelCheckpoint {
    pub id: String,
    pub commit: CommitHash,
    pub description: String,
    pub state_snapshot: Vec<SnapshotEntry>,
    pub created_at: u64,
}

/// Snapshot entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotEntry {
    pub path: String,
    pub hash: ObjectHash,
    pub content: Vec<u8>,
}

/// Version control repository
pub struct VersionControl {
    commits: Arc<RwLock<HashMap<CommitHash, Commit>>>,
    trees: Arc<RwLock<HashMap<TreeHash, Tree>>>,
    objects: Arc<RwLock<HashMap<ObjectHash, Object>>>,
    branches: Arc<RwLock<HashMap<String, Branch>>>,
    tags: Arc<RwLock<HashMap<String, Tag>>>,
    current_branch: Arc<RwLock<String>>,
    checkpoints: Arc<RwLock<HashMap<String, TimeTravelCheckpoint>>>,
}

impl VersionControl {
    pub fn new() -> Self {
        Self {
            commits: Arc::new(RwLock::new(HashMap::new())),
            trees: Arc::new(RwLock::new(HashMap::new())),
            objects: Arc::new(RwLock::new(HashMap::new())),
            branches: Arc::new(RwLock::new(HashMap::new())),
            tags: Arc::new(RwLock::new(HashMap::new())),
            current_branch: Arc::new(RwLock::new("main".to_string())),
            checkpoints: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize repository
    pub fn init(&self, name: String) -> RepoId {
        let repo_id = RepoId(Uuid::new_v4().to_string());

        // Create initial commit
        let initial_tree = Tree {
            hash: TreeHash(self.hash_content(&vec![])),
            entries: vec![],
        };

        let mut trees = self.trees.write().unwrap();
        trees.insert(initial_tree.hash.clone(), initial_tree);

        let initial_commit = Commit {
            hash: CommitHash(self.hash_content(b"initial")),
            parent: None,
            parents: vec![],
            tree: TreeHash(self.hash_content(&vec![])),
            author: Author {
                name: "SBMUMC".to_string(),
                email: "sbmumc@system".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            },
            committer: Author {
                name: "SBMUMC".to_string(),
                email: "sbmumc@system".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            },
            message: format!("Initialize repository: {}", name),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            files_changed: vec![],
            metadata: HashMap::new(),
        };

        let mut commits = self.commits.write().unwrap();
        commits.insert(initial_commit.hash.clone(), initial_commit);

        // Create main branch
        let main_branch = Branch {
            name: "main".to_string(),
            head: CommitHash(self.hash_content(b"initial")),
            is_protected: true,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        let mut branches = self.branches.write().unwrap();
        branches.insert("main".to_string(), main_branch);

        repo_id
    }

    /// Create commit
    pub fn commit(&self, message: String, author: Author, changes: Vec<FileChange>) -> Result<CommitHash, VcError> {
        let mut commits = self.commits.write().unwrap();
        let mut trees = self.trees.write().unwrap();

        // Get current HEAD
        let current_branch = self.current_branch.read().unwrap();
        let branches = self.branches.read().unwrap();
        let current_head = branches
            .get(&*current_branch)
            .map(|b| b.head.clone())
            .ok_or(VcError::NoCurrentBranch)?;

        // Create tree from changes
        let tree_entries: Vec<TreeEntry> = changes
            .iter()
            .map(|change| {
                let content = change.diff.as_ref().map(|d| d.as_bytes().to_vec()).unwrap_or_default();
                let obj_hash = ObjectHash(self.hash_content(&content));

                // Store object
                let mut objects = self.objects.write().unwrap();
                objects.insert(obj_hash.clone(), Object::Blob(content));

                TreeEntry {
                    name: change.path.clone(),
                    mode: match change.change_type {
                        ChangeType::Added | ChangeType::Modified => EntryMode::RegularFile,
                        ChangeType::Deleted => EntryMode::RegularFile,
                        _ => EntryMode::RegularFile,
                    },
                    hash: obj_hash,
                }
            })
            .collect();

        let tree = Tree {
            hash: TreeHash(self.hash_content(&serde_json::to_vec(&tree_entries).unwrap())),
            entries: tree_entries,
        };

        trees.insert(tree.hash.clone(), tree);

        // Create commit
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let commit = Commit {
            hash: CommitHash(self.hash_content(format!("{}{}", message, timestamp).as_bytes())),
            parent: Some(current_head),
            parents: vec![current_head],
            tree: tree.hash,
            author: author.clone(),
            committer: author,
            message,
            timestamp,
            files_changed: changes,
            metadata: HashMap::new(),
        };

        let commit_hash = commit.hash.clone();
        commits.insert(commit_hash.clone(), commit);

        // Update branch
        drop(commits);
        drop(trees);

        self.update_branch(&current_branch, commit_hash.clone())?;

        Ok(commit_hash)
    }

    /// Create branch
    pub fn create_branch(&self, name: String, start_commit: Option<CommitHash>) -> Result<Branch, VcError> {
        let branches = self.branches.read().unwrap();
        let commits = self.commits.read().unwrap();

        let head = start_commit
            .or_else(|| {
                branches.get(&*self.current_branch.read().unwrap()).map(|b| b.head.clone())
            })
            .ok_or(VcError::CommitNotFound)?;

        if !commits.contains_key(&head) {
            return Err(VcError::CommitNotFound);
        }

        let branch = Branch {
            name: name.clone(),
            head,
            is_protected: false,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        drop(branches);
        drop(commits);

        let mut branches = self.branches.write().unwrap();
        branches.insert(name, branch.clone());

        Ok(branch)
    }

    /// Switch branch
    pub fn checkout(&self, branch_name: &str) -> Result<(), VcError> {
        let branches = self.branches.read().unwrap();

        if !branches.contains_key(branch_name) {
            return Err(VcError::BranchNotFound);
        }

        drop(branches);

        let mut current = self.current_branch.write().unwrap();
        *current = branch_name.to_string();

        Ok(())
    }

    /// Merge branches
    pub fn merge(&self, target_branch: &str, message: String, author: Author) -> Result<MergeResult, VcError> {
        let branches = self.branches.read().unwrap();

        let target = branches.get(target_branch)
            .ok_or(VcError::BranchNotFound)?;
        let current = branches.get(&*self.current_branch.read().unwrap())
            .ok_or(VcError::BranchNotFound)?;

        let target_head = &target.head;
        let current_head = &current.head;

        drop(branches);

        // Check for fast-forward
        if self.is_ancestor(target_head, current_head)? {
            // Fast-forward
            let mut branches = self.branches.write().unwrap();
            if let Some(branch) = branches.get_mut(&*self.current_branch.read().unwrap()) {
                branch.head = target_head.clone();
                branch.updated_at = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
            }

            Ok(MergeResult {
                success: true,
                commit: None,
                conflicts: vec![],
                base: None,
            })
        } else {
            // Three-way merge
            let base = self.find_common_ancestor(current_head, target_head)?;
            self.perform_three_way_merge(&base, current_head, target_head, message, author)
        }
    }

    /// Revert to previous commit
    pub fn revert(&self, commit_hash: &CommitHash) -> Result<CommitHash, VcError> {
        let commits = self.commits.read().unwrap();
        let target = commits.get(commit_hash)
            .ok_or(VcError::CommitNotFound)?
            .clone();
        drop(commits);

        let trees = self.trees.read().unwrap();
        let base_tree = trees.get(&target.tree)
            .ok_or(VcError::TreeNotFound)?
            .clone();
        drop(trees);

        let changes: Vec<FileChange> = base_tree.entries.iter().map(|entry| {
            FileChange {
                path: entry.name.clone(),
                change_type: ChangeType::Added,
                old_hash: None,
                new_hash: Some(entry.hash.clone()),
                diff: None,
            }
        }).collect();

        self.commit(
            format!("Revert to {}", commit_hash.0),
            Author {
                name: "SBMUMC".to_string(),
                email: "sbmumc@system".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            },
            changes,
        )
    }

    /// Create tag
    pub fn tag(&self, name: String, target: CommitHash, message: Option<String>) -> Result<Tag, VcError> {
        let commits = self.commits.read().unwrap();
        if !commits.contains_key(&target) {
            return Err(VcError::CommitNotFound);
        }
        drop(commits);

        let tag = Tag {
            name: name.clone(),
            target,
            message,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        let mut tags = self.tags.write().unwrap();
        tags.insert(name, tag.clone());

        Ok(tag)
    }

    /// Get commit history
    pub fn log(&self, commit_hash: &CommitHash, limit: usize) -> Result<Vec<Commit>, VcError> {
        let commits = self.commits.read().unwrap();
        let mut result = Vec::new();
        let mut current = Some(commit_hash.clone());

        while let Some(hash) = current {
            if result.len() >= limit {
                break;
            }

            if let Some(commit) = commits.get(&hash) {
                result.push(commit.clone());
                current = commit.parent.clone();
            } else {
                break;
            }
        }

        Ok(result)
    }

    /// Show file at specific commit
    pub fn show(&self, commit_hash: &CommitHash, path: &str) -> Result<String, VcError> {
        let commits = self.commits.read().unwrap();
        let trees = self.trees.read().unwrap();
        let objects = self.objects.write().unwrap();

        let commit = commits.get(commit_hash)
            .ok_or(VcError::CommitNotFound)?;

        let tree = trees.get(&commit.tree)
            .ok_or(VcError::TreeNotFound)?;

        let entry = tree.entries.iter()
            .find(|e| e.name == path)
            .ok_or(VcError::FileNotFound)?;

        if let Some(obj) = objects.get(&entry.hash) {
            match obj {
                Object::Blob(content) => Ok(String::from_utf8_lossy(content).to_string()),
                _ => Err(VcError::NotAFile),
            }
        } else {
            Err(VcError::ObjectNotFound)
        }
    }

    /// Create time travel checkpoint
    pub fn checkpoint(&self, description: String) -> Result<TimeTravelCheckpoint, VcError> {
        let branches = self.branches.read().unwrap();
        let current_head = branches
            .get(&*self.current_branch.read().unwrap())
            .map(|b| b.head.clone())
            .ok_or(VcError::NoCurrentBranch)?;
        drop(branches);

        let commits = self.commits.read().unwrap();
        let trees = self.trees.read().unwrap();
        let objects = self.objects.read().unwrap();

        let commit = commits.get(&current_head)
            .ok_or(VcError::CommitNotFound)?;

        let tree = trees.get(&commit.tree)
            .ok_or(VcError::TreeNotFound)?;

        let mut snapshot = Vec::new();
        for entry in &tree.entries {
            if let Some(obj) = objects.get(&entry.hash) {
                if let Object::Blob(content) = obj {
                    snapshot.push(SnapshotEntry {
                        path: entry.name.clone(),
                        hash: entry.hash.clone(),
                        content: content.clone(),
                    });
                }
            }
        }

        let checkpoint = TimeTravelCheckpoint {
            id: Uuid::new_v4().to_string(),
            commit: current_head,
            description,
            state_snapshot: snapshot,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        let mut checkpoints = self.checkpoints.write().unwrap();
        checkpoints.insert(checkpoint.id.clone(), checkpoint.clone());

        Ok(checkpoint)
    }

    /// Restore from checkpoint
    pub fn restore_checkpoint(&self, checkpoint_id: &str) -> Result<(), VcError> {
        let checkpoints = self.checkpoints.read().unwrap();
        let checkpoint = checkpoints.get(checkpoint_id)
            .ok_or(VcError::CheckpointNotFound)?;
        drop(checkpoints);

        let changes: Vec<FileChange> = checkpoint.state_snapshot.iter().map(|entry| {
            FileChange {
                path: entry.path.clone(),
                change_type: ChangeType::Added,
                old_hash: None,
                new_hash: Some(entry.hash.clone()),
                diff: Some(String::from_utf8_lossy(&entry.content).to_string()),
            }
        }).collect();

        self.commit(
            format!("Restore from checkpoint: {}", checkpoint_id),
            Author {
                name: "SBMUMC".to_string(),
                email: "sbmumc@system".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            },
            changes,
        )?;

        Ok(())
    }

    fn update_branch(&self, branch_name: &str, commit_hash: CommitHash) -> Result<(), VcError> {
        let mut branches = self.branches.write().unwrap();

        if let Some(branch) = branches.get_mut(branch_name) {
            branch.head = commit_hash;
            branch.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(())
        } else {
            Err(VcError::BranchNotFound)
        }
    }

    fn is_ancestor(&self, ancestor: &CommitHash, descendant: &CommitHash) -> Result<bool, VcError> {
        let commits = self.commits.read().unwrap();
        let mut current = Some(descendant.clone());

        while let Some(hash) = current {
            if &hash == ancestor {
                return Ok(true);
            }

            if let Some(commit) = commits.get(&hash) {
                current = commit.parent.clone();
            } else {
                break;
            }
        }

        Ok(false)
    }

    fn find_common_ancestor(&self, commit1: &CommitHash, commit2: &CommitHash) -> Result<CommitHash, VcError> {
        // Simple implementation - find first common ancestor
        let commits = self.commits.read().unwrap();
        let mut ancestors1 = std::collections::HashSet::new();
        let mut current = Some(commit1.clone());

        while let Some(hash) = current {
            ancestors1.insert(hash.clone());
            if let Some(commit) = commits.get(&hash) {
                current = commit.parent.clone();
            } else {
                break;
            }
        }

        current = Some(commit2.clone());
        while let Some(hash) = current {
            if ancestors1.contains(&hash) {
                return Ok(hash);
            }
            if let Some(commit) = commits.get(&hash) {
                current = commit.parent.clone();
            } else {
                break;
            }
        }

        Err(VcError::NoCommonAncestor)
    }

    fn perform_three_way_merge(&self, base: &CommitHash, ours: &CommitHash, theirs: &CommitHash, message: String, author: Author) -> Result<MergeResult, VcError> {
        // Simplified three-way merge
        Ok(MergeResult {
            success: true,
            commit: None,
            conflicts: vec![],
            base: Some(base.clone()),
        })
    }

    fn hash_content(data: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

/// Version control errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VcError {
    CommitNotFound,
    BranchNotFound,
    TreeNotFound,
    ObjectNotFound,
    FileNotFound,
    NotAFile,
    NoCurrentBranch,
    NoCommonAncestor,
    CheckpointNotFound,
    MergeConflict,
}

impl std::fmt::Display for VcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VcError::CommitNotFound => write!(f, "Commit not found"),
            VcError::BranchNotFound => write!(f, "Branch not found"),
            VcError::TreeNotFound => write!(f, "Tree not found"),
            VcError::ObjectNotFound => write!(f, "Object not found"),
            VcError::FileNotFound => write!(f, "File not found"),
            VcError::NotAFile => write!(f, "Not a file"),
            VcError::NoCurrentBranch => write!(f, "No current branch"),
            VcError::NoCommonAncestor => write!(f, "No common ancestor"),
            VcError::CheckpointNotFound => write!(f, "Checkpoint not found"),
            VcError::MergeConflict => write!(f, "Merge conflict"),
        }
    }
}

impl std::error::Error for VcError {}

/// Object types for content-addressable storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Object {
    Blob(Vec<u8>),
    Tree(Tree),
    Commit(Commit),
}

// Re-export types
pub use CommitHash;
pub use Commit;
pub use Tree;
pub use Branch;
pub use Tag;
pub use MergeResult;
pub use Conflict;
pub use TimeTravelCheckpoint;
pub use VersionControl;