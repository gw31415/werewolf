use crate::{
    master::Config,
    role::{Role, Team},
};

use serde::Serialize;
use std::collections::{HashMap, HashSet};

pub type Name = String;

/// フェーズ
// unsafeでCell内のクローンを行うためメンバに注意。
// ArcやRcなど禁止。
#[derive(Serialize, Clone, Debug)]
pub enum State {
    /// メンバー募集中
    Waiting {
        /// 設定
        config: Config,
    },
    /// 夜
    Night {
        /// 何周目であるか
        count: usize,
        /// 役職
        role: HashMap<Name, Role>,
        /// 待機中の人
        waiting: HashSet<Name>,
        /// 生存している人
        survivors: HashSet<Name>,
    },
    /// 昼
    Day {
        /// 何周目であるか
        count: usize,
        /// 役職
        role: HashMap<Name, Role>,
        /// 待機中の人
        waiting: HashSet<Name>,
        /// 生存している人
        survivors: HashSet<Name>,

        /// 投票
        votes: HashMap<Name, Name>,
        /// 追放の候補者
        candidates: HashSet<Name>,
    },
    /// 終了
    End {
        /// 勝利したチーム
        winner: Team,
    },
}

impl Default for State {
    fn default() -> Self {
        Self::Waiting {
            config: Config::default(),
        }
    }
}
