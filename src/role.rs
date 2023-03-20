use std::collections::HashMap;

use serde::Serialize;

use crate::state::Name;

/// 役職
#[derive(Serialize, PartialEq, Eq, Clone, Debug)]
pub enum Role {
    /// 市民
    Citizen,
    /// 人狼
    Wolf,
    /// 占い師
    /// HashMapの値は黒(人狼サイド)のときにtrue
    Seer(HashMap<Name, bool>),
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
