use std::collections::HashMap;

use serde::Serialize;
use strum::EnumIter;
use thiserror::Error;

use crate::state::Name;

/// 認証周辺のエラー
#[derive(Error, Debug)]
pub enum Error {
    /// 指定された名前のロールが存在していない場合。
    #[error("Cannot find role named {0}.")]
    UnknownRole(String),
}

/// 役職
#[derive(Serialize, PartialEq, Eq, Clone, Debug, EnumIter)]
pub enum Role {
    /// 市民
    #[strum(serialize = "citizen")]
    Citizen,
    /// 人狼
    #[strum(serialize = "wolf")]
    Wolf { killing: Option<Name> },
    /// 占い師
    /// HashMapの値は黒(人狼サイド)のときにtrue
    #[strum(serialize = "seer")]
    Seer { prediction: HashMap<Name, Team> },
    /// 狩人
    #[strum(serialize = "hunter")]
    Hunter { saving: Option<Name> },
}

impl Role {
    /// チームを返す。
    pub fn team(&self) -> Team {
        match self {
            Self::Citizen | Self::Seer { .. } | Self::Hunter { .. } => Team::Citizen,
            Self::Wolf { .. } => Team::Wolf,
        }
    }
}

/// 陣営
#[derive(Serialize, PartialEq, Eq, Clone, Debug)]
pub enum Team {
    /// 市民陣営
    Citizen,
    /// 人狼陣営
    Wolf,
}
