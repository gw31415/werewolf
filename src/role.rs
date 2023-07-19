use std::collections::HashMap;

use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator, AsRefStr};
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
#[derive(Serialize, PartialEq, Eq, Clone, Debug, EnumIter, AsRefStr)]
pub enum Role {
    /// 市民
    Citizen,
    /// 人狼
    Wolf,
    /// 占い師
    /// HashMapの値は黒(人狼サイド)のときにtrue
    Seer(HashMap<Name, bool>),
}

impl TryFrom<&str> for Role {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for role in Role::iter() {
            if role.as_ref() == value {
                return Ok(role)
            }
        }
        Err(Error::UnknownRole(value.into()))
    }
}

impl Role {
    /// チームを返す。
    pub fn team(&self) -> Team {
        match self {
            Self::Citizen | Self::Seer(_) => Team::Citizen,
            Self::Wolf => Team::Wolf,
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
