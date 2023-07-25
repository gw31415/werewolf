use crate::role::Role;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 設定関連のエラー
#[derive(Error, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Error {
    /// role_countsに記載された人数とメンバー数が一致しません。
    #[error("The number of members does not match the number of people listed in role_counts.")]
    InvalidRoleCounts(Config),
}

/// ゲーム設定
#[derive(Default, Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub citizen: CitizenConfig,
    pub hunter: HunterConfig,
    pub seer: SeerConfig,
    pub wolf: WolfConfig,
}

impl Config {
    // 与えられたロールがスキップ可能かどうか。
    pub fn skippable(&self, role: &Role) -> bool {
        use Role::*;
        match role {
            Citizen => true,
            Wolf { .. } => self.wolf.skippable,
            Seer { .. } => self.seer.skippable,
            Hunter { .. } => self.hunter.skippable,
        }
    }
}

/// 市民の設定
#[derive(Default, Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct CitizenConfig {
    /// 人数
    pub count: usize,
}

/// 狩人の設定
#[derive(Default, Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct HunterConfig {
    /// 人数
    pub count: usize,
    /// スキップできるかどうか
    pub skippable: bool,
    /// 連続して同じ人を守れるかどうか
    pub consecutive_guard: bool,
}

/// 人狼の設定
#[derive(Default, Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct WolfConfig {
    /// 人数
    pub count: usize,
    /// スキップできるかどうか
    pub skippable: bool,
}

/// 占い師の設定
#[derive(Default, Debug, Serialize, Clone, Deserialize, PartialEq, Eq)]
pub struct SeerConfig {
    /// 人数
    pub count: usize,
    /// スキップできるかどうか
    pub skippable: bool,
}
