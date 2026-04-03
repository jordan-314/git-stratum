// Define any and all iterators relating to the repository class.
use crate::Error;
use crate::commit::Commit;

//TODO: This should probably run on generics rather than git2::Commit
/// Define state to keep track of during commit iteration
pub struct CommitIterator<'iter> {
    repo: &'iter git2::Repository,
    walker: git2::Revwalk<'iter>,
}

impl<'iter> CommitIterator<'iter> {
    pub fn new(repo: &'iter git2::Repository) -> Result<Self, Error> {
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
