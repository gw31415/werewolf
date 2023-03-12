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
    count: u64,
    /// 現在のフェーズ
    phase: Phase,
    /// メンバー一覧
    members: HashSet<Name>,
    /// 役職のマップ
    role: HashMap<Name, Role>,
}

impl State {
    /// stateを各ユーザーの権限に基づいてマスク・変換する
    pub fn mask_for(&self, name: &str) -> Self {
        todo!()
    }
}

// 初期化
impl Default for State {
    fn default() -> Self {
        State {
            count: 0,
            phase: Phase::Waiting,
            members: HashSet::new(),
            role: HashMap::new(),
        }
    }
}

/// フェーズ
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Phase {
    /// メンバー募集中
    Waiting,
    /// 夜
    Night,
    /// 昼
    Day,
    /// 終了
    End(Team),
}
