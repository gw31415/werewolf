pub mod werewolf {
    use serde::{Deserialize, Serialize};
    use std::{fmt::Display, ops::AddAssign};

    /// 送受信されるリクエスト
    pub trait Request: Serialize + for<'a> Deserialize<'a> {
        /// Stateを更新する。
        fn update(&self, state: &mut State);
    }

    /// ゲームの状態
    #[derive(Serialize, Deserialize)]
    pub struct State {
        /// 何周目かを表す整数値。
        /// ゲーム開始前は0で、夜がくる度に+1される。
        count: u64,
        /// 現在のフェーズ
        phase: Phase,
    }

    // 初期化
    impl Default for State {
        fn default() -> Self {
            State {
                count: 0,
                phase: Phase::Waiting,
            }
        }
    }

    // 状態の更新
    impl<T: Request> AddAssign<T> for State {
        fn add_assign(&mut self, rhs: T) {
            rhs.update(self);
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
