pub mod request;

use crate::role::{Role, Team};

use std::collections::{HashMap, HashSet};

pub type Name = String;

/// フェーズ
#[derive(Default, Debug, Clone)]
pub enum State {
    /// メンバー募集中
    #[default]
    Waiting,
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

        /// 次の昼の生存者
        next_survivors: HashSet<Name>,
    },
    /// 夜
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
    End(Team),
}
