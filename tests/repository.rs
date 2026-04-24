// Integration tests associated with repository.rs

mod common;
use common::make_repo;

use stratum::{Commit, Local, Repository};

/// Core test runner for the mock git repo, as there's only one commit which is
/// the same as HEAD this can be reused many times for this one repository.
fn run_test_for_repo_head(commit: &Commit<'_>) {
    assert_eq!(commit.msg(), Some(common::EXPECTED_MSG.to_string()));

    assert!(!commit.is_merge());
    assert!(commit.parents().collect::<Vec<String>>().is_empty());
    // Check author equivalence
    assert_eq!(
        commit.author().name(),
        Some(common::EXPECTED_ACTOR_NAME.to_string())
    );
    assert_eq!(
        commit.author().email(),
        Some(common::EXPECTED_ACTOR_EMAIL.to_string())
    );
    // Check committer equivalence
    assert_eq!(
        commit.committer().name(),
        Some(common::EXPECTED_ACTOR_NAME.to_string())
    );
    assert_eq!(
        commit.committer().email(),
        Some(common::EXPECTED_ACTOR_EMAIL.to_string())
    );
}

#[test]
fn init_repo_from_path() {
    // Same content as 'make_repo' function, this will flag if that causes
    // the other tests to fail
    let repo_path = make_repo();
    Repository::<Local>::new(repo_path.path().to_str().unwrap()).expect("Failed to open as repo");
}

#[test]
fn test_commit_traversal() {
    let repo_path = make_repo();
    let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
        .expect("Failed to open as repo");

    // Expecting an iter of length one
    let iter = repo.traverse_commits().expect("Iterator Error");
    for commit in iter {
        let commit = commit.expect("Expected a valid stratum commit");
        run_test_for_repo_head(&commit);
    }
}

#[test]
fn test_head() {
    let repo_path = make_repo();
    let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
        .expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    run_test_for_repo_head(&head);
}

#[test]
fn test_single() {
    let repo_path = make_repo();
    let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
        .expect("Failed to open as repo");

    let head_hash = repo
        .raw()
        .head()
        .unwrap()
        .peel_to_commit()
        .unwrap()
        .id()
        .to_string();
    let commit = repo.single(&head_hash).expect("Expected valid hash string");

    run_test_for_repo_head(&commit);
}

#[test]
fn test_insertions() {
    let repo_path = make_repo();
    let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
        .expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    assert_eq!(head.insertions().unwrap(), 1)
}

#[test]
fn test_deletions() {
    let repo_path = make_repo();
    let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
        .expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    assert_eq!(head.deletions().unwrap(), 0)
}

#[test]
fn test_lines() {
    let repo_path = make_repo();
    let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
        .expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    assert_eq!(head.lines().unwrap(), 1)
}

#[test]
fn test_files_changed() {
    let repo_path = make_repo();
    let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
        .expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    assert_eq!(head.files().unwrap(), 1)
}
