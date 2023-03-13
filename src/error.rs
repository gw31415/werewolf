use thiserror::Error;

use crate::Phase;

/// エラー一覧
#[derive(Error, Debug)]
pub enum Error {
    #[error("display name of `{0}` is already in use.")]
    NameAlreadyRegistered(String),
    #[error("unauthorized.")]
    Unauthorized,
    #[error("invalid Phase (found {found:?}, expected pattern {expected:?})")]
    InvalidPhase {
        found: Phase,
        expected: String,
    },
    #[error("you cannot vote to the player named: {0}.")]
    CannotVoteToThisPlayer(String),
    #[error("this request is allowed only survivors.")]
    SurvivorsOnly,
}
