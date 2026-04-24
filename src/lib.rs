// For now include everything as pub mod to get errors in IDE.
use git_url_parse::GitUrlParseError;
use thiserror::Error;

mod domain;
mod repository;
mod url;

pub use domain::{actor::Actor, commit::Commit};
pub use repository::{Local, Remote, Repository};
pub use url::GitUrl;

#[derive(Debug, Error)]
pub enum Error {
    /// An abstraction of git2::Error to raise the error effectively
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An abstraction of git-url-parse::GitUrlParseError
    #[error(transparent)]
    GitUrlError(#[from] GitUrlParseError),

    /// If a URL can be parsed but is not a valid GitUrl schem
    #[error("URL scheme was {0}, cannot clone URL.")]
    UrlScheme(String),

    /// An error associated with a bad path
    #[error("{0}")]
    PathError(String),
}
