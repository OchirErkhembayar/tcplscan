use git2::{Error, Repository};

fn _get_repository(_path: &str) -> Result<Repository, Error> {
    Repository::open(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_opens_repo() {
        let repo = Repository::open(".").unwrap();

        assert!(repo.state() == git2::RepositoryState::Clean);
    }

    #[test]
    fn test_remotes() {
        let repo = Repository::open(".").unwrap();

        assert_eq!("origin", repo.remotes().unwrap().get(0).unwrap());
    }
}
