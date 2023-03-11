pub mod werewolf {
    use bimap::BiHashMap;
    use rand::distributions::{Alphanumeric, DistString};
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;
    use thiserror::Error;

    /// エラー一覧
    #[derive(Error, Debug)]
    pub enum Error {
        #[error("this username is already in use.")]
        UserAlreadyRegistered,
        #[error("unauthorized.")]
        Unauthorized,
    }

    /// 送受信されるリクエスト
    pub trait Request: Serialize + for<'a> Deserialize<'a> {
        /// Stateを更新する。
        fn update(&self, state: &mut State);
    }

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
    }

    impl Master {
        /// ユーザーを登録する
        pub fn register(&mut self, name: Name) -> Result<Token, Error> {
            if self.tokens.contains_right(&name) {
                return Err(Error::UserAlreadyRegistered);
            }
            let token: Token = Alphanumeric.sample_string(&mut rand::thread_rng(), TOKEN_LENGTH);
            self.tokens.insert(token.clone(), name);
            Ok(token)
        }
    }

    /// ゲームの状態
    #[derive(Serialize, Deserialize)]
    pub struct State {
        /// 何周目かを表す整数値。
        /// ゲーム開始前は0で、夜がくる度に+1される。
        count: u64,
        /// 現在のフェーズ
        phase: Phase,
        /// メンバー一覧
        members: HashSet<Name>,
    }

    // 初期化
    impl Default for State {
        fn default() -> Self {
            State {
                count: 0,
                phase: Phase::Waiting,
                members: HashSet::new(),
            }
        }
    }

    /// フェーズ
    #[derive(Serialize, Deserialize)]
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
    #[derive(Serialize, Deserialize)]
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
