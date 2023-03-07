mod werewolf {
    use serde::{Deserialize, Serialize};
    use std::ops::AddAssign;

    /// 送受信されるメッセージ
    pub trait Message: Serialize + for<'a> Deserialize<'a> {
        /// メッセージの種類を識別するID
        const TYPEID: u64;
        /// Stateを更新する。
        fn update(&self, state: &mut State);
    }

    /// ゲームの状態
    #[derive(Serialize, Deserialize)]
    pub struct State;

    impl<T> AddAssign<T> for State
    where
        T: Message,
    {
        fn add_assign(&mut self, rhs: T) {
            rhs.update(self);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
