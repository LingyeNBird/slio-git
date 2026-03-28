//! Tag operations for git-core

use crate::error::GitError;
use crate::repository::Repository;
use log::info;
use std::process::Command;

/// A Git tag
#[derive(Debug, Clone)]
pub struct TagInfo {
    pub name: String,
    pub target: String,
    pub message: Option<String>,
    pub tagger_name: Option<String>,
    pub tagger_email: Option<String>,
    pub tagged_time: Option<i64>,
}

impl Default for TagInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            target: String::new(),
            message: None,
            tagger_name: None,
            tagger_email: None,
            tagged_time: None,
        }
    }
}

/// List all tags with full metadata.
pub fn list_tags(repo: &Repository) -> Result<Vec<TagInfo>, GitError> {
    info!("Listing all tags");

    let repo_path = repo.command_cwd();

    // Use for-each-ref with explicit format to get reliable tab-separated output.
    let format_arg = concat!(
        "%(refname:short)%01",  // name (unit-separated)
        "%(objectname:short)%01", // target commit
        "%(if)%(contents:body)%(then)%(contents:body)%(end)%01", // message body
        "%(taggername)%01",     // tagger name
        "%(taggeremail)%01",    // tagger email
        "%(taggerdate:unix)",   // timestamp
    );

    let output = Command::new("git")
        .args(["for-each-ref", "--sort=-taggerdate", "--format", format_arg, "refs/tags/"])
        .current_dir(&repo_path)
        .output()
        .map_err(|e| GitError::OperationFailed {
            operation: "list_tags".to_string(),
            details: format!("Failed to execute git for-each-ref: {}", e),
        })?;

    if !output.status.success() {
        return Err(GitError::OperationFailed {
            operation: "list_tags".to_string(),
            details: format!(
                "git for-each-ref failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut tags = Vec::new();

    for line in output_str.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split('\x01').collect();
        let name = fields.first().copied().unwrap_or("").trim().to_string();

        if name.is_empty() {
            continue;
        }

        let target = fields.get(1).map(|s| s.trim().to_string()).unwrap_or_default();
        let message = fields.get(2).and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
        });
        let tagger_name = fields.get(3).and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
        });
        let tagger_email = fields.get(4).and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
        });
        let tagged_time = fields.get(5).and_then(|s| s.trim().parse::<i64>().ok());

        tags.push(TagInfo {
            name,
            target,
            message,
            tagger_name,
            tagger_email,
            tagged_time,
        });
    }

    Ok(tags)
}

/// Create an annotated tag
pub fn create_tag(
    repo: &Repository,
    name: &str,
    target: &str,
    message: &str,
    tagger_name: &str,
    tagger_email: &str,
) -> Result<String, GitError> {
    info!("Creating annotated tag '{}' at {}", name, target);

    let repo_path = repo.command_cwd();

    // Set git environment for tagger
    let output = Command::new("git")
        .args(["tag", "-a", name, "-m", message, target])
        .env("GIT_COMMITTER_NAME", tagger_name)
        .env("GIT_COMMITTER_EMAIL", tagger_email)
        .current_dir(&repo_path)
        .output()
        .map_err(|e| GitError::OperationFailed {
            operation: "create_tag".to_string(),
            details: format!("Failed to execute git tag: {}", e),
        })?;

    if !output.status.success() {
        return Err(GitError::OperationFailed {
            operation: "create_tag".to_string(),
            details: format!(
                "git tag failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    info!("Tag '{}' created successfully", name);
    Ok(name.to_string())
}

/// Create a lightweight tag
pub fn create_lightweight_tag(
    repo: &Repository,
    name: &str,
    target: &str,
) -> Result<String, GitError> {
    info!("Creating lightweight tag '{}' at {}", name, target);

    let repo_path = repo.command_cwd();

    let output = Command::new("git")
        .args(["tag", name, target])
        .current_dir(&repo_path)
        .output()
        .map_err(|e| GitError::OperationFailed {
            operation: "create_lightweight_tag".to_string(),
            details: format!("Failed to execute git tag: {}", e),
        })?;

    if !output.status.success() {
        return Err(GitError::OperationFailed {
            operation: "create_lightweight_tag".to_string(),
            details: format!(
                "git tag failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    info!("Tag '{}' created successfully", name);
    Ok(name.to_string())
}

/// Delete a tag
pub fn delete_tag(repo: &Repository, name: &str) -> Result<(), GitError> {
    info!("Deleting tag '{}'", name);

    let repo_path = repo.command_cwd();

    let output = Command::new("git")
        .args(["tag", "-d", name])
        .current_dir(&repo_path)
        .output()
        .map_err(|e| GitError::OperationFailed {
            operation: "delete_tag".to_string(),
            details: format!("Failed to execute git tag: {}", e),
        })?;

    if !output.status.success() {
        return Err(GitError::OperationFailed {
            operation: "delete_tag".to_string(),
            details: format!(
                "git tag -d failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    info!("Tag '{}' deleted successfully", name);
    Ok(())
}
