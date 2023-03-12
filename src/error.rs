use thiserror::Error;

/// エラー一覧
#[derive(Error, Debug)]
pub enum Error {
    #[error("display name of `{0}` is already in use.")]
    NameAlreadyRegistered(String),
    #[error("unauthorized.")]
    Unauthorized,
}
