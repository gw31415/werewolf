use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::{Role, Team};

/// IDとして使用する表示名
pub type Name = String;

/// ゲームの状態
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct State {
    /// 何周目かを表す整数値。
    /// ゲーム開始前は0で、夜がくる度に+1される。
    pub count: u64,
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
    /// stateを各ユーザーの権限に基づいてマスク・変換する
    pub fn mask_for(&self, name: &str) -> Self {
        todo!()
    }
    /// 勝敗を確認する。終了した場合はPhaseをEndにし、trueを返す。
    /// 終了しなかった場合はfalseを返す。
    pub fn judge(&mut self) -> bool {
        todo!();
    }
}

// 初期化
impl Default for State {
    fn default() -> Self {
        State {
            count: 0,
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
    Night,
    /// 昼
    Day {
        /// 投票
        votes: HashMap<Name, Name>,
        /// 追放の候補者
        candidates: HashSet<Name>,
    },
    /// 終了
    End(Team),
}
