pub mod werewolf {
    use bimap::BiHashMap;
    use rand::distributions::{Alphanumeric, DistString};
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet};
    use thiserror::Error;

    /// エラー一覧
    #[derive(Error, Debug)]
    pub enum Error {
        #[error("display name of `{0}` is already in use.")]
        NameAlreadyRegistered(String),
        #[error("unauthorized.")]
        Unauthorized,
    }

    /// 送受信されるリクエスト
    #[derive(Serialize, Deserialize)]
    pub enum Request {}
    impl Request {
        /// Stateを更新する。
        fn apply_to(&self, state: &mut State, sender: &Name) -> Result<(), Error> {
            todo!()
        }
    }

    /// 役職
    #[derive(Serialize, Deserialize, PartialEq, Clone)]
    pub enum Role {}

    /// トークン
    pub type Token = String;
    /// Token文字列の長さ
    pub const TOKEN_LENGTH: usize = 32;

    /// IDとして使用する表示名
    pub type Name = String;

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

    impl Master {
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
        pub fn apply(&mut self, token: Token, req: Request) -> Result<HashSet<Name>, Error> {
            let Some(name) = self.tokens.get_by_left(&token) else { return Err(Error::Unauthorized) };
            req.apply_to(&mut self.state, name)?;
            let mut updated_list = HashSet::new();
            for name in self.tokens.right_values() {
                let next_state = self.state.mask_for(name);
                // ユーザー毎の状態を更新し、実際に更新されたユーザー名のリストを作成する。
                if self
                    .client_states
                    .insert(name.to_owned(), next_state.clone())
                    != Some(next_state)
                {
                    updated_list.insert(name.clone());
                }
            }
            Ok(updated_list)
        }
    }

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
        fn mask_for(&self, name: &str) -> Self {
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

    /// 陣営
    #[derive(Serialize, Deserialize, PartialEq, Clone)]
    pub enum Team {
        /// 市民
        Citizen,
        /// 人狼
        Wolf,
    }
}

fn main() {
    println!("Hello, world!");
}
