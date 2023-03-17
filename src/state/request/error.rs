use super::Phase;
use crate::role::Role;

use thiserror::Error;

/// リクエスト処理時のエラー
#[derive(Error, Debug)]
pub enum Error {
    /// 場面が適切でない場合
    #[error("invalid Phase (found {found:?}, expected pattern {expected:?})")]
    InvalidPhase { found: Box<Phase>, expected: String },
    /// 役職が適切でない場合
    #[error("invalid Role (found {found:?}, expected pattern {expected:?})")]
    InvalidRole { found: Box<Role>, expected: String },
    /// ターゲットが適切でない場合
    #[error("you cannot request about `{0}`.")]
    InvalidTarget(String),
    /// リクエスト元が追放または殺害されている場合
    #[error("this request is allowed only survivors.")]
    SurvivorsOnly,
    /// 場面あたりのリクエスト回数を超過した場合
    #[error("cannot act more than once")]
    MultipleActions,
}
