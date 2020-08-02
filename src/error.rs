pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Git(git2::Error),
    IO(std::io::Error),
    Crawler(jwalk::Error),
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        Error::Git(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}
impl From<jwalk::Error> for Error {
    fn from(e: jwalk::Error) -> Self {
        Error::Crawler(e)
    }
}
