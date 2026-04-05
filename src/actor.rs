use chrono::{DateTime, Utc};

use crate::interface::GitActor;

impl GitActor for git2::Signature<'_> {
    fn name(&self) -> Option<String> {
        self.name().map(String::from)
    }

    fn email(&self) -> Option<String> {
        self.email().map(String::from)
    }

    fn seconds(&self) -> i64 {
        self.when().seconds()
    }
}

/// A git actor who exists for the inspected repository
pub struct Actor<S: GitActor> {
    inner: S,
}

impl<S: GitActor> Actor<S> {
    /// Instantiate a new Actor from their signature
    pub fn new(signature: S) -> Self {
        Self { inner: signature }
    }

    /// Return the actors name if it exists
    pub fn name(&self) -> Option<String> {
        self.inner.name()
    }

    /// Return the actors email if it exists
    pub fn email(&self) -> Option<String> {
        self.inner.email()
    }

    /// Return the timestamp of actor action if it exists
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp_secs(self.inner.seconds())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor() {
        let sig = git2::Signature::new(
            "test",
            "test@example.com",
            &git2::Time::new(1_600_000_000, 0),
        )
        .unwrap();

        let actor = Actor::new(sig);

        assert_eq!(actor.name(), Some("test".to_string()));
        assert_eq!(actor.email(), Some("test@example.com".to_string()));
        assert_eq!(actor.timestamp().unwrap().timestamp(), 1_600_000_000);
    }
}
