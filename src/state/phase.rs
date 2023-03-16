use super::{Name, Team};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// フェーズ
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum Phase {
    /// メンバー募集中
    Waiting,
    /// 夜
    Night {
        /// 何回目の夜であるか
        count: usize,
        waiting: HashSet<Name>,
    },
    /// 昼
    Day {
        /// 何回目の昼であるか
        count: usize,
        /// 投票
        votes: HashMap<Name, Name>,
        /// 追放の候補者
        candidates: HashSet<Name>,
    },
    /// 終了
    End(Team),
}
