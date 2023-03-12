use serde::{Deserialize, Serialize};
use super::{State, Name, Error};

/// 送受信されるリクエスト
#[derive(Serialize, Deserialize)]
pub enum Request {}
impl Request {
    /// Stateを更新する。
    pub fn apply_to(&self, state: &mut State, sender: &Name) -> Result<(), Error> {
        todo!()
    }
}
