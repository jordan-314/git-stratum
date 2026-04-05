use crate::actor::Actor;

/// A singular git commit for the repository being inspected
pub struct Commit<'a> {
    inner: git2::Commit<'a>,
}

impl<'a> Commit<'a> {
    /// Instantiate a new Commit object from a git2 commit
    pub fn new(inner: git2::Commit<'a>) -> Self {
        Self { inner }
    }

    /// Return the commit hash
    pub fn hash(&self) -> String {
        self.inner.id().to_string()
    }

    /// Return the commit message if it exists
    pub fn msg(&self) -> Option<String> {
        self.inner.message().map(|s| s.to_string())
    }

    //TODO: Inject dependancy into Commit to keep generic?
    /// Return the commit author
    pub fn author(&self) -> Actor<git2::Signature<'_>> {
        Actor::new(self.inner.author())
    }

    /// Return the commit committer
    pub fn committer(&self) -> Actor<git2::Signature<'_>> {
        Actor::new(self.inner.committer())
    }

    /// Retrun the hashes of all commit parents
    pub fn parents(&self) -> Vec<String> {
        self.inner.parent_ids().map(|id| id.to_string()).collect()
    }

    /// Return whether the commit is a merge commit
    pub fn is_merge(&self) -> bool {
        self.inner.parent_count() > 1
    }
}

//TODO: Test without using a repository to fetch a commit?
// This is harder than it sounds. May require a redesign of commits to not be
// tightly coupled with git2.
