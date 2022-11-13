#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ParseError,
    RepositoryError,
    ApplicationError(String),
}
