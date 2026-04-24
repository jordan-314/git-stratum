// Define any and all iterators relating to the repository class.
use crate::Commit;
use crate::Error;

/// Define state to keep track of during commit iteration
pub struct CommitIterator<'a> {
    repo: &'a git2::Repository,
    walker: git2::Revwalk<'a>,
}

impl<'a> CommitIterator<'a> {
    pub fn new(repo: &'a git2::Repository) -> Result<Self, Error> {
        let mut walker = repo.revwalk().map_err(Error::Git)?;
        //TODO: Allow pushing of any arbitrary commit
        walker.push_head().map_err(Error::Git)?;

        Ok(Self { repo, walker })
    }
}

impl<'a> Iterator for CommitIterator<'a> {
    type Item = Result<Commit<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.walker.next().map(|oid_result| {
            oid_result.map_err(Error::Git).and_then(|oid| {
                self.repo
                    .find_commit(oid)
                    .map(Commit::new)
                    .map_err(Error::Git)
            })
        })
    }
}
