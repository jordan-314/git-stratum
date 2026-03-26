use git_url_parse::GitUrl;
use git2::Repository;

use crate::StratumError;

#[derive(Debug)]
pub enum UrlType {
    Https,
    Http,
    Ssh,
    File,
}

impl UrlType {
    pub fn is_remote(&self) -> bool {
        !matches!(self, Self::File)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct RepositoryPath(GitUrl, String);

impl RepositoryPath {
    #[allow(dead_code)]
    /// Parse the given string into a RepositoryPath object
    pub fn parse(s: &str) -> Result<Self, StratumError> {
        let url = GitUrl::parse(s).map_err(StratumError::GitUrlError)?;
        Ok(Self(url, s.to_string()))
    }

    /// Return the scheme of the given Url if it exists
    pub fn scheme(&self) -> Option<UrlType> {
        let url_type = match self.raw_scheme()? {
            "https" => UrlType::Https,
            "http" => UrlType::Http,
            "ssh" => UrlType::Ssh,
            "file" => UrlType::File,
            &_ => return None,
        };
        Some(url_type)
    }

    #[allow(dead_code)]
    /// Clone the Url if it is a remote url
    pub fn clone(&self, dest: &str) -> Result<(), StratumError> {
        // Check the scheme is valid even if though the Url was parsed correctly
        //
        // Perform this action because URLs which are NOT valid Git urls such as
        // bolt:// will be parsed.
        let url_type = self.scheme().ok_or(StratumError::GitUrlSchemeError(
            self.raw_scheme().unwrap_or("").to_string(),
        ))?;

        if url_type.is_remote() {
            Repository::clone(&self.raw_url(), dest).map_err(StratumError::GitError)?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    /// Return the raw scheme of the Url
    fn raw_scheme(&self) -> Option<&str> {
        self.0.scheme()
    }

    #[allow(dead_code)]
    /// Return the raw Url that was passed
    ///
    /// Clones the string value in slot 1 of the tuple struct
    fn raw_url(&self) -> String {
        self.1.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_remote() {
        use UrlType as T;

        let variant = T::File;
        assert!(!variant.is_remote());

        // Check all other variants are false.
        let remote_variants = [T::Http, T::Https, T::Ssh];
        for var in remote_variants {
            assert!(var.is_remote());
        }
    }

    #[test]
    fn test_parse() {
        assert!(RepositoryPath::parse("/tmp/file").is_ok());
        assert!(RepositoryPath::parse("https://web/server/file").is_ok());

        assert!(RepositoryPath::parse("vkhbiwdsa").is_err());
    }

    #[test]
    fn test_parse_with_bad_git_scheme() {
        let url = RepositoryPath::parse("bolt://tmp/path").unwrap();
        assert!(url.scheme().is_none());
    }

    #[test]
    fn test_scheme() {
        use UrlType as T;

        assert!(matches!(
            RepositoryPath::parse("https://tmp/file")
                .expect("Url parsing failed.")
                .scheme(),
            Some(T::Https)
        ));

        assert!(matches!(
            RepositoryPath::parse("/tmp")
                .expect("Path parsing failed")
                .scheme(),
            Some(T::File)
        ));
    }
}
