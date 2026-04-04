//! Integration tests for new git-core modules added in 011-idea-git-parity.
//!
//! Tests: blame, graph, worktree, submodule detection, commit message history.
//! Signature verification is tested only for extraction (not gpg/ssh availability).

mod test_helpers;

use git_core::Repository;
use test_helpers::TestRepo;

// ── T099a: Blame ──────────────────────────────────────────────────────────────

#[test]
fn blame_file_returns_correct_attribution_for_single_author() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("hello.txt", "line one\nline two\nline three\n", "initial")
        .unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let entries =
        git_core::blame_file(&r, std::path::Path::new("hello.txt")).unwrap();

    assert!(!entries.is_empty(), "blame should return at least one entry");
    assert_eq!(entries[0].author_name, "Codex Test");
    // All lines should be from the same commit
    let total_lines: u32 = entries.iter().map(|e| e.line_count).sum();
    assert_eq!(total_lines, 3);
}

#[test]
fn blame_file_tracks_line_changes_across_commits() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("file.txt", "aaa\nbbb\nccc\n", "first commit")
        .unwrap();
    repo.add_and_commit("file.txt", "aaa\nBBB\nccc\n", "second commit")
        .unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let entries =
        git_core::blame_file(&r, std::path::Path::new("file.txt")).unwrap();

    // There should be at least 2 hunks (some from first commit, some from second)
    assert!(
        entries.len() >= 2,
        "blame should have multiple hunks after edit, got {}",
        entries.len()
    );

    let messages: Vec<&str> = entries.iter().map(|e| e.message.as_str()).collect();
    assert!(
        messages.iter().any(|m| m.contains("first")),
        "should reference first commit"
    );
    assert!(
        messages.iter().any(|m| m.contains("second")),
        "should reference second commit"
    );
}

// ── T099e: Graph ──────────────────────────────────────────────────────────────

#[test]
fn compute_graph_assigns_lanes_for_linear_history() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "a", "commit 1").unwrap();
    repo.add_and_commit("a.txt", "b", "commit 2").unwrap();
    repo.add_and_commit("a.txt", "c", "commit 3").unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let history = git_core::get_history(&r, Some(10)).unwrap();
    let ids: Vec<String> = history.iter().map(|e| e.id.clone()).collect();

    let nodes = git_core::compute_graph(&r, &ids).unwrap();
    assert_eq!(nodes.len(), 3);
    // Linear history should stay on lane 0
    assert!(
        nodes.iter().all(|n| n.lane == 0),
        "linear commits should all be on lane 0"
    );
}

#[test]
fn compute_ref_labels_finds_branches_and_tags() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "a", "commit 1").unwrap();

    // Create a tag
    std::process::Command::new("git")
        .args(["tag", "v1.0"])
        .current_dir(repo.path())
        .output()
        .unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let labels = git_core::compute_ref_labels(&r).unwrap();

    assert!(!labels.is_empty(), "should find at least one ref label");

    // Should find the main branch and tag
    let all_names: Vec<String> = labels
        .values()
        .flatten()
        .map(|l| l.name.clone())
        .collect();
    assert!(
        all_names.iter().any(|n| n == "v1.0"),
        "should find tag v1.0, got {:?}",
        all_names
    );
}

// ── T099c: Worktree ──────────────────────────────────────────────────────────

#[test]
fn worktree_list_shows_main_worktree() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "a", "init").unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let worktrees = git_core::list_worktrees(&r).unwrap();

    assert!(
        !worktrees.is_empty(),
        "should list at least the main worktree"
    );
    assert!(worktrees[0].is_main, "first worktree should be main");
}

#[test]
fn worktree_create_and_remove_lifecycle() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "a", "init").unwrap();

    let r = Repository::discover(repo.path()).unwrap();

    // Create a branch first
    std::process::Command::new("git")
        .args(["branch", "wt-branch"])
        .current_dir(repo.path())
        .output()
        .unwrap();

    let wt_path = repo.path().join("my-worktree");
    git_core::create_worktree(&r, &wt_path, Some("wt-branch")).unwrap();

    let worktrees = git_core::list_worktrees(&r).unwrap();
    assert!(
        worktrees.len() >= 2,
        "should have main + new worktree, got {}",
        worktrees.len()
    );

    git_core::remove_worktree(&r, &wt_path).unwrap();
    let worktrees_after = git_core::list_worktrees(&r).unwrap();
    assert!(
        worktrees_after.len() < worktrees.len(),
        "worktree count should decrease after removal"
    );
}

// ── T099d: Submodule ──────────────────────────────────────────────────────────

#[test]
fn is_submodule_returns_false_for_regular_files() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("regular.txt", "content", "add file")
        .unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    assert!(!git_core::is_submodule(&r, "regular.txt"));
}

#[test]
fn list_submodules_returns_empty_for_repo_without_submodules() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "a", "init").unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let submodules = git_core::list_submodules(&r).unwrap();
    assert!(submodules.is_empty());
}

// ── T099b: Signature (extraction only) ────────────────────────────────────────

#[test]
fn verify_commit_signature_returns_unsigned_for_normal_commit() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "a", "unsigned commit")
        .unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let history = git_core::get_history(&r, Some(1)).unwrap();
    let commit_id = &history[0].id;

    let status = git_core::verify_commit_signature(&r, commit_id).unwrap();
    assert!(!status.is_signed, "normal commit should not be signed");
    assert!(!status.is_verified);
}

// ── Commit message history ────────────────────────────────────────────────────

#[test]
fn commit_message_history_save_and_load_roundtrip() {
    let temp = tempfile::tempdir().unwrap();
    let repo_path = temp.path();

    // Save messages
    git_core::save_recent_message(repo_path, "fix: bug #123");
    git_core::save_recent_message(repo_path, "feat: new feature");

    // Load messages
    let messages = git_core::load_recent_messages(repo_path);
    assert!(messages.len() >= 2);
    assert_eq!(messages[0], "feat: new feature"); // newest first
    assert_eq!(messages[1], "fix: bug #123");
}

// ── Branch merge check ────────────────────────────────────────────────────────

#[test]
fn is_branch_merged_detects_merged_branch() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "a", "init").unwrap();

    // Create and merge a branch
    std::process::Command::new("git")
        .args(["checkout", "-b", "feature"])
        .current_dir(repo.path())
        .output()
        .unwrap();
    repo.add_and_commit("b.txt", "b", "feature commit")
        .unwrap();
    std::process::Command::new("git")
        .args(["checkout", "main"])
        .current_dir(repo.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["merge", "feature", "--no-ff", "-m", "merge feature"])
        .current_dir(repo.path())
        .output()
        .unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let merged = r.is_branch_merged("feature").unwrap();
    assert!(merged, "feature branch should be merged into main");
}

// ── T007: Uncommit ────────────────────────────────────────────────────────────

#[test]
fn uncommit_to_commit_soft_resets_and_preserves_staging() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "first", "commit 1").unwrap();
    repo.add_and_commit("b.txt", "second", "commit 2").unwrap();
    repo.add_and_commit("c.txt", "third", "commit 3").unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let history = git_core::get_history(&r, Some(10)).unwrap();
    assert_eq!(history.len(), 3);

    // Uncommit to commit 3 (the latest/HEAD) — removes only commit 3
    // uncommit_to_commit resets to target^, so passing commit 3 resets to commit 2
    let target_id = &history[0].id; // commit 3 (HEAD, newest)
    git_core::uncommit_to_commit(&r, target_id).unwrap();

    // Refresh and verify commits 1 and 2 remain
    let r2 = Repository::discover(repo.path()).unwrap();
    let history_after = git_core::get_history(&r2, Some(10)).unwrap();
    assert_eq!(history_after.len(), 2, "commits 1 and 2 should remain after uncommitting commit 3");

    // Changes from commit 3 (c.txt) should be in staging area
    let status = git_core::index::get_status(&r2).unwrap();
    let staged: Vec<_> = status.iter().filter(|c| c.staged).collect();
    assert!(
        !staged.is_empty(),
        "c.txt should be staged after uncommit, got {} staged files",
        staged.len()
    );
}

// ── T008: Unstash as branch ──────────────────────────────────────────────────

#[test]
fn unstash_as_branch_creates_branch_and_applies_changes() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "base", "initial").unwrap();
    repo.write_file("b.txt", "stashed content").unwrap();

    // Stage and stash
    std::process::Command::new("git")
        .args(["add", "b.txt"])
        .current_dir(repo.path())
        .output()
        .unwrap();
    git_core::stash_save(&Repository::discover(repo.path()).unwrap(), Some("test stash")).unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    let stashes = git_core::list_stashes(&r).unwrap();
    assert!(!stashes.is_empty());

    // Unstash as new branch
    git_core::unstash_as_branch(&r, 0, "stash-branch").unwrap();

    // Verify new branch exists and is checked out
    let r2 = Repository::discover(repo.path()).unwrap();
    let current = r2.current_branch().unwrap();
    assert_eq!(current.as_deref(), Some("stash-branch"));

    // Verify file exists
    assert!(repo.path().join("b.txt").exists());
}

// ── T009: Keep index stash ────────────────────────────────────────────────────

#[test]
fn stash_with_keep_index_preserves_staged_files() {
    let repo = TestRepo::new().unwrap();
    repo.add_and_commit("a.txt", "base", "initial").unwrap();

    // Create two changes: one staged, one unstaged
    repo.write_file("staged.txt", "staged content").unwrap();
    std::process::Command::new("git")
        .args(["add", "staged.txt"])
        .current_dir(repo.path())
        .output()
        .unwrap();
    repo.write_file("unstaged.txt", "unstaged content").unwrap();

    let r = Repository::discover(repo.path()).unwrap();
    git_core::stash_save_with_options(&r, Some("keep index test"), false, true).unwrap();

    // With keep-index, staged.txt should still be in the working tree
    assert!(
        repo.path().join("staged.txt").exists(),
        "staged file should be preserved with --keep-index"
    );
}
