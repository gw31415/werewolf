use super::{Error, Name, State};
use serde::{Deserialize, Serialize};

/// 送受信されるリクエスト
#[derive(Serialize, Deserialize)]
pub enum Request {}
impl Request {
    /// Stateを更新する。
    pub fn apply_to(&self, state: &mut State, sender: &Name) -> Result<(), Error> {
        todo!()
    }
}
