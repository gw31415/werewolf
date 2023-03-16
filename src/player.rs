use crate::error::Error;
use crate::request::Request;
use crate::state::State;
use crate::Name;
use std::collections::HashMap;

/// プレイヤー
pub struct Player<'master> {
    name: &'master Name,
    state: &'master mut State,
    client_states: &'master mut HashMap<&'master Name, State>,
}

impl<'master> Player<'master> {
    /// リクエストを処理する。
    pub fn process(self, request: Request) -> Result<Vec<(&'master Name, State)>, Error> {
        let Self {
            name,
            state,
            client_states,
        } = self;
        request.apply_to(state, name)?;
        // この更新結果を他ユーザーに通知をすべきなのか。
        let mut updated_list = Vec::new();
        for name in state.members.iter() {
            let next_state = state.create_masked_for(name);
            // ユーザー毎の状態を更新し、実際に更新されたユーザー名のリストを作成する。
            if Some(&next_state) != client_states.insert(name, next_state.clone()).as_ref() {
                updated_list.push((name, next_state));
            }
        }
        Ok(updated_list)
    }
}

mod master {
    use super::{Error, Name, Player, State};
    use bimap::BiHashMap;
    use rand::distributions::{Alphanumeric, DistString};
    use std::collections::HashMap;

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

    impl Default for Master<'_> {
        fn default() -> Self {
            Master::new()
        }
    }

    impl<'master> Master<'master> {
        /// ユーザー待機状態のゲームマスターのインスタンスを返す。
        pub fn new() -> Self {
            Master {
                state: State::default(),
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

        /// トークンからプレイヤーインスタンスを作成する
        pub fn login(&'master mut self, token: &Token) -> Result<Player<'master>, ()> {
            let Self {
                ref mut state,
                ref tokens,
                client_states,
            } = self;
            let Some(name) = tokens.get_by_left(token) else {
                return  Err(());
            };
            Ok(Player {
                state,
                name,
                client_states,
            })
        }
    }
}
pub use master::{Master, Token, TOKEN_LENGTH};
