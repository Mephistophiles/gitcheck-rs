pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    GitError(git2::Error),
    IOError(std::io::Error),
    Crawler(jwalk::Error),
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        Error::GitError(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}
impl From<jwalk::Error> for Error {
    fn from(e: jwalk::Error) -> Self {
        Error::Crawler(e)
    }
}
