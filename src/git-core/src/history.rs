//! History operations for git-core

use crate::error::GitError;
use crate::repository::Repository;
use git2::Sort;
use log::info;

/// Commit history entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub id: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: i64,
    pub parent_ids: Vec<String>,
}

/// Get commit history
pub fn get_history(
    repo: &Repository,
    max_count: Option<usize>,
) -> Result<Vec<HistoryEntry>, GitError> {
    info!("Getting commit history, max: {:?}", max_count);

    let repo_lock = repo.inner.read().unwrap();

    let mut revwalk = repo_lock.revwalk().map_err(|e| GitError::OperationFailed {
        operation: "get_history".to_string(),
        details: e.to_string(),
    })?;

    revwalk.push_head().map_err(|e| GitError::OperationFailed {
        operation: "get_history".to_string(),
        details: e.to_string(),
    })?;

    let _ = revwalk.set_sorting(Sort::TIME | Sort::TOPOLOGICAL);

    let mut history = Vec::new();
    let limit = max_count.unwrap_or(100);

    for (i, oid_result) in revwalk.enumerate() {
        if i >= limit {
            break;
        }

        if let Ok(oid) = oid_result {
            if let Ok(commit) = repo_lock.find_commit(oid) {
                history.push(HistoryEntry {
                    id: oid.to_string(),
                    message: commit.message().unwrap_or("").to_string(),
                    author_name: commit.author().name().unwrap_or("").to_string(),
                    author_email: commit.author().email().unwrap_or("").to_string(),
                    timestamp: commit.time().seconds(),
                    parent_ids: commit.parents().map(|p| p.id().to_string()).collect(),
                });
            }
        }
    }

    info!("Retrieved {} history entries", history.len());
    Ok(history)
}

/// Get commit history starting from a specific ref, branch, tag, or commit id.
pub fn get_history_for_ref(
    repo: &Repository,
    reference: &str,
    max_count: Option<usize>,
) -> Result<Vec<HistoryEntry>, GitError> {
    info!(
        "Getting commit history for reference '{}', max: {:?}",
        reference, max_count
    );

    let repo_lock = repo.inner.read().unwrap();

    let object = repo_lock
        .revparse_single(reference)
        .map_err(|e| GitError::OperationFailed {
            operation: "get_history_for_ref".to_string(),
            details: e.to_string(),
        })?;
    let commit = object
        .peel_to_commit()
        .map_err(|e| GitError::OperationFailed {
            operation: "get_history_for_ref".to_string(),
            details: e.to_string(),
        })?;

    let mut revwalk = repo_lock.revwalk().map_err(|e| GitError::OperationFailed {
        operation: "get_history_for_ref".to_string(),
        details: e.to_string(),
    })?;

    revwalk
        .push(commit.id())
        .map_err(|e| GitError::OperationFailed {
            operation: "get_history_for_ref".to_string(),
            details: e.to_string(),
        })?;

    let _ = revwalk.set_sorting(Sort::TIME | Sort::TOPOLOGICAL);

    let mut history = Vec::new();
    let limit = max_count.unwrap_or(100);

    for (i, oid_result) in revwalk.enumerate() {
        if i >= limit {
            break;
        }

        if let Ok(oid) = oid_result {
            if let Ok(commit) = repo_lock.find_commit(oid) {
                history.push(HistoryEntry {
                    id: oid.to_string(),
                    message: commit.message().unwrap_or("").to_string(),
                    author_name: commit.author().name().unwrap_or("").to_string(),
                    author_email: commit.author().email().unwrap_or("").to_string(),
                    timestamp: commit.time().seconds(),
                    parent_ids: commit.parents().map(|p| p.id().to_string()).collect(),
                });
            }
        }
    }

    info!(
        "Retrieved {} history entries for reference '{}'",
        history.len(),
        reference
    );
    Ok(history)
}

/// Search commits by message
pub fn search_history(
    repo: &Repository,
    pattern: &str,
    max_count: Option<usize>,
) -> Result<Vec<HistoryEntry>, GitError> {
    info!("Searching history for '{}'", pattern);

    let repo_lock = repo.inner.read().unwrap();

    let mut revwalk = repo_lock.revwalk().map_err(|e| GitError::OperationFailed {
        operation: "search_history".to_string(),
        details: e.to_string(),
    })?;

    revwalk.push_head().map_err(|e| GitError::OperationFailed {
        operation: "search_history".to_string(),
        details: e.to_string(),
    })?;

    let _ = revwalk.set_sorting(Sort::TIME | Sort::TOPOLOGICAL);

    let mut history = Vec::new();
    let limit = max_count.unwrap_or(100);
    let pattern_lower = pattern.to_lowercase();

    for (i, oid_result) in revwalk.enumerate() {
        if i >= limit {
            break;
        }

        if let Ok(oid) = oid_result {
            if let Ok(commit) = repo_lock.find_commit(oid) {
                let message = commit.message().unwrap_or("").to_lowercase();
                if message.contains(&pattern_lower) {
                    history.push(HistoryEntry {
                        id: oid.to_string(),
                        message: commit.message().unwrap_or("").to_string(),
                        author_name: commit.author().name().unwrap_or("").to_string(),
                        author_email: commit.author().email().unwrap_or("").to_string(),
                        timestamp: commit.time().seconds(),
                        parent_ids: commit.parents().map(|p| p.id().to_string()).collect(),
                    });
                }
            }
        }
    }

    info!("Found {} matching history entries", history.len());
    Ok(history)
}
