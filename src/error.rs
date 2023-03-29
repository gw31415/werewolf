use thiserror::Error;

use crate::master::Error as AuthError;
use crate::request::Error as RequestError;

/// エラー一覧
#[derive(Error, Debug)]
pub enum Error {
    /// 認証時のエラー
    #[error("AuthError: {0}")]
    Auth(#[from] AuthError),
    /// リクエスト処理時のエラー
    #[error("RequestFailed: {0}")]
    RequestFailed(#[from] RequestError),
}
