use super::{Name, Permission, State};

use bimap::BiHashMap;
use rand::distributions::{Alphanumeric, DistString};
use std::collections::HashMap;
use thiserror::Error;

/// 認証周辺のエラー
#[derive(Error, Debug)]
pub enum Error {
    #[error("display name of `{0}` is already in use.")]
    NameAlreadyRegistered(String),
    #[error("authentication failed.")]
    AuthenticationFailed,
}

/// トークン
pub type Token = String;
/// Token文字列の長さ
pub const TOKEN_LENGTH: usize = 32;

/// ゲームマスター
pub struct Master<'master> {
    /// 状態
    state: State,
    /// トークンから表示名への辞書
    tokens: BiHashMap<Token, Name>,
    /// 各ユーザーが閲覧している状態(マスク&変換済みのもの)
    client_states: HashMap<&'master Name, State>,
}

impl<'master> Default for Master<'master> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'master> Master<'master> {
    /// ユーザー待機状態のゲームマスターのインスタンスを返す。
    pub fn new() -> Self {
        Master {
            state: State::new(),
            tokens: BiHashMap::new(),
            client_states: HashMap::new(),
        }
    }
    /// ユーザーを登録する
    pub fn signup(&mut self, name: Name) -> Result<Token, Error> {
        if self.tokens.contains_right(&name) {
            return Err(Error::NameAlreadyRegistered(name));
        }
        let token: Token = Alphanumeric.sample_string(&mut rand::thread_rng(), TOKEN_LENGTH);
        self.tokens.insert(token.clone(), name);
        Ok(token)
    }
    /// トークンからパーミッションを得る
    pub fn login(&'master mut self, token: &Token) -> Result<Permission<'master>, Error> {
        let Self {
            ref mut state,
            ref tokens,
            client_states,
        } = self;
        let Some(name) = tokens.get_by_left(token) else {
            return  Err(Error::AuthenticationFailed);
        };
        Ok(Permission {
            state,
            name,
            client_states,
        })
    }
}
