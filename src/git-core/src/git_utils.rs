//! Shared internal helpers used by both commit_actions and rebase modules.
//!
//! These functions are not part of the public API.

use crate::error::GitError;
use crate::index;
use crate::repository::Repository;
use git2::Oid;
use std::fs;
use std::path::Path;

/// Resolve the current HEAD commit OID.
pub(crate) fn current_head_oid(repo: &Repository, operation: &str) -> Result<Oid, GitError> {
    let repo_lock = repo.inner.read().unwrap();
    let head = repo_lock.head().map_err(|e| GitError::OperationFailed {
        operation: operation.to_string(),
        details: e.to_string(),
    })?;
    let commit = head
        .peel_to_commit()
        .map_err(|e| GitError::OperationFailed {
            operation: operation.to_string(),
            details: e.to_string(),
        })?;
    Ok(commit.id())
}

/// Resolve a revision spec (e.g. "HEAD~3", a SHA, a ref) to a commit OID.
pub(crate) fn resolve_commit_oid(
    repo: &Repository,
    spec: &str,
    _operation: &str,
) -> Result<Oid, GitError> {
    let repo_lock = repo.inner.read().unwrap();
    let object = repo_lock
        .revparse_single(spec)
        .map_err(|_| GitError::CommitNotFound {
            id: spec.to_string(),
        })?;
    let commit = object
        .peel_to_commit()
        .map_err(|_| GitError::CommitNotFound {
            id: spec.to_string(),
        })?;
    Ok(commit.id())
}

/// Check whether `ancestor` is an ancestor of `descendant`.
pub(crate) fn is_ancestor(
    repo: &Repository,
    ancestor: Oid,
    descendant: Oid,
) -> Result<bool, GitError> {
    if ancestor == descendant {
        return Ok(true);
    }

    let repo_lock = repo.inner.read().unwrap();
    repo_lock
        .graph_descendant_of(descendant, ancestor)
        .map_err(|e| GitError::OperationFailed {
            operation: "graph_descendant_of".to_string(),
            details: e.to_string(),
        })
}

/// Ensure the working tree is clean (no in-progress operations, no dirty files).
pub(crate) fn ensure_clean_worktree(
    repo: &Repository,
    operation: &str,
) -> Result<(), GitError> {
    match repo.get_state() {
        crate::repository::RepositoryState::Clean
        | crate::repository::RepositoryState::Dirty => {}
        _ => {
            return Err(GitError::OperationFailed {
                operation: operation.to_string(),
                details: format!(
                    "当前仓库正处于 {}，请先完成或中止当前流程",
                    repo.state_hint()
                        .unwrap_or_else(|| "进行中的 Git 操作".to_string())
                ),
            });
        }
    }

    if index::get_status(repo)
        .map_err(|e| GitError::OperationFailed {
            operation: operation.to_string(),
            details: e.to_string(),
        })?
        .is_empty()
    {
        Ok(())
    } else {
        Err(GitError::OperationFailed {
            operation: operation.to_string(),
            details: "当前仓库还有未提交改动，请先提交、暂存或清理工作区".to_string(),
        })
    }
}

/// Walk the first-parent chain from HEAD back to the root, returning OIDs oldest-first.
pub(crate) fn current_branch_first_parent_chain(
    repo: &Repository,
    operation: &str,
) -> Result<Vec<Oid>, GitError> {
    let head_oid = current_head_oid(repo, operation)?;
    let repo_lock = repo.inner.read().unwrap();
    let mut commit = repo_lock
        .find_commit(head_oid)
        .map_err(|e| GitError::OperationFailed {
            operation: operation.to_string(),
            details: e.to_string(),
        })?;

    let mut chain = Vec::new();
    loop {
        chain.push(commit.id());
        if commit.parent_count() == 0 {
            break;
        }
        commit = commit.parent(0).map_err(|e| GitError::OperationFailed {
            operation: operation.to_string(),
            details: e.to_string(),
        })?;
    }

    chain.reverse();
    Ok(chain)
}

/// Get the one-line subject of a commit by OID.
pub(crate) fn commit_subject_for_oid(
    repo_lock: &git2::Repository,
    oid: Oid,
    operation: &str,
) -> Result<String, GitError> {
    let commit = repo_lock
        .find_commit(oid)
        .map_err(|e| GitError::OperationFailed {
            operation: operation.to_string(),
            details: e.to_string(),
        })?;
    Ok(commit
        .summary()
        .unwrap_or("(no subject)")
        .replace('\n', " ")
        .replace('\r', " "))
}

/// Write a GIT_SEQUENCE_EDITOR script that copies a prepared todo file to `$1`.
pub(crate) fn write_sequence_editor_script(
    todo_path: &Path,
    script_path: &Path,
) -> Result<(), GitError> {
    #[cfg(unix)]
    let contents = format!("#!/bin/sh\ncat '{}' > \"$1\"\n", todo_path.display());
    #[cfg(windows)]
    let contents = format!("@echo off\r\ntype \"{}\" > %1\r\n", todo_path.display());

    fs::write(script_path, contents).map_err(GitError::Io)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(script_path)
            .map_err(GitError::Io)?
            .permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(script_path, permissions).map_err(GitError::Io)?;
    }

    Ok(())
}
