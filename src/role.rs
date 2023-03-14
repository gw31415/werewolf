use serde::{Deserialize, Serialize};

/// 役職
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Role {
    /// 市民
    Citizen,
    /// 人狼
    Wolf,
}

impl Role {
    /// チームを返す。
    pub fn team(&self) -> Team {
        match self {
            Self::Citizen => Team::Citizen,
            Self::Wolf => Team::Wolf,
        }
    }
}

/// 陣営
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Team {
    /// 市民陣営
    Citizen,
    /// 人狼陣営
    Wolf,
}
