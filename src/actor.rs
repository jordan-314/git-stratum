use chrono::{DateTime, Utc};
use git2::Signature;

/// A git actor who exists for the inspected repository
pub struct Actor<'a> {
    inner: Signature<'a>,
}

impl<'a> Actor<'a> {
    /// Instantiate a new Actor from their signature
    pub fn new(signature: Signature<'a>) -> Self {
        Self { inner: signature }
    }

    /// Return the actors name if it exists
    pub fn name(&self) -> Option<String> {
        self.inner.name().map(|s| s.to_string())
    }

    /// Return the actors email if it exists
    pub fn email(&self) -> Option<String> {
        self.inner.email().map(|s| s.to_string())
    }

    /// Return the timestamp of actor action if it exists
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp_secs(self.inner.when().seconds())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor() {
        let sig = Signature::new(
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
