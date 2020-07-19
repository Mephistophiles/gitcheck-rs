pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    GitError(git2::Error),
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        Error::GitError(e)
    }
}
