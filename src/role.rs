use serde::{Deserialize, Serialize};

/// 役職
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Role {}

/// 陣営
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Team {
    /// 市民
    Citizen,
    /// 人狼
    Wolf,
}
