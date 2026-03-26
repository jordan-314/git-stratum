// For now include everything as pub mod to get errors in IDE.
use git_url_parse::GitUrlParseError;
use thiserror::Error;

pub mod actor;
pub mod commit;
pub mod remote;
pub mod repository;

pub use actor::MinedActor;
pub use commit::MinedCommit;
pub use repository::Repository;

#[derive(Debug, Error)]
pub enum StratumError {
    /// An abstraction of git2::Error to raise the error effectively
    #[error("{0}")]
    GitError(git2::Error),

    /// An abstraction of git-url-parse::GitUrlParseError
    #[error("{0}")]
    GitUrlError(GitUrlParseError),

    /// If a URL can be parsed but is not a valid GitUrl schem
    #[error("Invalid scheme: {0}")]
    GitUrlSchemeError(String),

    /// An error associated with a bad path
    #[error("{0}")]
    PathError(String),
}
