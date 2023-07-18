use std::cell::Cell;

use super::{Name, Permission, State};

use bimap::BiHashMap;
use rand::random;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 認証周辺のエラー
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
}

/// トークン
pub type Token = [u8; 32];

/// ゲームマスター
pub struct Master {
    /// トークンから表示名への辞書
    tokens: BiHashMap<Token, Name>,
    /// ゲーム設定。ゲーム開始から変更されないデータ全般
    config: Config,
    /// 状態。ゲームの進行と共に変化していくデータ全般
    state: Cell<State>,
}

/// ゲーム設定
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Config {}

impl Default for Master {
    fn default() -> Self {
        Self::new()
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
        Ok(Permission { name, state, config })
    }
}
