mod error;
mod request;
mod role;
mod state;

pub use error::Error;
pub use request::Request;
pub use role::{Role, Team};
pub use state::{Name, Phase, State};

use bimap::BiHashMap;
use rand::distributions::{Alphanumeric, DistString};
use std::collections::HashMap;

/// トークン
pub type Token = String;
/// Token文字列の長さ
pub const TOKEN_LENGTH: usize = 32;

/// ゲームマスター
#[derive(Default)]
pub struct Master {
    /// 状態
    state: State,
    /// トークンから表示名への辞書
    tokens: BiHashMap<Token, Name>,
    /// 各ユーザーが閲覧している状態(マスク&変換済みのもの)
    client_states: HashMap<Name, State>,
}

impl<'master> Master {
    /// ユーザーを登録する
    pub fn register(&mut self, name: Name) -> Result<Token, Error> {
        if self.tokens.contains_right(&name) {
            return Err(Error::NameAlreadyRegistered(name));
        }
        let token: Token = Alphanumeric.sample_string(&mut rand::thread_rng(), TOKEN_LENGTH);
        self.tokens.insert(token.clone(), name);
        Ok(token)
    }
    /// リクエストを適用する。
    /// 更新があるユーザーのリストを返却する。
    pub fn apply(
        &'master mut self,
        token: Token,
        req: Request,
    ) -> Result<Vec<(&'master Name, State)>, Error> {
        let Some(name) = self.tokens.get_by_left(&token) else { return Err(Error::Unauthorized) };
        req.apply_to(&mut self.state, name)?;
        let mut updated_list = Vec::new(); // 更新があるユーザーの名称を保持する。
        for name in self.tokens.right_values() {
            let next_state = self.state.mask_for(name);
            // ユーザー毎の状態を更新し、実際に更新されたユーザー名のリストを作成する。
            if self
                .client_states
                .insert(name.to_owned(), next_state.clone())
                .as_ref()
                != Some(&next_state)
            {
                updated_list.push((name, next_state));
            }
        }
        Ok(updated_list)
    }
}
