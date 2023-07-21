use std::{cell::Cell, collections::HashSet};

use crate::role::{Error as RoleError, Role};

use super::{Name, Permission, State};

use bimap::BiHashMap;
use rand::{random, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use thiserror::Error;

/// マスター関連のエラー
#[derive(Error, Debug)]
pub enum Error {
    /// 登録時のユーザー名が被る場合
    #[error("display name of `{0}` is already in use.")]
    NameAlreadyRegistered(String),
    /// 認証に失敗した場合
    #[error("authentication failed.")]
    AuthenticationFailed,
    /// ゲームが既に始まっていた場合
    #[error("game has already started.")]
    GameAlreadyStarted,
    /// 役割処理の際のエラー
    #[error("RoleError: {0}")]
    Role(#[from] RoleError),
    /// 設定のエラー
    #[error("ConfigError: {0}")]
    Config(#[from] ConfigError),
}

/// 設定関連のエラー
#[derive(Error, Debug)]
pub enum ConfigError {
    /// role_countsに記載された人数とメンバー数が一致しません。
    #[error("The number of members does not match the number of people listed in role_counts.")]
    InvalidRoleCounts(Config),
}

/// トークン
pub type Token = [u8; 32];

/// ゲームマスター
pub struct Master {
    /// トークンから表示名への辞書
    tokens: BiHashMap<Token, Name>,
    /// ゲーム設定。ゲームのルールが主。
    config: Config,
    /// 状態。場面とそれに依存するデータ。
    state: Cell<State>,
}

/// 市民の設定
#[derive(Default, Debug, Serialize, Clone, Deserialize)]
pub struct CitizenConfig {
    /// 人数
    pub count: usize,
}

/// 狩人の設定
#[derive(Default, Debug, Serialize, Clone, Deserialize)]
pub struct HunterConfig {
    /// 人数
    pub count: usize,
    /// スキップできるかどうか
    pub skippable: bool,
    /// 連続して同じ人を守れるかどうか
    pub consecutive_guard: bool,
}

/// 人狼の設定
#[derive(Default, Debug, Serialize, Clone, Deserialize)]
pub struct WolfConfig {
    /// 人数
    pub count: usize,
    /// スキップできるかどうか
    pub skippable: bool,
}

/// 占い師の設定
#[derive(Default, Debug, Serialize, Clone, Deserialize)]
pub struct SeerConfig {
    /// 人数
    pub count: usize,
    /// スキップできるかどうか
    pub skippable: bool,
}

/// ゲーム設定
#[derive(Default, Debug, Serialize, Clone, Deserialize)]
pub struct Config {
    pub citizen: CitizenConfig,
    pub hunter: HunterConfig,
    pub seer: SeerConfig,
    pub wolf: WolfConfig,
}

impl Default for Master {
    fn default() -> Self {
        Self::new()
    }
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

impl From<Config> for Master {
    fn from(config: Config) -> Self {
        Master {
            config,
            ..Default::default()
        }
    }
}

impl Master {
    /// ユーザー待機状態のゲームマスターのインスタンスを返す。
    pub fn new() -> Self {
        Master {
            state: Cell::new(State::default()),
            tokens: BiHashMap::new(),
            config: Config::default(),
        }
    }
    /// ユーザーを登録する
    /// # Example
    /// ```
    /// use werewolf::master::{Master, Error::NameAlreadyRegistered};
    /// let mut master = Master::new();
    /// assert!(matches!(master.signup("たろう".to_string()), Ok(_token)));
    /// assert!(matches!(master.signup("はなこ".to_string()), Ok(_token)));
    /// assert!(matches!(master.signup("たろう".to_string()), Err(NameAlreadyRegistered(_))));
    /// ```
    pub fn signup(&mut self, name: Name) -> Result<Token, Error> {
        if let State::Waiting(_) = self.state.get_mut() {
            if self.tokens.contains_right(&name) {
                return Err(Error::NameAlreadyRegistered(name));
            }
            let token: Token = random();
            self.tokens.insert(token, name);
            Ok(token)
        } else {
            Err(Error::GameAlreadyStarted)
        }
    }
    /// トークンからパーミッションを得る
    /// # Example
    /// ```
    /// use werewolf::master::{Master, Error::AuthenticationFailed};
    /// let mut master = Master::new();
    /// let token = master.signup("たろう".to_string()).unwrap();
    /// assert!(matches!(master.login(&token), Ok(_permission)));
    /// assert!(matches!(master.login(&Default::default()), Err(AuthenticationFailed)));
    /// ```
    pub fn login(&mut self, token: &Token) -> Result<Permission, Error> {
        let Self {
            state,
            ref tokens,
            config,
        } = self;
        let Some(name) = tokens.get_by_left(token) else {
            return  Err(Error::AuthenticationFailed);
        };
        Ok(Permission {
            name,
            state,
            config,
        })
    }

    /// 開始していないゲームをスタートする。
    /// # Example
    /// ```
    /// use werewolf::master::{Master, Error::GameAlreadyStarted};
    /// let mut master = Master::new();
    /// assert!(matches!(master.start(), Ok(())));
    /// assert!(matches!(master.start(), Err(GameAlreadyStarted)));
    /// ```
    pub fn start(&mut self) -> Result<(), Error> {
        if let State::Waiting(_) = self.state.get_mut() {
            let survivors = HashSet::from_iter(self.tokens.right_values().map(|a| a.to_owned()));
            let role = {
                let mut all_roles = Role::iter()
                    .flat_map(|role| {
                        let count: usize = match &role {
                            Role::Citizen => self.config.citizen.count,
                            Role::Hunter { .. } => self.config.hunter.count,
                            Role::Wolf { .. } => self.config.wolf.count,
                            Role::Seer { .. } => self.config.seer.count,
                        };
                        std::iter::repeat(role).take(count)
                    })
                    .collect::<Vec<Role>>();
                if all_roles.len() != survivors.len() {
                    return Err(ConfigError::InvalidRoleCounts(self.config.clone()).into());
                }
                all_roles.shuffle(&mut rand::thread_rng());
                survivors.clone().into_iter().zip(all_roles).collect()
            };

            // stateの初期化。
            self.state = Cell::new(State::Night {
                count: 0,
                role,
                waiting: survivors.clone(),
                survivors,
            });
            Ok(())
        } else {
            Err(Error::GameAlreadyStarted)
        }
    }
}
