// Integration tests associated with repository.rs

mod common;
use std::path::PathBuf;

use common::test_data_dir;

use stratum::{Local, Repository};

fn small_repo() -> PathBuf {
    test_data_dir().join("small_repo")
}

#[test]
fn init_repo_from_path() {
    // Same content as 'make_repo' function, this will flag if that causes
    // the other tests to fail
    let p = small_repo();
    Repository::<Local>::new(p).expect("Failed to open as repo");
}

// #[test]
// fn test_commit_traversal() {
//     let repo_path = make_repo();
//     let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
//         .expect("Failed to open as repo");

//     // Expecting an iter of length one
//     let iter = repo.traverse_commits().expect("Iterator Error");
//     for commit in iter {
//         let commit = commit.expect("Expected a valid stratum commit");
//         run_test_for_repo_head(&commit);
//     }
// }

// #[test]
// fn test_head() {
//     let repo_path = make_repo();
//     let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
//         .expect("Failed to open as repo");

//     let head = repo.head().expect("Failed to create HEAD commit");
//     run_test_for_repo_head(&head);
// }

// #[test]
// fn test_single() {
//     let repo_path = make_repo();
//     let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
//         .expect("Failed to open as repo");

//     let head_hash = repo
//         .raw()
//         .head()
//         .unwrap()
//         .peel_to_commit()
//         .unwrap()
//         .id()
//         .to_string();
//     let commit = repo.single(&head_hash).expect("Expected valid hash string");

//     run_test_for_repo_head(&commit);
// }

// #[test]
// fn test_insertions() {
//     let repo_path = make_repo();
//     let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
//         .expect("Failed to open as repo");

//     let head = repo.head().expect("Failed to create HEAD commit");
//     assert_eq!(head.insertions().unwrap(), 1)
// }

// #[test]
// fn test_deletions() {
//     let repo_path = make_repo();
//     let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
//         .expect("Failed to open as repo");

//     let head = repo.head().expect("Failed to create HEAD commit");
//     assert_eq!(head.deletions().unwrap(), 0)
// }

// #[test]
// fn test_lines() {
//     let repo_path = make_repo();
//     let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
//         .expect("Failed to open as repo");

//     let head = repo.head().expect("Failed to create HEAD commit");
//     assert_eq!(head.lines().unwrap(), 1)
// }

// #[test]
// fn test_files_changed() {
//     let repo_path = make_repo();
//     let repo = Repository::<Local>::new(repo_path.path().to_str().unwrap())
//         .expect("Failed to open as repo");

//     let head = repo.head().expect("Failed to create HEAD commit");
//     assert_eq!(head.files().unwrap(), 1)
// }
