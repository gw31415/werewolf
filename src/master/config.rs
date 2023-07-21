use super::*;

use thiserror::Error;

/// 設定関連のエラー
#[derive(Error, Debug)]
pub enum Error {
    /// role_countsに記載された人数とメンバー数が一致しません。
    #[error("The number of members does not match the number of people listed in role_counts.")]
    InvalidRoleCounts(Config),
}

/// ゲーム設定
#[derive(Default, Debug, Serialize, Clone, Deserialize)]
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
