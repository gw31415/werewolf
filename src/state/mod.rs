mod phase;
pub mod request;

use super::role::{Role, Team};
pub use phase::Phase;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// IDとして使用する表示名
pub type Name = String;

/// ゲームの状態
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
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
    // 初期化
    pub(crate) fn new() -> Self {
        State {
            phase: Phase::Waiting,
            members: HashSet::new(),
            survivors: HashSet::new(),
            role: HashMap::new(),
        }
    }
}
