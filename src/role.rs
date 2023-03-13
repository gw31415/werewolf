use serde::{Deserialize, Serialize};

/// 役職
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Role {
    /// 市民
    Citizen,
}

impl Role {
    /// チームを返す。
    pub fn team(&self) -> Team {
        match self {
            Self::Citizen => Team::Citizen,
        }
    }
}

/// 陣営
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Team {
    /// 市民
    Citizen,
    /// 人狼
    Wolf,
}
