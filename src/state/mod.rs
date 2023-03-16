mod request;

pub use request::Request;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::{Role, Team};

/// IDとして使用する表示名
pub type Name = String;

/// ゲームの状態
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct State {
    /// 現在のフェーズ
    pub phase: Phase,
    /// メンバー一覧
    pub members: HashSet<Name>,
    /// 役職のマップ
    pub role: HashMap<Name, Role>,
    /// 生存者
    pub survivors: HashSet<Name>,
}

impl State {
    /// stateを各ユーザーの権限に基づいてマスク・変換したものを作成する。
    pub(crate) fn create_masked_for(&self, name: &str) -> Self {
        let mut output = self.clone();
        // 他プレイヤーの情報を外す
        for another_member in self.members.iter() {
            if another_member == name {
                continue;
            }
            output.role.remove(another_member);
        }
        output
    }
}

// 初期化
impl Default for State {
    fn default() -> Self {
        State {
            phase: Phase::Waiting,
            members: HashSet::new(),
            survivors: HashSet::new(),
            role: HashMap::new(),
        }
    }
}

/// フェーズ
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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
