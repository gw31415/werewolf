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
    pub fn create_masked_for(&self, name: &str) -> Self {
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
    /// 勝敗を確認する。終了した場合はPhaseをEndにし、trueを返す。
    /// 終了しなかった場合はfalseを返す。
    pub(crate) fn judge(&mut self) -> bool {
        let State {
            ref survivors,
            ref role,
            ref mut phase,
            ..
        } = self;

        // 陣営の数を数える
        let mut iter = survivors.iter();
        let (mut wolf_count, mut citizen_count) = (0usize, 0usize);
        let mut check_wolf_win_after_increment = |survivor: &Name| {
            let role = role.get(survivor).unwrap();
            match role.team() {
                Team::Wolf => {
                    wolf_count += 1;
                }
                Team::Citizen => {
                    citizen_count += 1;
                }
            }
            if wolf_count >= citizen_count {
                *phase = Phase::End(Team::Wolf);
                return true;
            }
            false
        };

        'wolf_presence_check: {
            // 市民の勝利条件確認(人狼の存在の有無)
            for survivor in &mut iter {
                if check_wolf_win_after_increment(survivor) {
                    return true;
                }
                if let Role::Wolf(_) = self.role.get(survivor).unwrap() {
                    break 'wolf_presence_check;
                }
            }
            // 人狼が存在しなければ市民陣営の勝利
            *phase = Phase::End(Team::Citizen);
            return true;
        }
        for survivor in &mut iter {
            if check_wolf_win_after_increment(survivor) {
                return true;
            }
        }
        // この時点で、人狼の勝利条件確認の終了 (人狼>市民)
        false
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
