#[cfg_attr(test, mockall::automock)]
pub trait GitActor {
    /// The name of the actor
    fn name(&self) -> Option<String>;
    /// The email of the actor
    fn email(&self) -> Option<String>;
    /// The number of seconds since epoch when the actors action occured
    fn seconds(&self) -> i64;
}

#[cfg(test)]
mod test {
    use super::*;

    fn actor_factory() -> MockGitActor {
        MockGitActor::new()
    }

    #[test]
    fn test_name() {
        let mut actor = actor_factory();
        actor.expect_name().return_once(|| Some("test".to_string()));
        assert_eq!(actor.name(), Some("test".to_string()));
    }

    #[test]
    fn test_email() {
        let mut actor = actor_factory();
        actor
            .expect_email()
            .return_once(|| Some("test".to_string()));
        assert_eq!(actor.email(), Some("test".to_string()));
    }

    #[test]
    fn test_seconds() {
        let mut actor = actor_factory();
        actor.expect_seconds().return_once(|| 1_600_000_000);
        assert_eq!(actor.seconds(), 1_600_000_000);
    }
}
